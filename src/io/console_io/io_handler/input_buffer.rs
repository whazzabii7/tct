//! # === input_buffer ===
//!
//! buffer to handle current input, is writen to [`History`]
//! after flushing.

// use crate::tct_error::BufferError;

const MAX_BUFFER_SIZE: usize = 21504;

/// Buffer acts like a stack with pointer
#[derive(Debug)]
pub struct InputBuffer {
    mem: Box<[u8;MAX_BUFFER_SIZE]>,
    ibp: usize,
}

impl InputBuffer {
    /// Default constructor
    pub fn new() -> Self {
        Self {
            mem: Box::new([0u8;MAX_BUFFER_SIZE]),
            ibp: 0,
        }
    }

    /// joins current `buffer` content to `String`
    pub fn flush(&mut self) -> String {
        print!("\r");
        let mut count: usize = 0;   
        let mut str_vec = Vec::new();
        while count < self.ibp {
            str_vec.push(char::from(self.mem[count]));
            count += 1;
        }
        self.ibp = 0;
        str_vec.iter().collect::<String>() 
    }

    /// adds `char` to `buffer`
    pub fn push(&mut self, ch: u8) {
        self.mem[self.ibp] = ch;
        self.ibp += 1;
    }

    /// removes `char` from `buffer`
    pub fn pop(&mut self) {
        if self.ibp > 0 {
            print!("\x08 \x08");
            self.ibp -= 1;
        }
    }

    /// removes full content without writing
    pub fn clear(&mut self) {
        self.ibp = 0;
    }

}
