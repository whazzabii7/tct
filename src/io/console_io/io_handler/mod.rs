use std::io::{self, Read, Write};
use crate::{
    io::console_io::IoMode,
    tct_error::IoError,
};
    
mod raw_mode;
use raw_mode::raw_mode::*;

mod input_buffer;
use input_buffer::*;

pub struct IoHandler {
    buffer: InputBuffer,
    os: String,
    input: io::Stdin,
    out: io::Stdout,
}

impl IoHandler {
    pub fn init(os: String) -> Self {
        Self {
            os,
            buffer: InputBuffer::new(),
            out: io::stdout(),
            input: io::stdin(),
        }
    }

    pub fn print_banner(&self) -> Result<(), IoError> {
        print!("\x1B[1;36m");
        println!("╔════════════════════════════╗");
        println!("║        TCT CLI Tool        ║");
        println!("╚════════════════════════════╝");
        print!("\x1B[0m");
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

    pub fn input_mode(&mut self, mode: &IoMode) -> std::io::Result<()> {
        let _raw = RawMode::enable()?;
        _ = self.prompt(mode);
        for byte in self.input.lock().bytes() {
            let b = byte?;
            match b {
                27 => {
                    println!("\nESC pressed. BYE!");
                    break;
                }
                b'\n' => {
                    let out = self.buffer.flush().unwrap();
                    println!("\nInput: {:?}", out);
                    break;
                }
                _ => {
                    print!("{}", char::from(b));
                    _ = self.flush();
                    self.buffer.push(b);
                }
            }
        }
        Ok(())
    }
}
