use crate::{
    not,
    io::console_io::{
        IoMode,
        io_handler::IoHandler,
        colors::*,
    },
    tct_error::IoError,
};

fn list_of_commands(input: &str, handler: &mut IoHandler, stopped: &mut bool) -> Result<(), IoError> {
    // TODO: add the missing commands
    match input {
        "i" => {
            handler.input_mode()?;
        },
        "q" => *stopped = true,
        "exit" => *stopped = true,
        _ => println!("{fg}You entered: {reset}{in}",
            fg = Colorize::Fg(Color::Orange),
            in = input, 
            reset = Colorize::Reset),
    }
    Ok(())
}

fn process_command(handler: &mut IoHandler, stopped: &mut bool) -> Result<(), IoError> {
    match handler.read() {
        Err(e) => panic!("paniced with: {:?}", e),
        Ok(input) => {
            list_of_commands(&input, handler, stopped)?;
        }
    }
    Ok(())
}

pub fn command_loop(handler: &mut IoHandler) -> Result<(), IoError> {
    let mut stopped = false;
    while not!(stopped) {
        handler.prompt(&IoMode::Command)?;
        process_command(handler, &mut stopped)?;
    }
    Ok(())
}
