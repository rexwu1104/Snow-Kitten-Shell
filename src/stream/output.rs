use std::io::{Stdout, stdout, Write};

pub struct Output {
    ptr: Stdout
}

impl Output {
    pub fn new() -> Self {
        Self {
            ptr: stdout()
        }
    }

    pub fn fwrite(&mut self, msg: impl Into<String>) -> std::io::Result<usize> {
        let result = self.ptr.write(msg.into().as_bytes());
        self.ptr.flush().unwrap();
        
        result
    }

    pub fn write(&mut self, msg: impl Into<String>) -> std::io::Result<usize> {
        self.ptr.write(msg.into().as_bytes())
    }
}