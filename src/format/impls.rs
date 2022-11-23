use std::ops::Range;

use colored::Colorize;

use crate::command::Command;

use super::Format;

impl Format {
    #[inline]
    pub fn from(content: impl Into<String>) -> Self {
        let content: String = content.into();
        let content = content.as_str();
        let length = content.len();
        let mut idx = 0;

        let args_regex = regex::Regex::new(r#"[ ]?([^" ]+|"(?:[^\\"]|\\"| )*"?)"#).unwrap();
        let option_regex = regex::Regex::new(r#"(?:\B-|--)([^"-= ]+)(?:=+| +)?([^"\-= ]+|"(?:[^\\"]|\\"| )*")?"#).unwrap();

        let mut command = None;
        let mut args = vec![];
        let mut options = vec![];
        while idx != length {
            if idx == 0 {
                if let Some(mtc) = args_regex.find_at(&content, idx) {
                    command = Some((mtc.start(), mtc.end()));
                    idx = mtc.end();
                }
                
                continue;
            }

            if let Some(mtc) = option_regex.find_at(&content, idx) {
                let (start, end) = (mtc.start(), mtc.end());
                if start == idx + content[idx..].len() - content[idx..].trim().len() {
                    if mtc.as_str().contains("=") {
                        let fs = mtc.as_str().split("=").collect::<Vec<&str>>();
                        let f = fs[0];
                        
                        idx = end;
                        options.push([(start, start + f.len()), (start + f.len() + 1, end)]);
                    } else if mtc.as_str().contains(" ") {
                        let fs = mtc.as_str().split(" ").collect::<Vec<&str>>();
                        let f = fs[0];
                        
                        idx = end;
                        options.push([(start, start + f.len()), (start + f.len() + 1, end)]);
                    } else {
                        idx = end;
                        options.push([(start, end), (end, end)]);
                    }

                    continue;
                }
            }

            if let Some(mtc) = args_regex.find_at(&content, idx) {
                idx = mtc.end();
                args.push((mtc.start(), mtc.end()));
                
                continue;
            }

            idx += 1;
        }

        Self {
            raw: content.into(),
            command,
            args,
            options
        }
    }
}

impl Format {
    pub fn transform(&self, max: Option<usize>) -> String {
        let mut raw = self.raw.clone();
        let mut change_len = 0;
        if let Some(n) = max {
            raw.replace_range(n.., "");
        };

        if let Some(mut name) = self.command {
            if let Some(n) = max {
                if name.1 > n {
                    name.1 = n;
                }
            }

            let length = raw.len();
            raw.replace_range(
                name.0 + change_len..name.1 + change_len,
                &raw.as_str()[
                    name.0 + change_len ..
                    name.1 + change_len
                ].bright_green().to_string());
                
            change_len += raw.len() - length;
            if let Some(n) = max {
                if name.1 == n {
                    return raw;
                }
            }
        }

        let mut colors: Vec<(usize, usize, &str)> = vec![];
        for arg in &self.args {
            colors.push((arg.0, arg.1, "arg"));
        }

        for option in &self.options {
            colors.push((option[0].0, option[0].1, "key"));
            colors.push((option[1].0, option[1].1, "value"));
        }

        colors.sort_by(|ta, tb| ta.0.cmp(&tb.0));
        for color in colors {
            let mut color = color.clone();
            if let Some(n) = max {
                if color.1 > n {
                    color.1 = n;
                }
            }

            let length = raw.len();
            match color.2 {
                "arg" => {
                    raw.replace_range(
                        color.0 + change_len..color.1 + change_len,
                        &raw.as_str()[
                            color.0 + change_len ..
                            color.1 + change_len
                        ].bright_cyan().to_string());
                },
                "key" => {
                    raw.replace_range(
                        color.0 + change_len..color.1 + change_len,
                        &raw.as_str()[
                            color.0 + change_len ..
                            color.1 + change_len
                        ].bold().bright_yellow().to_string());
                },
                "value" => {
                    raw.replace_range(
                        color.0 + change_len..color.1 + change_len,
                        &raw.as_str()[
                            color.0 + change_len ..
                            color.1 + change_len
                        ].bright_red().to_string());
                },
                _ => ()
            }

            change_len += raw.len() - length;
            if let Some(n) = max {
                if color.1 == n {
                    return raw;
                }
            }
        }

        raw
    }
}

impl Format {
    pub fn to_command(&self) -> Command {
        Command::from(self)
    }

    pub fn get_args(&self) -> Vec<String> {
        self.args
            .iter()
            .map(|&(s, e)| *&self.raw.as_str()[s..e].trim())
            .map(String::from)
            .collect()
    }

    pub fn get_options(&self) -> Vec<(String, String)> {
        self.options
            .iter()
            .map(|[(ks, ke), (vs, ve)]| (
                *&self.raw.as_str()[*ks..*ke].trim(),
                *&self.raw.as_str()[*vs..*ve].trim()
            ))
            .map(|(k, v)| (String::from(k), String::from(v)))
            .collect()
    }

    pub fn get_name(&self) -> String {
        if let Some(command) = self.command {
            self.raw.as_str()[command.0..command.1].to_string()
        } else {
            String::new()
        }
    }
}

impl Format {
    fn len(&self) -> usize {
        self.raw.len()
    }
}