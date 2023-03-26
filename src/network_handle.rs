use std::net::TcpStream;
use std::io::Read;

use crate::mouse;
use crate::parse::{prase_data, Action, ClickBtn, Hold};

pub fn handle_stream(mut stream: TcpStream) {
    let mut buff:[u8; 1024] = [0; 1024];
    loop {
        let read_size = stream.read(&mut buff).unwrap();
        if read_size == 0 {
            break;
        }
        let data = std::str::from_utf8_mut(&mut buff[..read_size]).unwrap();
        let res = prase_data(data);
        let res = match res {
            Ok(a) => a,
            Err(e) => {
                println!("Handler Error occur: {}\nError data: {}", e.to_string(), data);
                continue;
            }
        };
        match res {
            Action::Click(side) => {
                match side {
                    ClickBtn::Left => mouse::mouse_key(
                        mouse::MOUSEEVENTF_LEFTDOWN |
                        mouse::MOUSEEVENTF_LEFTUP
                    ),
                    ClickBtn::Right => mouse::mouse_key(
                        mouse::MOUSEEVENTF_RIGHTDOWN|
                        mouse::MOUSEEVENTF_RIGHTUP
                    )
                };
            },
            Action::Move(m) => {
                mouse::move_mouse(m.dx as i32, m.dy as i32);
            },
            Action::Hold(h) => {
                match h {
                    Hold::Down => mouse::mouse_key(
                        mouse::MOUSEEVENTF_LEFTDOWN
                    ),
                    Hold::Up => mouse::mouse_key(
                        mouse::MOUSEEVENTF_LEFTUP
                    )
                };
            }
        }
    }
    println!("Connection closed.");
}
