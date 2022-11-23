use std::io::{stdout, Write};

use colored::Colorize;
use crossterm::event::{KeyEvent, KeyModifiers, KeyCode};

use crate::{command::Command, format::Format};

use super::{Input, KeyBoardSignalGenerator, KeyBoardSignal, signal::signal_genertor, Signal, messages::WELCOME};

impl Iterator for Input {
    type Item = Command;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = vec![];
        let mut buf_temp = buf.clone();
        let mut history_position = self.history.len();

        let mut _len = self.prompt_prefix(true);
        for signal in &self.generator {
            match signal {
                KeyBoardSignal::BackSpace => if self.cursor != 0 {
                    buf.remove(self.cursor - 1);
                    self.cursor -= 1;
                },
                KeyBoardSignal::Delete => if self.cursor != buf.len() {
                    buf.remove(self.cursor);
                }
                KeyBoardSignal::Ctrl(c) => match c {
                    'c' => self.signal = Signal::Interrupt,
                    'd' => std::process::exit(0),
                    _ => ()
                },
                KeyBoardSignal::CursorMove(m) => match m {
                    0 => self.cursor = 0,
                    3 => self.cursor = buf.len(),
                    1 => if self.cursor != 0 {
                        self.cursor -= 1;
                    },
                    2 => if self.cursor != buf.len() {
                        self.cursor += 1;
                    },
                    _ => ()
                },
                KeyBoardSignal::Enter => {
                    self.write_line();
                    self.cursor = 0;
                    let temp = buf.iter().collect::<String>();
                    if temp != "" && if let Some(s) = self.history.last() { temp != s.clone() } else { true } {
                        self.history.push(buf.iter().collect::<String>());
                    }

                    return Some(Format::from(buf.iter().collect::<String>()).to_command());
                },
                KeyBoardSignal::Fx(_) => (),
                KeyBoardSignal::History(prev) => if prev {
                    if history_position == self.history.len() {
                        buf_temp = buf.clone();
                    }

                    if history_position != 0 {
                        history_position -= 1;
                    }
                    
                    if history_position != self.history.len() {
                        buf = self.history[history_position].chars().collect();
                        self.cursor = buf.len();
                    }
                } else {
                    if self.history.len() != 0 && history_position == self.history.len() - 1 {
                        history_position += 1;
                        buf = buf_temp.clone();
                        self.cursor = buf.len();
                    } else if history_position != self.history.len() {
                        history_position += 1;
                        buf = self.history[history_position].chars().collect();
                        self.cursor = buf.len();
                    }
                },
                KeyBoardSignal::Insert(c) => {
                    if history_position != self.history.len() {
                        buf = self.history[history_position - 1].chars().collect();
                        history_position = 0;
                    }

                    buf.insert(self.cursor, c);
                    self.cursor += 1;
                },
                KeyBoardSignal::Tab => (),
                _ => ()
            }

            _len = self.prompt_prefix(true);
            self.write(Format::from(buf.iter().collect::<String>()).transform(None));
            if self.cursor != buf.len() {
                self.prompt_prefix(false);
                self.write(Format::from(buf.iter().collect::<String>()).transform(Some(self.cursor)));
            }
        }

        None
    }
}

impl Iterator for &KeyBoardSignalGenerator {
    type Item = KeyBoardSignal;

    fn next(&mut self) -> Option<Self::Item> {
        let KeyEvent {
            code,
            modifiers,
            ..
        } = self.recv.recv().unwrap();

        Some(match (code, modifiers) {
            (KeyCode::Backspace, _) => KeyBoardSignal::BackSpace,
            (KeyCode::Delete, _) => KeyBoardSignal::Delete,
            (KeyCode::Enter, _) => KeyBoardSignal::Enter,
            (KeyCode::Tab, _) => KeyBoardSignal::Tab,
            (KeyCode::Char(c), KeyModifiers::CONTROL) => KeyBoardSignal::Ctrl(c),
            (KeyCode::Char(c), KeyModifiers::NONE) => KeyBoardSignal::Insert(c),
            (KeyCode::Char(c), KeyModifiers::SHIFT) => KeyBoardSignal::Insert(c.to_ascii_uppercase()),
            (KeyCode::Up, _) => KeyBoardSignal::History(true),
            (KeyCode::Down, _) => KeyBoardSignal::History(false),
            (KeyCode::Left, _) => KeyBoardSignal::CursorMove(1),
            (KeyCode::Right, _) => KeyBoardSignal::CursorMove(2),
            (KeyCode::Home, _) => KeyBoardSignal::CursorMove(0),
            (KeyCode::End, _) => KeyBoardSignal::CursorMove(3),
            (KeyCode::F(x), _) => KeyBoardSignal::Fx(x.into()),
            _ => KeyBoardSignal::None
        })
    }
}

impl Input {
    pub fn new() -> Self {
        Self {
            user_name: "guest".into(),
            path: std::env::current_dir().unwrap(),
            permission: super::Permission::Normal,
            signal: super::Signal::Waiting,
            cursor: 0,
            history: vec![],
            generator: signal_genertor()
        }
    }

    // pub fn from(&self, user: Option<String>) -> Self {
        
    // }

    pub fn welcome_message(&self) {
        print!("{}", WELCOME);
    }

    pub fn write(&self, message: impl Into<String>) {
        stdout().write(message.into().as_bytes()).unwrap();
        stdout().flush().unwrap();
    }

    fn write_line(&self) {
        stdout().write("\n".as_bytes()).unwrap();
        stdout().flush().unwrap();
    }

    fn prompt_prefix(&self, space: bool) -> usize {
        let (width, _height) = crossterm::terminal::size().unwrap();
        let prefix_len = self.path.to_str().unwrap().len() + self.user_name.len() + 9;
        let message = format!(
            "{} {}({})> ",
            "(sks)".bold().blue(),
            self.user_name.bold().red(),
            self.path.to_str().unwrap().bold());

        if space {
            stdout().write(format!("\r{}", String::from(' ').repeat(width.into())).as_bytes()).unwrap();
        }

        stdout().write(format!("\r{}", message).as_bytes()).unwrap();
        stdout().flush().unwrap();

        prefix_len
    }
}