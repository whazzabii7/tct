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
    use windows_sys::Win32::System::Console::{
        GetConsoleMode, SetConsoleMode, GetStdHandle,
        ENABLE_PROCESSED_INPUT, ENABLE_LINE_INPUT, ENABLE_ECHO_INPUT, STD_INPUT_HANDLE,
    };
    use windows_sys::Win32::System::Console::SetConsoleCtrlHandler;

    /// Represents a raw mode console session
    pub struct RawMode {
        original: u32,  // Original console mode
        handle: isize,  // Handle to console input
    }

    impl RawMode {
        /// Enables raw mode by disabling line input, echo, and processed input
        pub fn enable() -> io::Result<Self> {
            unsafe {
                let handle = GetStdHandle(STD_INPUT_HANDLE);
                let mut mode = 0;

                if GetConsoleMode(handle, &mut mode) == 0 {
                    return Err(io::Error::last_os_error());
                }

                let original = mode;
                mode &= !(ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT | ENABLE_PROCESSED_INPUT);

                if SetConsoleMode(handle, mode) == 0 {
                    return Err(io::Error::last_os_error());
                }

                Ok(Self { original, handle })
            }
        }
    }

    impl Drop for RawMode {
        fn drop(&mut self) {
            unsafe {
                SetConsoleMode(self.handle, self.original);
            }
        }
    }
}

