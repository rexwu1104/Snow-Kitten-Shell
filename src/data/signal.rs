use crossbeam_channel::{bounded, Sender};
use crossterm::event::{KeyEvent, read};

use super::KeyBoardSignalGenerator;

#[cfg(target_family = "windows")]
static mut SENDER: Option<Sender<KeyEvent>> = None;

#[cfg(target_family = "windows")]
pub(super) fn signal_genertor() -> KeyBoardSignalGenerator {
    use std::thread::spawn;

    use crossterm::event::{Event, KeyCode, KeyModifiers, KeyEventKind, KeyEventState};
    use winapi::um::consoleapi::SetConsoleCtrlHandler;

    let (sender, receviver) = bounded::<KeyEvent>(100);
    
    unsafe extern "system" fn ctrlc(_: u32) -> i32 {
        if let Some(sender) = &SENDER {
            sender.send(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            }).unwrap();
        }

        1
    }

    unsafe {
        SENDER = Some(sender.clone());
        SetConsoleCtrlHandler(Some(ctrlc), 1);
    }

    spawn(move || {
        while let Ok(event) = read() {
            match event {
                Event::Key(key) => sender.send(key).unwrap(),
                _ => ()
            }
        }
    });

    KeyBoardSignalGenerator {
        recv: receviver
    }
}
