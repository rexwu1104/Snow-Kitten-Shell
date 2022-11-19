pub mod message;
pub mod builtin;
pub mod process;
mod impls;

use std::path::PathBuf;

use colored::Colorize;

use crate::stream::{output::Output, input::Input};

pub struct Commander {
    bufs: Vec<String>,
    path: PathBuf,
    output: Output,
    input: Input,
    user: String
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

fn command_line_prefix(path: PathBuf, user: String) -> String {
    format!(
        "{} {}({})> ",
        "(sks)".bold().bright_blue(),
        user.bold().red(),
        path.to_str().unwrap().bold()
    )
}

fn write_line(commander: &mut Commander, content: impl Into<String>) -> () {
    commander.output.write("\r").unwrap();
    commander.output.write(command_line_prefix(commander.path.clone(), commander.user.clone())).unwrap();
    commander.output.fwrite(content).unwrap();
}