use command::{Commander, process::process_command};

mod stream;
mod command;
mod signal;

fn main() -> anyhow::Result<()> {
    let mut commander = Commander::new();
    commander.welcome();

    for command in commander {
        process_command(command)
    }

    Ok(())
}