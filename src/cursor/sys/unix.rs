use std::{
    io::{self, Write},
    time::Duration,
};

use crate::Error;
use crate::{
    event::{filter::CursorPositionFilter, poll_internal, read_internal, InternalEvent},
    terminal::{disable_raw_mode, enable_raw_mode, sys::is_raw_mode_enabled},
};

/// Returns the cursor position (column, row).
///
/// The top left cell is represented as `(0, 0)`.
///
/// On unix systems, this function will block and possibly time out while
/// [`crossterm::event::read`](crate::event::read) or [`crossterm::event::poll`](crate::event::poll) are being called.
pub fn position() -> Result<(usize, usize), Error> {
    if is_raw_mode_enabled() {
        read_position_raw()
    } else {
        read_position()
    }
}

fn read_position() -> Result<(usize, usize), Error> {
    enable_raw_mode()?;
    let pos = read_position_raw();
    disable_raw_mode()?;
    pos
}

fn read_position_raw() -> Result<(usize, usize), Error> {
    // Use `ESC [ 6 n` to and retrieve the cursor position.
    let mut stdout = io::stdout();
    stdout.write_all(b"\x1B[6n")?;
    stdout.flush()?;

    loop {
        match poll_internal(Some(Duration::from_millis(2000)), &CursorPositionFilter) {
            Ok(true) => {
                if let Ok(InternalEvent::CursorPosition(x, y)) =
                    read_internal(&CursorPositionFilter)
                {
                    return Ok((x, y));
                }
            }
            Ok(false) => {
                return Err(Error::CursorPositionTimeout);
            }
            Err(_) => {}
        }
    }
}
