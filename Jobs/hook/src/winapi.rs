use winapi::um::winuser::{
    DispatchMessageA, GetMessageA, SetWindowsHookExW, TranslateMessage, UnhookWindowsHookEx, MSG,
    WH_KEYBOARD_LL,
};

unsafe extern "system" fn callback(code: i32, wparam: usize, lparam: isize) -> isize {
    println!("nCode: {}, wParam: {}, lParam: {}", code, wparam, lparam);
    0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handle;
    unsafe {
        // 로우레벨(LL)이 아니면 핸들링이 안됨, 등록은 됨
        // MOUSE는 움직이는것마다 핸들링이 걸려 오류발생
        handle = SetWindowsHookExW(WH_KEYBOARD_LL, Some(callback), std::ptr::null_mut(), 0);

        // input으로하면 핸들링이 안됨 해당방법을 사용해야함
        let mut msg: MSG = MSG {
            hwnd: std::ptr::null_mut(),
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: std::mem::zeroed(),
        };
        while GetMessageA(&mut msg as *mut MSG, std::ptr::null_mut(), 0, 0) == 1 {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }

        UnhookWindowsHookEx(handle);
    }
    Ok(())
}
