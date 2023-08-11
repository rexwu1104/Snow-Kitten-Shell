use std::{path::PathBuf, env::{set_current_dir, current_dir}};

use crate::data::Input;

use super::Command;

pub const SYSTEM_COMMANDS: [&str; 2] = [
    "exit",
    "cd"
];

pub fn exit(_: &mut Input) -> ! {
    std::process::exit(0);
}

pub fn cd(input: &mut Input, command: &Command) -> () {
    let temp_path = command.args.iter().nth(0);
    match temp_path {
        Some(path) => {
            set_current_dir(path).unwrap();
            input.set_path(current_dir().unwrap());
        },
        None => ()
    }
}