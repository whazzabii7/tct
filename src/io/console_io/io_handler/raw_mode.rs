// Only compile this module on Unix-like systems
#[cfg(unix)]
pub mod raw_mode {
    use std::os::unix::io::AsRawFd;
    use std::io;
    use libc::{
        termios, tcgetattr, tcsetattr, fcntl, F_GETFL, F_SETFL, O_NONBLOCK,
        TCSANOW, ECHO, ICANON,
    };

    /// Represents a raw mode terminal session
    pub struct RawMode {
        original: termios, // Original terminal settings
        fd: i32,           // File descriptor for stdin
        original_flags: i32, // Original file descriptor flags
    }

    impl RawMode {
        /// Enables raw mode by disabling canonical input and echoing,
        /// and sets stdin to non-blocking mode
        pub fn enable() -> io::Result<Self> {
            let fd = std::io::stdin().as_raw_fd();
            let mut term: termios = unsafe { std::mem::zeroed() };

            unsafe {
                // Save current terminal settings
                tcgetattr(fd, &mut term);
                let original = term;

                // Set raw mode (disable canonical + echo)
                term.c_lflag &= !(ICANON | ECHO);
                if tcsetattr(fd, TCSANOW, &term) != 0 {
                    return Err(io::Error::last_os_error());
                }

                // Set stdin to non-blocking mode
                let original_flags = fcntl(fd, F_GETFL);
                fcntl(fd, F_SETFL, original_flags | O_NONBLOCK);

                Ok(Self { original, fd, original_flags })
            }
        }
    }

    impl Drop for RawMode {
        fn drop(&mut self) {
            unsafe {
                // Restore terminal settings
                tcsetattr(self.fd, TCSANOW, &self.original);
                // Restore original file flags
                fcntl(self.fd, F_SETFL, self.original_flags);
            }
        }
    }
}

// Only compile this module on Windows systems
#[cfg(windows)]
pub mod raw_mode {
    use std::io;
    use windows_sys::Win32::{
        Foundation::HANDLE,
        System::Console::{
            GetConsoleMode, SetConsoleMode, GetStdHandle,
            ENABLE_PROCESSED_INPUT, ENABLE_LINE_INPUT, ENABLE_ECHO_INPUT,
            ENABLE_VIRTUAL_TERMINAL_PROCESSING,
            STD_INPUT_HANDLE, STD_OUTPUT_HANDLE,
        },
    };

    /// Represents a raw mode console session with ANSI color support
    pub struct RawMode {
        input_handle: HANDLE,
        output_handle: HANDLE,
        original_input_mode: u32,
        original_output_mode: u32,
    }

    impl RawMode {
        /// Enables raw mode and ANSI color support
        pub fn enable() -> io::Result<Self> {
            unsafe {
                // Input: raw mode
                let input_handle = GetStdHandle(STD_INPUT_HANDLE);
                let mut input_mode = 0;
                if GetConsoleMode(input_handle, &mut input_mode) == 0 {
                    return Err(io::Error::last_os_error());
                }
                let original_input_mode = input_mode;
                input_mode &= !(ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT | ENABLE_PROCESSED_INPUT);
                if SetConsoleMode(input_handle, input_mode) == 0 {
                    return Err(io::Error::last_os_error());
                }

                // Output: enable ANSI escape sequences
                let output_handle = GetStdHandle(STD_OUTPUT_HANDLE);
                let mut output_mode = 0;
                if GetConsoleMode(output_handle, &mut output_mode) == 0 {
                    return Err(io::Error::last_os_error());
                }
                let original_output_mode = output_mode;
                output_mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
                if SetConsoleMode(output_handle, output_mode) == 0 {
                    return Err(io::Error::last_os_error());
                }

                Ok(Self {
                    input_handle,
                    output_handle,
                    original_input_mode,
                    original_output_mode,
                })
            }
        }
    }

    impl Drop for RawMode {
        fn drop(&mut self) {
            unsafe {
                SetConsoleMode(self.input_handle, self.original_input_mode);
                SetConsoleMode(self.output_handle, self.original_output_mode);
            }
        }
    }
}
