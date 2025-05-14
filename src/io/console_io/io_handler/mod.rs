//! # === io_handler ===
//!
//! Defines Handler for console I/O

use std::io::{self, Write};
use crate::{
    io::console_io::{
        colors::*,
        IoMode,
    },
    tct_error::IoError,
};

mod raw_mode;
use raw_mode::raw_mode::*;

mod command_mode;
mod input_mode;

mod input_buffer;
use input_buffer::*;

/// Handles console I/O
pub struct IoHandler {
    buffer: InputBuffer,
    input: io::Stdin,
    out: io::Stdout,
}

impl IoHandler {
    /// Default constructor
    pub fn init() -> Self {
        Self {
            buffer: InputBuffer::new(),
            out: io::stdout(),
            input: io::stdin(),
        }
    }

    /// like the logo of the program
    pub fn print_banner(&self) -> Result<(), IoError> {
        print!("{}", Colorize::Fg(Color::BannerColor));
        println!("╔════════════════════════════╗");
        println!("║   Terminal Call Tracker    ║");
        println!("╚════════════════════════════╝");
        print!("{}", Colorize::Reset);
        self.flush()?;
        Ok(())
    }

    /// processes and displays current prompt; TODO: maybe make userdefined (like from a lua
    /// function)
    pub fn prompt(&self, mode: &IoMode) -> Result<(), IoError> {
        print!("{}> ", mode);
        self.flush()?;
        Ok(())
    }

    /// converts input from user
    pub fn read(&self) -> Result<String, IoError> {
        let mut input = String::new();
        get_input(&mut input)?;
        Ok(input.trim().to_owned())
    }

    /// flushes the [stdout], wrapper function for `std::io::stdout().flush()`
    pub fn flush(&self) -> Result<(), IoError> {
        if let Err(_) = io::stdout().flush() {
            return Err(IoError::FailedFlush);
        }
        Ok(())
    }

    /// calls the [`command_mode`] loop
    pub fn command_mode(&mut self) -> Result<(), IoError> {
        print!("{}[TCT]{} Start interactive session...\n",
            Colorize::Fg(Color::TctGreen), Colorize::Reset);
        print!("{}[INFO]{} Enter 'q' to quit.\n\n",
            Colorize::Fg(Color::InfoBlue), Colorize::Reset);
        command_mode::command_loop(self)?;
        Ok(())
    }

    /// calls the input_mode loop
    pub fn input_mode(&mut self) -> Result<(), IoError> {
        if let Err(e) = RawMode::enable() { 
            return Err(IoError::Io(e))
        }
        self.prompt(&IoMode::Input)?;
        input_mode::input_loop(self)?;
        Ok(())
    }
}

// actually gets the user input
fn get_input(buf: &mut String) -> Result<(), IoError> {
        match io::stdin().read_line(buf) {
            Ok(c) => if c == 0 { return Err(IoError::FailedRead);},
            Err(e) => return Err(IoError::Io(e)),
        }
        Ok(())
    }


