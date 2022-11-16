pub mod message;
pub mod builtin;
pub mod process;

use std::path::PathBuf;

use colored::Colorize;
use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};
use regex::Regex;

use crate::stream::{output::Output, input::Input};

use self::message::WELCOME;

pub struct Commander {
    bufs: Vec<String>,
    path: PathBuf,
    output: Output,
    input: Input
}

#[derive(Debug, Clone)]
pub(super) struct Args {
    value: String
}

#[derive(Debug, Clone)]
pub(super) struct ArgsOption {
    key: String,
    value: Option<String>
}

#[derive(Debug, Clone)]
pub struct Command {
    pub(super) name: String,
    pub(super) args: Vec<Args>,
    pub(super) options: Vec<ArgsOption>,
    pub(super) long_options: Vec<ArgsOption>
}

impl Commander {
    pub fn new() -> Self {
        Self {
            bufs: vec![],
            path: std::env::current_dir().unwrap(),
            output: Output::new(),
            input: Input::new()
        }
    }

    pub fn welcome(&mut self) -> () {
        self.output.fwrite(WELCOME).unwrap();
    }
}

impl Iterator for Commander {
    type Item = Command;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf_idx = 0;
        let mut buf = String::new();
        let mut current;

        self.output.fwrite([String::from('\n'), command_line_prefix(self.path.clone(), "guest".into())].concat()).unwrap();
        for event in self.input.clone() {
            current = buf.clone();

            let buf = &mut buf;
            let (width, _height) = crossterm::terminal::size().unwrap();
            let KeyEvent {
                code,
                modifiers,
                ..
            } = event;

            if modifiers == KeyModifiers::CONTROL {
                match code {
                    KeyCode::Char(c) => {
                        match c {
                            'c' => {
                                self.output.write("\r").unwrap();
                                self.output.write(command_line_prefix(self.path.clone(), "guest".into())).unwrap();
                                self.output.fwrite("^C".red().to_string()).unwrap();
                                return Some(Command::new());
                            },
                            'd' => {
                                self.output.write("\r").unwrap();
                                self.output.write(command_line_prefix(self.path.clone(), "guest".into())).unwrap();
                                self.output.fwrite("exit".to_string()).unwrap();
                                return Some(Command::from("exit".into()));
                            },
                            _ => ()
                        }
                    },
                    _ => ()
                }
            } else {
                match code {
                    KeyCode::Char(c) => buf.push(c),
                    KeyCode::Backspace => { buf.pop(); },
                    KeyCode::Enter => {
                        self.bufs.push(buf.to_string());
                        return Some(Command::from(buf.to_string()))
                    },
                    KeyCode::Up => {
                        if buf_idx != self.bufs.len() {
                            buf_idx += 1;
                        }

                        *buf = if buf_idx != 0 {
                            self.bufs[self.bufs.len() - buf_idx].clone()
                        } else {
                            current.clone()
                        }
                    },
                    KeyCode::Down => {
                        if buf_idx != 0 {
                            buf_idx -= 1;
                        }

                        *buf = if buf_idx != 0 {
                            self.bufs[self.bufs.len() - buf_idx].clone()
                        } else {
                            current.clone()
                        }
                    },
                    _ => ()
                }
            }
            
            let prefix = command_line_prefix(self.path.clone(), "guest".into());
            let length = width as isize -
                self.path.clone().to_str().unwrap().len() as isize -
                "guest".len() as isize -
                10 as isize -
                buf.len() as isize;

            self.output.write("\r").unwrap();
            self.output.write(prefix).unwrap();
            self.output.write(if length < 0 {
                buf.clone().as_str()[0..(buf.len() as isize + length) as usize].to_string()
            } else {
                buf.clone()
            }).unwrap();
            self.output.fwrite(String::from(' ').repeat(if length < 0 { 0 } else { length as usize })).unwrap();
        }

        None
    }
}

impl Command {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            args: vec![],
            options: vec![],
            long_options: vec![]
        }
    }

    pub fn from(content: String) -> Self {
        let name = content.split_whitespace().nth(0).unwrap().to_string();
        let mut content = content.replacen(&name, "", 1);

        let long_options = Self::parse_long_options(&mut content);
        let options = Self::parse_options(&mut content);
        let args = Self::parse_args(&mut content);

        if name != "" {
            Self {
                name,
                args,
                options,
                long_options
            }
        } else {
            Self::new()
        }
    }

    fn parse_args(content: &mut String) -> Vec<Args> {
        let mut args = vec![];
        let re = Regex::new(r"([A-Za-z_][A-Za-z0-9_]*)").unwrap();
        
        for mtc in re.captures_iter(content.clone().as_str()) {
            args.push(Args::from(mtc[1].into()));

            let c = content.replacen(&mtc[0].to_string(), "", 1);
            content.clear();
            content.push_str(c.as_str());
        }

        args
    }

    fn parse_options(content: &mut String) -> Vec<ArgsOption> {
        let mut options = vec![];
        let re = Regex::new(r"[^-]-([A-Za-z_][A-Za-z0-9_]*)[= ]?([A-Za-z_][A-Za-z0-9_]*)?").unwrap();
        
        for mtc in re.captures_iter(content.clone().as_str()) {
            if mtc.len() == 2 {
                options.push(ArgsOption::from((mtc[1].into(), None)));
            } else {
                options.push(ArgsOption::from((mtc[1].into(), Some(mtc[2].into()))));
            }

            let c = content.replacen(&mtc[0].to_string(), "", 1);
            content.clear();
            content.push_str(c.as_str());
        }

        options
    }

    fn parse_long_options(content: &mut String) -> Vec<ArgsOption> {
        let mut long_options = vec![];
        let re = Regex::new(r"--([A-Za-z_][A-Za-z0-9_]*)[= ]?([A-Za-z_][A-Za-z0-9_]*)?").unwrap();

        for mtc in re.captures_iter(content.clone().as_str()) {
            if mtc.len() == 2 {
                long_options.push(ArgsOption::from((mtc[1].into(), None)));
            } else {
                long_options.push(ArgsOption::from((mtc[1].into(), Some(mtc[2].into()))));
            }

            let c = content.replacen(&mtc[0].to_string(), "", 1);
            content.clear();
            content.push_str(c.as_str());
        }

        long_options
    }
}

impl Args {
    pub fn from(value: String) -> Self {
        Self {
            value
        }
    }
}

impl ArgsOption {
    pub fn from(pair: (String, Option<String>)) -> Self {
        Self {
            key: pair.0,
            value: pair.1
        }
    }
}

fn command_line_prefix(path: PathBuf, user: String) -> String {
    format!(
        "{} {}({})> ",
        "(sks)".bold().bright_blue(),
        user.bold().red(),
        path.to_str().unwrap().bold()
    )
}