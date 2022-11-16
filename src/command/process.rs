use super::{Command, builtin::exit};

pub fn process_command(command: Command) {
    match command.name.as_str() {
        "exit" => exit(command),
        _ => ()
    }
}