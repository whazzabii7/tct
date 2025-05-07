#[cfg(unix)]
pub mod raw_mode {
    use std::os::unix::io::AsRawFd;
    use std::io;
    use libc::{termios, tcgetattr, tcsetattr, TCSANOW, ECHO, ICANON};

    pub struct RawMode {
        original: termios,
        fd: i32,
    }

    impl RawMode {
        pub fn enable() -> io::Result<Self> {
            let fd = std::io::stdin().as_raw_fd();
            let mut term: termios = unsafe { std::mem::zeroed() };

            unsafe {
                tcgetattr(fd, &mut term);
                let original = term;
                term.c_lflag &= !(ICANON | ECHO);
                tcsetattr(fd, TCSANOW, &term);
                Ok(Self { original, fd })
            }
        }
    }

    impl Drop for RawMode {
        fn drop(&mut self) {
            unsafe {
                tcsetattr(self.fd, TCSANOW, &self.original);
            }
        }
    }
}

#[cfg(windows)]
mod raw_mode {
    use std::io;
    use windows_sys::Win32::System::Console::{
        GetConsoleMode, SetConsoleMode, GetStdHandle,
        ENABLE_PROCESSED_INPUT, ENABLE_LINE_INPUT, ENABLE_ECHO_INPUT, STD_INPUT_HANDLE,
    };

    pub struct RawMode {
        original: u32,
        handle: isize,
    }

    impl RawMode {
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

