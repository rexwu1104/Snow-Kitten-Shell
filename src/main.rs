mod data;
mod format;
mod command;
mod system;

fn main() {
    let input = data::Input::new();

    input.welcome_message();
    for _ in input {}
}