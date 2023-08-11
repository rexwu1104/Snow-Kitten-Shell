use crate::{format::Format, data::Input};

use super::Command;

impl Command {
    pub fn from<'a>(format: &'a Format) -> Self {
        Self {
            name: format.get_name(),
            args: format.get_args(),
            options: format.get_options()
        }
    }

    #[cfg(target_family = "windows")]
    pub fn execute(&self, input: &mut Input) -> Option<std::process::Child> {
        use crate::command::system::cd;

        use super::system::exit;

        let name = self.name.clone();
        let args = self.args.clone();
        let options = self.options.clone();

        let mut command =
            std::process::Command::new("powershell");
        
        command
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .args(&["/C", name.as_str()]);

        for arg in args {
            command.arg(arg);
        }

        for option in options {
            command.args([option.0, option.1]);
        }

        let result = match command.spawn() {
            Ok(child) => {
                Some(child)
            },
            Err(err) => {
                println!("{}", err);

                None
            }
        };

        if let Some(name) = self.name.trim().split_whitespace().nth(0) {
            if super::system::SYSTEM_COMMANDS.contains(&name) {
                match name {
                    "exit" => exit(input),
                    "cd" => cd(input, self),
                    _ => ()
                }
            }
        }

        result
    }
}