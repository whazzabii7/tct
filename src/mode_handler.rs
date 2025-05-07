use crate::{
    console_io::{IoHandler, IoMode},
    tct_error::*,
};

// use std::error::Error;

use crate::history::*;

pub struct ModeHandler {
    os: String,
    current_mode: IoMode,
    io_handler: IoHandler,
    file_handler: u8,
    current_history: History,
}

impl ModeHandler {
    pub fn init(os: String) -> Self {
        Self {
            io_handler: IoHandler::init(os.clone()),
            os,
            current_mode: IoMode::Command,
            file_handler: 0,
            current_history: History{}
        }
    }

    pub fn run(&mut self) -> Result<(), IoError> {
        self.io_handler.print_banner()?;
        main_loop(&mut self.io_handler)?;
        Ok(())
    }
}

fn main_loop(io_handler: &mut IoHandler) -> Result<(), IoError> {
    let current_mode = IoMode::Command;
    print!("\x1B[1;32m[TCT]\x1B[0m Starte interaktive Sitzung...\n");
    print!("\x1B[1;34m[INFO]\x1B[0m Gib ':q' ein zum Beenden.\n\n");
    loop {
        _ = io_handler.prompt(&current_mode);
        match io_handler.read() {
            Err(e) => panic!("paniced with: {:?}", e),
            Ok(input) => {
                match input.as_str() {
                    "i" => {
                        _ = io_handler.input_mode(&IoMode::Input);
                    },
                    "q" => break,
                    "exit" => break,
                    _ => println!("\x1B[1;33m[Du hast eingegeben:]\x1B[0m {}", input),
                }
            }
        }
    }
    Ok(())
}
