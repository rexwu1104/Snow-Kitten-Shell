use crate::data::KeyBoardSignalGenerator;

mod impls;

#[derive(Debug)]
pub struct Command {
    name: String,
    args: Vec<String>,
    options: Vec<(String, String)>
}