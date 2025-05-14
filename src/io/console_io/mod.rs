//! # === console_io ===
//!
//! defines console I/O modules and I/O Modes.

use std::fmt;

pub mod io_handler;
pub use io_handler::IoHandler;

pub mod colors;
use colors::*;

const ESC: &str = "\x1B";

/// the different Input modes
#[derive(Debug)]
pub enum IoMode {
    Command,
    Input
}


// use Display to create a reusable TUI block
// to show current mode
impl fmt::Display for IoMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IoMode::Command => write!(f, 
                "{}[COMD]{}",
                Colorize::Fg(Color::Command.into()), Colorize::Reset
            ),
            IoMode::Input => write!(f, 
                "{}[INPT]{}",
                Colorize::Fg(Color::Input.into()), Colorize::Reset
            ),
        }
    }
}
