use crate::tct_error::BufferError;

const MAX_BUFFER_SIZE: usize = 21504;

#[derive(Debug)]
pub struct InputBuffer {
    mem: Box<[u8;MAX_BUFFER_SIZE]>,
    ibp: usize,
}

impl InputBuffer {
    pub fn new() -> Self {
        Self {
            mem: Box::new([0u8;MAX_BUFFER_SIZE]),
            ibp: 0,
        }
    }

    pub fn flush(&mut self) -> Result<String, BufferError> {
        print!("\r");
        let mut count: usize = 0;   
        let mut str_vec = Vec::new();
        while count < self.ibp {
            str_vec.push(char::from(self.mem[count]));
            count += 1;
        }
        Ok(str_vec.iter().collect::<String>()) 
    }

    pub fn push(&mut self, ch: u8) {
        self.mem[self.ibp] = ch;
        self.ibp += 1;
    }

    pub fn clear(&mut self) {
        self.ibp = 0;
    }

}
