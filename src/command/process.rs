use super::{Command, builtin::command::exit};

pub fn process_command(command: Command) {
    match command.name.as_str() {
        "exit" => exit(command),
        "" => (),
        other => println!("{other}: command not found")
    }
}