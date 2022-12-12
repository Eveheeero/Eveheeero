use windows::Win32::{
    Foundation::{BOOL, HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{
        CallNextHookEx, DispatchMessageA, GetMessageA, SetWindowsHookExW, TranslateMessage,
        UnhookWindowsHookEx, HOOKPROC, MSG, WH_KEYBOARD_LL,
    },
};

unsafe extern "system" fn callback(code: i32, wparam: WPARAM, laparam: LPARAM) -> LRESULT {
    println!(
        "code: {}, wparam: {}, lparam: {}",
        code, wparam.0, laparam.0
    );
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
