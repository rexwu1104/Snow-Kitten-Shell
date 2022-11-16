use crossbeam_channel::{Receiver, Sender};
use crossterm::event::{KeyEvent, KeyCode, KeyModifiers, KeyEventKind, KeyEventState};
use winapi::um::consoleapi::SetConsoleCtrlHandler;

#[cfg(target_family = "windows")]
pub fn signal_handle() -> Receiver<KeyEvent> {
    use std::thread::spawn;
    use crossbeam_channel::{bounded, SendError};
    use crossterm::event::{read, Event};

    let (sender, recevier) = bounded::<KeyEvent>(100);
    ignore_ctrlc(&sender);
    spawn(move || {
        while let Ok(event) = read() {
            match event {
                Event::Key(key) => {
                    let result = sender.send(key);
                    if let Ok(_) = result {

                    } else {
                        if let Err(err) = result {
                            let SendError(err) = err;
                            println!("{:#?}", err);
                        }
                    }
                },
                _ => ()
            }
        }
    });

    recevier
}

#[cfg(target_family = "unix")]
pub fn signal_handle() {

}

fn ignore_ctrlc(sender: &Sender<KeyEvent>) -> i32 {
    unsafe {
        SENDER = Some(sender.clone())
    }

    unsafe { SetConsoleCtrlHandler(Some(ctrlc_handle), 1) }
}

static mut SENDER: Option<Sender<KeyEvent>> = None;
unsafe extern "system" fn ctrlc_handle(_signal: u32) -> i32 {
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