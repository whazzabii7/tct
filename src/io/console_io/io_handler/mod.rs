use std::{
    io::{self, Write},
    env,
};
use crate::{
    not,
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


pub struct IoHandler {
    buffer: InputBuffer,
    input: io::Stdin,
    out: io::Stdout,
}

impl IoHandler {
    pub fn init() -> Self {
        Self {
            buffer: InputBuffer::new(),
            out: io::stdout(),
            input: io::stdin(),
        }
    }

    pub fn print_banner(&self) -> Result<(), IoError> {
        print!("{}", Colorize::Fg(Color::BannerColor));
        println!("╔════════════════════════════╗");
        println!("║   Terminal Call Tracker    ║");
        println!("╚════════════════════════════╝");
        print!("{}", Colorize::Reset);
        self.flush()?;
        Ok(())
    }

    pub fn prompt(&self, mode: &IoMode) -> Result<(), IoError> {
        print!("{}> ", mode);
        self.flush()?;
        Ok(())
    }

    pub fn read(&self) -> Result<String, IoError> {
        let mut input = String::new();
        self.get_input(&mut input)?;
        Ok(input.trim().to_owned())
    }

    pub fn flush(&self) -> Result<(), IoError> {
        if let Err(_) = io::stdout().flush() {
            return Err(IoError::FailedFlush);
        }
        Ok(())
    }

    pub fn get_input(&self, buf: &mut String) -> Result<(), IoError> {
        match io::stdin().read_line(buf) {
            Ok(c) => if c == 0 { return Err(IoError::FailedRead);},
            Err(e) => return Err(IoError::Io(e)),
        }
        Ok(())
    }

    pub fn command_mode(&mut self) -> Result<(), IoError> {
        print!("{}[TCT]{} Start interactive session...\n",
            Colorize::Fg(Color::TctGreen), Colorize::Reset);
        print!("{}[INFO]{} Enter 'q' to quit.\n\n",
            Colorize::Fg(Color::InfoBlue), Colorize::Reset);
        command_mode::command_loop(self)?;
        Ok(())
    }

    pub fn input_mode(&mut self) -> Result<(), IoError> {
        if let Err(e) = RawMode::enable() { 
            return Err(IoError::Io(e))
        }
        self.prompt(&IoMode::Input)?;
        input_mode::input_loop(self)?;
        Ok(())
    }
}
