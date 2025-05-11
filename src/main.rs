// #########################################
// #         Terminal Call Tracker         #
// #########################################
// ╔═══════════════════════════════════════╗
// ║TCT uses a CLI tool that is specified  ║
// ║in a config file and creates a history ║
// ║of the exact commands used and the full║
// ║output received.                       ║
// ╚═══════════════════════════════════════╝
mod io;
mod mode_handler;
mod history;
mod tct_error;

#[macro_use]
mod macros;

use io::*;
use mode_handler::ModeHandler;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    let mut tester = history::History::new();
    _ = tester.set_date();
    tester.set_name("Test");
    tester.add_entry("This is the test Command!", "CommandPrompt");
    tester.add_entry("This is the test Answer!", "OutputAnswer");
    let mut mode_handler = ModeHandler::init();
    mode_handler.run()?;
    
    println!("{}", tester.get_date());
    let tt = tester.entries.iter();
    for i in tt {
        println!("{}", i);
    }
    Ok(())
}
