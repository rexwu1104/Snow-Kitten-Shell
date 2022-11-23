use crate::format::Format;

use super::Command;

impl Command {
    pub fn from<'a>(format: &'a Format) -> Self {
        Self {
            name: format.get_name(),
            args: format.get_args(),
            options: format.get_options()
        }
    }
}