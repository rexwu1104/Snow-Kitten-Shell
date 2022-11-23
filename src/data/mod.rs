mod impls;
mod signal;
mod messages;

use std::path::PathBuf;

use crossbeam_channel::Receiver;
use crossterm::event::KeyEvent;

pub struct Input {
    user_name: String,
    path: PathBuf,
    permission: Permission,
    signal: Signal,

    cursor: usize,
    history: Vec<String>,
    generator: KeyBoardSignalGenerator
}

pub(super) enum Permission {
    Root,
    Normal
}

pub(super) enum Signal {
    Waiting,
    Processing,
    Listening,
    Interrupt
}

#[derive(Debug)]
pub(super) enum KeyBoardSignal {
    CursorMove(usize),
    BackSpace,
    Delete,
    Enter,
    Tab,
    Ctrl(char),
    Insert(char),
    History(bool),
    Fx(usize),
    None
}

#[derive(Clone)]
pub(super) struct KeyBoardSignalGenerator {
    recv: Receiver<KeyEvent>
}