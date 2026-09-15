use std::{collections::VecDeque, thread::sleep, time::{Duration, Instant}};

use crate::{color::Rgb, draw_mode::DrawMode, effect::{Effect, SnowOptions}, event::{Event, Key}, fill::Fill, rgb_image::RgbImage, term_frame::TermFrame, termio::TermIO};

use clap::Parser;

pub mod ansi_codes;
pub mod borrowed_fd;
pub mod color;
pub mod draw_mode;
pub mod effect;
pub mod epoll;
pub mod fill;
pub mod event;
pub mod point3d;
pub mod rect;
pub mod rgb_image;
pub mod size2d;
pub mod style;
pub mod term_frame;
pub mod termio;
pub mod unicode;

//// debug input
//fn main() -> std::io::Result<()> {
//    let mut termio = TermIO::from_tty()?;
//
//    termio.move_cursor(0, 0)?;
//    termio.enable_mouse()?;
//    termio.flush()?;
//
//    loop {
//        let event = termio.wait()?;
//        println!("{event}");
//        match event {
//            Event::KeyPress { key: Key::Char('q'), alt: false, ctrl: false, shift: false } => {
//                break;
//            }
//            Event::ConnectionClosed => {
//                break;
//            }
//            _ => {}
//        }
//    }
//
//    Ok(())
//}


fn interruptable_sleep(duration: Duration) -> bool {
    #[cfg(target_family = "unix")]
    {
        let req = libc::timespec {
            tv_sec:  duration.as_secs() as libc::time_t,
            tv_nsec: duration.subsec_nanos() as i64,
        };
        let ret = unsafe { libc::nanosleep(&req, std::ptr::null_mut()) };
        return ret == 0;
    }

    #[cfg(not(target_family = "unix"))]
    {
        std::thread::sleep(duration);
        return true;
    }
}

#[derive(Parser)]
struct Args {
    #[clap(short, long, default_value = "60")]
    fps: u32,

    #[clap(long, default_value = "#432d84")]
    bg: Fill,

    #[clap(long, default_value = "#FFFFFF")]
    fg: Rgb,

    #[clap(long, default_value = "half_block")]
    draw_mode: DrawMode,

    #[clap(short = 'c', long, default_value = "128")]
    particle_count: u32,

    #[clap(long, default_value = "16.0")]
    speed: f32,

    #[clap(long, default_value = "30.0")]
    angle: f32,

    #[clap(long, default_value = "1.0")]
    depth: f32,

    #[clap(long, default_value = "0.25")]
    variation: f32,
}

#[derive(Debug)]
pub struct LogEntry {
    pub timestamp: Instant,
    pub message: String,
}

impl LogEntry {
    #[inline]
    pub fn new(message: String) -> Self {
        Self { timestamp: Instant::now(), message }
    }
}

fn main() -> std::io::Result<()> {
    let Args { fps, bg, fg, draw_mode, particle_count, speed, angle, depth, variation } = Args::parse();

    let log_timeout = Duration::from_secs(5);

    //if 1 == 1 {
    //    println!("{} {} {}", bg, bg.to_hsl(), bg.to_hsl().to_rgb());
    //    return Ok(());
    //}

    let mut effect /*: Box<dyn Effect> = Box::new*/ = (
        SnowOptions::new()
            .color(fg)
            .particles(particle_count as usize)
            .angle_grad(angle)
            .depth(depth)
            .variation(variation)
            .speed(speed)
            .build()
    );

    let mut termio = TermIO::from_tty()?;
    let frame_duration = Duration::from_secs(1) / fps;

    let mut full_redraw = true;
    let mut winsize = *termio.window_size();

    // winsize.rows = 8;
    // winsize.columns = 16;
    // let orig_winsize = winsize;

    let mut term_frame = TermFrame::new(winsize.into());
    let mut prev_term_frame = term_frame.clone();
    let mut frame = RgbImage::new(draw_mode.size() * term_frame.size(), Rgb::default());

    // if 1 == 1 {
    //     drop(termio);
    //     println!("term_frame.size(): {:?}", term_frame.size());
    //     println!("frame.size(): {:?}", frame.size());
    //     println!("frame_time: {frame_time:?}");
    //     return Ok(());
    // }

    let mut log = VecDeque::new();

    let mut ts_startup = Instant::now();
    let mut ts_frame_start = ts_startup;
    let mut paused = false;
    let mut osd = true;

    let rad = angle * std::f32::consts::PI / 180.0;
    log.push_back(LogEntry {
        timestamp: ts_frame_start,
        message: format!("angle: {angle}°, dx={:.3}, dy={:.3}", rad.cos(), rad.sin()),
    });

    loop {
        // process input
        let event = if paused {
            Some(termio.wait()?)
        } else {
            termio.poll()?
        };

        if let Some(event) = event {
            match event {
                Event::WindowSize { rows, columns } => {
                    winsize.rows = rows;
                    winsize.columns = columns;
                    full_redraw = true;
                    term_frame.resize(&winsize.into());
                    prev_term_frame.resize(term_frame.size());
                    frame.resize(&(draw_mode.size() * term_frame.size()));
                    log.push_back(LogEntry {
                        timestamp: ts_frame_start,
                        message: format!("terminal size: {}, image size: {}", term_frame.size(), frame.size())
                    });
                }
                Event::KeyPress { key: Key::Char(' '), ctrl: false, alt: false, shift: false } => {
                    if paused {
                        paused = false;
                        let now = Instant::now();
                        let dt = now - ts_frame_start;
                        ts_startup += dt;
                        for entry in &mut log {
                            entry.timestamp += dt;
                        }
                        ts_frame_start = now;
                    } else {
                        paused = true;
                    }
                }
                Event::KeyPress { key: Key::Char('+'), ctrl: false, alt, shift: false } => {
                    let amount = if alt { 10 } else { 1 };
                    let count = effect.change_amount(amount);
                    log.push_back(LogEntry {
                        timestamp: ts_frame_start,
                        message: format!("added {amount} particles, new total is {count}")
                    });
                }
                Event::KeyPress { key: Key::Char('-'), ctrl: false, alt, shift: false } => {
                    let amount = if alt { -10 } else { -1 };
                    let count = effect.change_amount(amount);
                    log.push_back(LogEntry {
                        timestamp: ts_frame_start,
                        message: format!("removed {amount} particles, new total is {count}")
                    });
                }
                Event::KeyPress { key: Key::Char('o'), ctrl: false, alt: false, shift: false } => {
                    osd = !osd;
                }
                Event::KeyPress { key: Key::Char('q') | Key::Escape, ctrl: false, alt: false, shift: false } | Event::ConnectionClosed => {
                    break;
                }
                _ => {
                    // TODO: other actions?
                }
            }
        }

        // animate
        let t = ts_frame_start - ts_startup;
        //bg_hsl.h = (bg_hsl.h + t.as_secs_f32() * 0.1) % 1.0;
        //frame.fill(bg_hsl.to_rgb());
        //frame.fill(bg);
        match bg {
            Fill::Solid(color) => {
                frame.fill(color);
            }
            Fill::HorizontalGradient { left, right } => {
                frame.hgradient(left, right);
            }
            Fill::VerticalGradient { top, bottom } => {
                frame.vgradient(top, bottom);
            }
        }

        if !paused {
            effect.animate(frame.size(), frame_duration, t);
        }

        effect.draw(&mut frame);

        term_frame.draw(0, 0, &frame, draw_mode);

        while log.pop_front_if(|entry| (ts_frame_start - entry.timestamp) >= log_timeout).is_some() {}

        if osd {
            for (row, entry) in log.iter().skip(
                if log.len() > term_frame.size().height {
                    log.len() - term_frame.size().height
                } else {
                    0
                }).enumerate() {

                term_frame.draw_text(0, row, fg, bg.start(), &entry.message);
            }
        }

        // draw to terminal
        termio.move_cursor(0, 0)?;
        termio.clear_style()?;

        if full_redraw {
            termio.clear_screen()?;
            term_frame.full_redraw(&mut termio)?;
        } else {
            term_frame.diff_redraw(&prev_term_frame, &mut termio)?;
        }
        termio.flush()?;

        std::mem::swap(&mut term_frame, &mut prev_term_frame);

        full_redraw = false;

        if !paused {
            let ts_frame_end = Instant::now();
            let elapsed = ts_frame_end - ts_frame_start;
            if elapsed < frame_duration {
                sleep(frame_duration - elapsed);

                // XXX: Window resizing causes interrupt! duh!
                // if !interruptable_sleep(frame_time - elapsed) {
                //     break;
                // }
            }

            ts_frame_start += frame_duration;
        }
    }

    //drop(termio);
    //eprintln!("orig_winsize: {orig_winsize}");
    //eprintln!("term_frame.size(): {:?}", term_frame.size());
    //eprintln!("frame.size(): {:?}", frame.size());
    //eprintln!("frame_time: {frame_duration:?}");
    ////eprintln!("LAST FRAME:\n{}", frame);

    Ok(())
}
