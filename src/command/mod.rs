use crate::data::KeyBoardSignalGenerator;

mod impls;
mod system;

#[derive(Debug, Clone)]
pub struct Command {
    name: String,
    args: Vec<String>,
    options: Vec<(String, String)>
}