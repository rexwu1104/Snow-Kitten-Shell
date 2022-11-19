use colored::Colorize;
use crossterm::event::{KeyCode, KeyModifiers, KeyEvent};
use regex::Regex;

use crate::stream::{input::Input, output::Output};

use super::{ArgsOption, Args, Command, command_line_prefix, Commander, message::WELCOME, write_line};

impl Commander {
    pub fn new() -> Self {
        Self {
            bufs: vec![],
            path: std::env::current_dir().unwrap(),
            output: Output::new(),
            input: Input::new(),
            user: String::from("guest")
        }
    }

    pub fn welcome(&mut self) -> () {
        self.output.fwrite(WELCOME).unwrap();
    }
}

impl Iterator for Commander {
    type Item = Command;

    fn next(&mut self) -> Option<Self::Item> {
        let mut position: i32 = 0;
        let mut buf_idx = 0;
        let mut buf = String::new();
        let mut current;
        let mut temp: String = String::new();

        self.output.fwrite( command_line_prefix(self.path.clone(), "guest".into())).unwrap();
        for event in self.input.clone() {
            current = buf.clone();

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
                                write_line(self, "^C\n".red().to_string());
                                return Some(Command::new());
                            },
                            'd' => {
                                write_line(self, "exit");
                                return Some(Command::from("exit".into()));
                            },
                            _ => ()
                        }
                    },
                    _ => ()
                }
            } else {
                match code {
                    KeyCode::Char(c) => {
                        if position != 0 {
                            buf.insert((buf.len() as i32 + position) as usize, c);
                        } else {
                            buf.push(c);
                        }

                        temp = current;
                    },
                    KeyCode::Backspace => {
                        if -position == buf.len() as i32 {
                            continue;
                        } else if position == 0 {
                            buf.pop();
                        } else {
                            buf.remove((buf.len() as i32 + position - 1) as usize);
                            if -position > buf.len() as i32 {
                                position += 1;
                            }
                        }
                    },
                    KeyCode::Enter => {
                        if buf_idx == 0 {
                            self.bufs.push(buf.to_string());
                        }

                        self.output.fwrite("\n").unwrap();
                        return Some(Command::from(buf.to_string()))
                    },
                    KeyCode::Up => {
                        position = 0;
                        if buf_idx != self.bufs.len() {
                            buf_idx += 1;
                        }

                        buf = if buf_idx != 0 {
                            self.bufs[self.bufs.len() - buf_idx].clone()
                        } else {
                            temp.clone()
                        }
                    },
                    KeyCode::Down => {
                        position = 0;
                        if buf_idx != 0 {
                            buf_idx -= 1;
                        }

                        buf = if buf_idx != 0 {
                            self.bufs[self.bufs.len() - buf_idx].clone()
                        } else {
                            temp.clone()
                        }
                    },
                    KeyCode::Left => {
                        if position != -(buf.len() as i32) {
                            position -= 1;
                        }
                    },
                    KeyCode::Right => {
                        if position != 0 {
                            position += 1;
                        }
                    },
                    _ => ()
                }
            }
            
            self.output.write("\r").unwrap();
            self.output.fwrite(String::from(" ").repeat(width.into())).unwrap();

            write_line(self, buf.clone());
            let content = {
                if position == -(buf.len() as i32) {
                    String::new()
                } else {
                    buf.as_str()[0..(buf.len() as i32 + position) as usize].to_string()
                }
            };

            write_line(self, content);
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
        if content.trim() == "" {
            return Self::new();
        }

        let name = content.trim().split_whitespace().nth(0).unwrap().to_string();
        let mut content = content.trim().replacen(&name, "", 1);

        let long_options = Self::parse_long_options(&mut content);
        let options = Self::parse_options(&mut content);
        let args = Self::parse_args(&mut content);

        Self {
            name,
            args,
            options,
            long_options
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