use std::net::TcpStream;
use std::io::Read;

use crate::mouse;
use crate::prase::{prase_data, Action, ClickBtn};

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
                println!("Handler Error occur: {}", e.to_string());
                continue;
            }
        };
        match res {
            Action::Click(side) => {
                match side {
                    ClickBtn::Left => mouse::click(
                        mouse::MOUSEEVENTF_LEFTDOWN |
                        mouse::MOUSEEVENTF_LEFTUP
                    ),
                    ClickBtn::Right => mouse::click(
                        mouse::MOUSEEVENTF_RIGHTDOWN|
                        mouse::MOUSEEVENTF_RIGHTUP
                    )
                };
            },
            Action::Move(m) => {
                mouse::move_mouse(m.dx as i32, m.dy as i32);
            }

        }
    }
    println!("Connection closed.");
}
