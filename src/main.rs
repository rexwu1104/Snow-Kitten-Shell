mod data;
mod format;
mod command;

fn main() {
    // println!("{}", format::Format::from(r#""count numbers" args -cc=help --so "options or more" = "insert args" --鍵=值"#).transform());
    let input = data::Input::new();

    input.welcome_message();
    for c in input {
        println!("{:#?}", c);
    }
}