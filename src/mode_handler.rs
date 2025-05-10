use crate::{
    console_io::IoHandler,
    tct_error::*,
    history::*,
};

pub struct ModeHandler {
    io_handler: IoHandler,
    file_handler: u8,
    current_history: History,
}

impl ModeHandler {
    pub fn init() -> Self {
        Self {
            io_handler: IoHandler::init(),
            file_handler: 0,
            current_history: History::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), IoError> {
        self.io_handler.print_banner()?;
        self.io_handler.command_mode()?;
        Ok(())
    }
}
