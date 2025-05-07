mod io;
mod mode_handler;
mod history;
mod tct_error;

use io::*;
use mode_handler::ModeHandler;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let os: String;
    if cfg!(target_os = "windows") {
        os = String::from("windows");
    } else if cfg!(target_os = "linux") {
        os = String::from("linux")
    } else {
        panic!("OS is not supported");
    }

    let mut mode_handler = ModeHandler::init(String::from(&os));
    mode_handler.run()?;
    
    println!("test: {}", os);
    Ok(())
}
