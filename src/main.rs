use std::time::{Duration, Instant};

use crate::{event::{Event, Key}, termio::TermIO};

pub mod ansi_codes;
pub mod borrowed_fd;
pub mod color;
pub mod draw_mode;
pub mod epoll;
pub mod event;
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
    let mut termio = TermIO::from_tty()?;
    let fps = 60;
    let frame_time = Duration::from_secs(1) / fps;

    termio.move_cursor(0, 0)?;
    termio.flush()?;

    let mut winsize = *termio.window_size();

    loop {
        let ts_before = Instant::now();

        // process input
        if let Some(event) = termio.poll()? {
            match event {
                Event::WindowSize { rows, columns } => {
                    // TODO: resize, full redraw
                    winsize.rows = rows;
                    winsize.columns = columns;
                }
                Event::ConnectionClosed => {
                    break;
                }
                Event::KeyPress { key: Key::Char('q'), ctrl: false, alt: false, shift: false } => {
                    break;
                }
                _ => {
                    // TODO: other actions?
                }
            }
        }

        // draw (TODO)

        let ts_after = Instant::now();
        let elapsed = ts_after - ts_before;
        if elapsed < frame_time {
            if !interruptable_sleep(frame_time - elapsed) {
                break;
            }
        }
    }


    Ok(())
}
