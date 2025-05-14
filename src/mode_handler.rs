//! # === mode_handler ===
//! 
//! This module actually executes the main loop.
//! Also the ModeHandler manages both I/O handlers,
//! the [`FileHandler`] and the [`IoHandler`], which handles
//! the console input and output. 
//!
//! The ModeHandler is initialized with an instance of the 
//! I/O handlers and with an empty [`History`].

use crate::{
    console_io::IoHandler,
    file_io::FileHandler,
    tct_error::*,
    history::*,
};

/// Handler for setup and communication of the I/O handlers
pub struct ModeHandler {
    io_handler: IoHandler,
    file_handler: FileHandler,
    current_history: History,
}

impl ModeHandler {
    /// Default constructor of ModeHandler
    pub fn init() -> Self {
        Self {
            io_handler: IoHandler::init(),
            file_handler: FileHandler::init(),
            current_history: History::new(),
        }
    }

    /// Starts the main execution of the program
    pub fn run(&mut self) -> Result<(), IoError> {
        self.io_handler.print_banner()?;
        self.io_handler.command_mode()?;
        Ok(())
    }
}
