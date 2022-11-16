use crossbeam_channel::Receiver;
use crossterm::event::KeyEvent;

use crate::signal::signal_handle;

#[derive(Clone)]
pub struct Input {
    recevier: Receiver<KeyEvent>
}

impl Input {
    pub fn new() -> Self {
        Self {
            recevier: signal_handle()
        }
    }
}

impl Iterator for Input {
    type Item = KeyEvent;
    
    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(key) = self.recevier.clone().recv() {
            Some(key)
        } else {
            None
        }
    }
}