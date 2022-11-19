use crate::command::Command;

pub fn exit(object: Command) -> ! {
    let arg = object.args.get(0);
    if let Some(code) = arg {
        std::process::exit(code.value.parse().unwrap())
    } else {
        std::process::exit(0)
    }
}

pub fn ls(object: Command) -> () {

}