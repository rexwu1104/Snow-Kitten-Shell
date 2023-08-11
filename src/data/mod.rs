mod impls;
mod signal;
mod messages;

use std::{path::PathBuf, thread::Thread, any::Any, process::Child};

use crossbeam_channel::{Receiver, Sender};
use crossterm::event::KeyEvent;

pub struct Input<'a> {
    user_name: String,
    path: PathBuf,
    permission: Permission,
    signal: Signal,

    cursor: usize,
    history: Vec<String>,
    generator: KeyBoardSignalGenerator,

    bin_files: Vec<PathBuf>,
    processing: Option<Child>,

    sender: Sender<&'a dyn Any>,
    receiver: Receiver<&'a dyn Any>,
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