use std::fmt;

pub mod io_handler;
pub use io_handler::IoHandler;

mod colors;
use colors::*;

const ESC: &str = "\x1B";

#[derive(Debug)]
pub enum IoMode {
    Command,
    Input
}

impl fmt::Display for IoMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IoMode::Command => write!(f, 
                "{}[COM]{}",
                Colorize::Fg(Color::Command.into()), Colorize::Reset
            ),
            IoMode::Input => write!(f, 
                "{}[IN]{}",
                Colorize::Fg(Color::Input.into()), Colorize::Reset
            ),
        }
    }
}
