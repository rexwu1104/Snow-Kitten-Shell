pub mod windows;
pub mod unix;

#[cfg(target_family = "windows")]
pub use self::windows as command;

#[cfg(target_family = "unix")]
pub use self::unix as command;
