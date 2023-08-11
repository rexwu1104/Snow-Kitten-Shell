use std::{io::{stdout, Write}, thread::spawn, time::Duration, path::PathBuf};

use colored::Colorize;
use crossbeam_channel::bounded;
use crossterm::event::{KeyEvent, KeyModifiers, KeyCode};

use crate::{command::Command, format::Format, system::load_executable};
use super::{Input, KeyBoardSignalGenerator, KeyBoardSignal, signal::signal_genertor, Signal, messages::WELCOME, Permission};

impl<'a> Iterator for Input<'a> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        match self.signal {
            Signal::Waiting => {
                let command = self.waiting();
                if let Some(cmd) = command.clone() {
                    let process = cmd.execute(self).expect("command expect");

                    self.processing = Some(process);
                }

                Some(())
            },
            Signal::Processing => {
                self.processing();

                Some(())
            }
            Signal::Listening => {
                self.listening();

                Some(())
            },
            Signal::Interrupt => {
                self.interrupt();

                Some(())
            }
        }
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

impl Input<'_> {
    pub fn new() -> Self {
        let (s, r) = bounded(100);
        Self {
            user_name: "guest".into(),
            path: std::env::current_dir().unwrap(),
            permission: Permission::Normal,
            signal: Signal::Waiting,
            cursor: 0,
            history: vec![],
            generator: signal_genertor(),
            bin_files: load_executable(),
            processing: None,
            sender: s,
            receiver: r
        }
    }

    // pub fn from(&self, user: Option<String>) -> Self {
        
    // }

    pub fn set_path(&mut self, path: PathBuf) -> () {
        self.path = path;
    }

    pub fn welcome_message(&self) {
        print!("{}", WELCOME);
    }

    fn write(&self, message: impl Into<String>) {
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

impl Input<'_> {
    fn waiting(&mut self) -> Option<Command> {
        let mut buf = vec![];
        let mut buf_temp = buf.clone();
        let mut history_position = self.history.len();

        let mut searching = false;
        let mut search_idx = 0;
        let mut search_buf = vec![];

        self.prompt_prefix(true);
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
                    'c' => self.write("^C\n".red().to_string()),
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

                    self.signal = Signal::Processing;
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
                        history_position = 0;
                    }

                    buf.insert(self.cursor, c);
                    self.cursor += 1;
                    searching = false;
                },
                KeyBoardSignal::Tab => {
                    if !searching {
                        searching = true;
                        search_buf = self.bin_files.iter().filter(|&pb|
                            pb.file_name()
                              .unwrap()
                              .to_str()
                              .unwrap()
                              .starts_with(&buf.iter().collect::<String>()))
                              .map(|pb| pb.file_name().unwrap().to_str().unwrap().to_string())
                              .collect();

                        search_buf.sort();
                    }

                    buf = search_buf[search_idx].chars().collect();
                    self.cursor = buf.len();
                    if search_idx != search_buf.len() - 1 {
                        search_idx += 1;
                    } else {
                        search_idx = 0;
                    }
                },
                _ => ()
            }

            self.prompt_prefix(true);
            self.write(Format::from(buf.iter().collect::<String>()).transform(None));
            if self.cursor != buf.len() {
                self.prompt_prefix(false);
                self.write(Format::from(buf.iter().collect::<String>()).transform(Some(self.cursor)));
            }
        }

        None
    }

    fn processing(&mut self) -> () {
        if let Some(mut child) = self.processing.take() {
            // let cin = child.stdin.take().unwrap();

            let (s, r) = bounded(1);
            spawn(move || {
                loop {
                    match r.recv_timeout(Duration::from_nanos(100)) {
                        Ok(_) => return,
                        _ => ()
                    }
                }
            });

            while let Ok(code) = child.try_wait() {
                match code {
                    Some(_code) => {
                        match s.send(()) {
                            _ => {
                                self.signal = Signal::Waiting;
                                return;
                            }
                        }
                    },
                    _ => ()
                }
            }

            self.signal = Signal::Waiting;
        }
    }

    fn interrupt(&mut self) -> () {
        
    }

    fn listening(&mut self) -> () {

    }
}

// impl KeyBoardSignalGenerator {
//     pub fn take_timeout(&mut self) -> anyhow::Result<KeyBoardSignal, RecvTimeoutError> {
//         match self.recv.recv_timeout(Duration::from_nanos(100)) {
//             Ok(KeyEvent {
//                 code,
//                 modifiers,
//                 ..
//             }) => Ok(match (code, modifiers) {
//                 (KeyCode::Backspace, _) => KeyBoardSignal::BackSpace,
//                 (KeyCode::Delete, _) => KeyBoardSignal::Delete,
//                 (KeyCode::Enter, _) => KeyBoardSignal::Enter,
//                 (KeyCode::Tab, _) => KeyBoardSignal::Tab,
//                 (KeyCode::Char(c), KeyModifiers::CONTROL) => KeyBoardSignal::Ctrl(c),
//                 (KeyCode::Char(c), KeyModifiers::NONE) => KeyBoardSignal::Insert(c),
//                 (KeyCode::Char(c), KeyModifiers::SHIFT) => KeyBoardSignal::Insert(c.to_ascii_uppercase()),
//                 (KeyCode::Up, _) => KeyBoardSignal::History(true),
//                 (KeyCode::Down, _) => KeyBoardSignal::History(false),
//                 (KeyCode::Left, _) => KeyBoardSignal::CursorMove(1),
//                 (KeyCode::Right, _) => KeyBoardSignal::CursorMove(2),
//                 (KeyCode::Home, _) => KeyBoardSignal::CursorMove(0),
//                 (KeyCode::End, _) => KeyBoardSignal::CursorMove(3),
//                 (KeyCode::F(x), _) => KeyBoardSignal::Fx(x.into()),
//                 _ => KeyBoardSignal::None
//             }),
//             Err(err) => Err(err)
//         }
//     }
// }