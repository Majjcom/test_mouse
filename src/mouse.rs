use winapi::um::winuser::{mouse_event, GetCursorPos, MOUSEEVENTF_MOVE, MOUSEEVENTF_ABSOLUTE};
// use winapi::um::winuser::keybd_event;
use winapi::shared::windef::POINT;

use winapi::shared::minwindef::DWORD;
pub use winapi::um::winuser::{MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP};

pub fn click(method: DWORD) {
    unsafe {
        mouse_event(method, 0, 0, 0, 0);
    };
}

pub fn move_mouse(dx: i32, dy: i32) {
    unsafe {
        let mut point = POINT{ x: 0, y: 0 };
        GetCursorPos(&mut point);
        let x = point.x + dx;
        let y = point.y + dy;
        mouse_event(MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE, x as u32, y as u32, 0, 0);
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
