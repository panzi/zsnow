use std::{thread::sleep, time::{Duration, Instant}};

use crate::{color::{Hsl, Rgb}, draw_mode::DrawMode, effect::{Effect, SnowOptions}, event::{Event, Key}, rgb_image::RgbImage, term_frame::TermFrame, termio::TermIO};

pub mod ansi_codes;
pub mod borrowed_fd;
pub mod color;
pub mod draw_mode;
pub mod effect;
pub mod epoll;
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

fn main() -> std::io::Result<()> {
    // TODO: parse arguments
    let fps = 60;
    let bg = Rgb::from_u32(0x432d84);
    let fg = Rgb::from_u32(0xffffff);
    let draw_mode = DrawMode::HalfBlock;
    let particle_count = 128;

    let bg_bottom = bg
        .to_hsl()
        .map(|&Hsl { h, s, .. }|
            Hsl { h, s: s * 0.8, l: 0.5 }
        )
        .to_rgb();

    //if 1 == 1 {
    //    println!("{} {} {}", bg, bg.to_hsl(), bg.to_hsl().to_rgb());
    //    return Ok(());
    //}

    let mut effect: Box<dyn Effect> = Box::new(
        SnowOptions::new().color(fg).particles(particle_count).build()
    );

    let mut termio = TermIO::from_tty()?;
    let frame_duration = Duration::from_secs(1) / fps;

    let mut full_redraw = true;
    let mut winsize = *termio.window_size();

    // winsize.rows = 8;
    // winsize.columns = 16;
    let orig_winsize = winsize;

    let mut term_frame = TermFrame::new(winsize.into());
    let mut prev_term_frame = term_frame.clone();
    let mut frame = RgbImage::new(draw_mode.size() * term_frame.size(), bg);

    // if 1 == 1 {
    //     drop(termio);
    //     println!("term_frame.size(): {:?}", term_frame.size());
    //     println!("frame.size(): {:?}", frame.size());
    //     println!("frame_time: {frame_time:?}");
    //     return Ok(());
    // }

    let mut ts_startup = Instant::now();
    let mut ts_frame_start = ts_startup;
    let mut paused = false;

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
                }
                Event::KeyPress { key: Key::Char(' '), ctrl: false, alt: false, shift: false } => {
                    if paused {
                        paused = false;
                        let now = Instant::now();
                        ts_startup += now - ts_frame_start;
                        ts_frame_start = now;
                    } else {
                        paused = true;
                    }
                }
                Event::KeyPress { key: Key::Char('+'), ctrl: false, alt, shift: false } => {
                    let amount = if alt { 10 } else { 1 };
                    effect.change_amount(amount);
                }
                Event::KeyPress { key: Key::Char('-'), ctrl: false, alt, shift: false } => {
                    let amount = if alt { -10 } else { -1 };
                    effect.change_amount(amount);
                }
                Event::KeyPress { key: Key::Char('q'), ctrl: false, alt: false, shift: false } => {
                    break;
                }
                Event::ConnectionClosed => {
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
        frame.vgradient(bg, bg_bottom);

        effect.animate(&mut frame, frame_duration, t);

        term_frame.draw(0, 0, &frame, draw_mode);

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

    drop(termio);
    eprintln!("orig_winsize: {orig_winsize}");
    eprintln!("term_frame.size(): {:?}", term_frame.size());
    eprintln!("frame.size(): {:?}", frame.size());
    eprintln!("frame_time: {frame_duration:?}");
    //eprintln!("LAST FRAME:\n{}", frame);

    Ok(())
}
