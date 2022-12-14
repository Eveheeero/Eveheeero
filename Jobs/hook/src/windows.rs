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

unsafe extern "system" fn callback(code: i32, wparam: WPARAM, laparam: LPARAM) -> LRESULT {
    if code == HC_ACTION.try_into().unwrap() {
        for now in std::iter::once(0x20)
            .chain(std::iter::once(0x0D))
            .chain(0x30..0x39)
            .chain(0x41..0x5A)
        {
            let space_status = GetAsyncKeyState(now);
            if space_status.leading_ones() == 0 && space_status.trailing_zeros() == 0 {
                // 0x8000
                println!("{} is pressed", now);
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

        let mut msg = MSG::default();
        while GetMessageA(&mut msg, HWND(0), 0, 0) == BOOL(1) {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }

        UnhookWindowsHookEx(handle);
    }
    Ok(())
}
