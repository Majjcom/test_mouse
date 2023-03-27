use winapi::um::winuser::{
    mouse_event, MOUSEEVENTF_MOVE,
    MOUSEEVENTF_WHEEL, WHEEL_DELTA
};

use winapi::shared::minwindef::DWORD;
pub use winapi::um::winuser::{
    MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP
};

pub fn mouse_key(method: DWORD) {
    unsafe {
        mouse_event(method, 0, 0, 0, 0);
    };
}

pub fn mouse_wheel(down: bool) {
    unsafe {
        let mut m = WHEEL_DELTA;
        if down {
            m = -m;
        }
        mouse_event(MOUSEEVENTF_WHEEL, 0, 0, m as u32, 0);
    }
}

pub fn move_mouse(dx: i32, dy: i32) {
    unsafe {
        // let mut point = POINT{ x: 0, y: 0 };
        // GetCursorPos(&mut point);
        // let screen_x = GetSystemMetrics(SM_CXSCREEN) as f32;
        // let screen_y = GetSystemMetrics(SM_CYSCREEN) as f32;
        // let x = (point.x as f32 + dx as f32) * 65536f32 / screen_x;
        // let y = (point.y as f32 + dy as f32) * 65537f32 / screen_y;
        // mouse_event(MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE, x as u32, y as u32, 0, 0);
        mouse_event(MOUSEEVENTF_MOVE, dx as u32, dy as u32, 0, 0);
    };
}

pub fn window() {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::um::winuser::{MB_OK, MessageBoxW};
    let wide: Vec<u16> = OsStr::new("Hello").encode_wide().chain(once(0)).collect();
    unsafe {
        MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK);
    };
}
