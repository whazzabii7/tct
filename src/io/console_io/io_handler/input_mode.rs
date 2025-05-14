//! # === input_mode ===
//!
//! routine that represents the input mode.
//! while the loop the user is able to use configured
//! executable with given commands.
//!
//! ## Example
//! ```bash 
//! cat samplefile.txt | grep searchedexpr
//! ```

use crate::{
    not,
    io::console_io::{
        IoMode,
        IoHandler,
//         colors,
    },
    tct_error::IoError,
};
use std::{
    io::{self, Read, StdinLock},
    thread,
    time::Duration,
};

// recognices if `ESC` was pressed or a escaped multi byte key.
// exits [`input_mode`] on single `ESC`
fn process_escaped_key(handle: &mut StdinLock<'static>, buffer: &mut [u8;1]) -> bool {
    thread::sleep(Duration::from_millis(10));
    match handle.read(buffer) {
        Ok(0) | Err(_) => {
            println!("\nESC pressed alone. Exiting.");
            return true;
        }
        Ok(_) => {
            println!("\nEscape sequence.");
        }
    }
    false
}

// interprets pressed non escaped key. flushes [`buffer`] on `enter`,
// handles `backspace` to pop buffer, else print pressed key to console and [`buffer`]
fn process_key(io_handler: &mut IoHandler, byte: u8) -> Result<(), IoError> {
    match byte {
        0x0A => {                                   // Enter key: flush buffer and use 
            let out = io_handler.buffer.flush();  // the string result
            println!("\nInput: {:?}", out);
            io_handler.prompt(&IoMode::Input)?; 
        }
        0x08 | 0x7F => {                            // Backspace: delete last typed key
            io_handler.buffer.pop();
            io_handler.flush()?; 
        }
        _ => {                                      // Write typed key to buffer
            print!("{}", char::from(byte));
            io_handler.flush()?;
            io_handler.buffer.push(byte);
        }
    }
    Ok(())
}

// checks user input and forks processing depending if there was an escape byte or not
fn read_input(io_handler: &mut IoHandler, buffer: &mut [u8;1], handle: &mut StdinLock<'static>, stopped: &mut bool) -> Result<(), IoError> {
    match handle.read(buffer) {
        Ok(1) => {
            let byte = buffer[0];
            if byte == 0x1B {
                if process_escaped_key(handle, buffer) {
                    *stopped = true;
                }
            } else {
                process_key(io_handler, byte)?;
            }
        },
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
            thread::sleep(Duration::from_millis(10));
        },
        Err(e) => return Err(IoError::Io(e)),
        _ => {}
    }
    Ok(())
}

/// module is called here:
pub fn input_loop(io_handler: &mut IoHandler) -> Result<(), IoError> {
    let mut buffer = [0u8;1];
    let mut handle = io_handler.input.lock();
    let mut stopped = false;

    // processed till signal tells to stop
    while not!(stopped) {
        read_input(io_handler, &mut buffer, &mut handle, &mut stopped)?;
    }

    Ok(())
}
