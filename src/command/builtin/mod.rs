use super::Command;

#[cfg(target_family = "windows")]
pub fn exit(object: Command) -> ! {
    let arg = object.args.get(0);
    if let Some(code) = arg {
        std::process::exit(code.value.parse().unwrap())
    } else {
        std::process::exit(0)
    }
}

#[cfg(target_family = "unix")]
pub fn exit(object: Command) -> ! {
    let arg = object.args.get(0);
    if let Some(code) = arg {
        std::process::exit(code.value.parse().unwrap())
    } else {
        std::process::exit(0)
    }
}

#[cfg(target_family = "windows")]
pub fn ls(object: Command) -> () {

}

#[cfg(target_family = "unix")]
pub fn ls(object: Command) -> () {
    
}