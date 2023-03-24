use test_mouse::thread_pool::ThreadPool;
use test_mouse::mouse::{self, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP};
use std::{thread, time::Duration};

fn main() {
    println!("Hello, world!");
    mouse::window();
    let pool = ThreadPool::new(5);
    
    for i in 0..4 {
        thread::sleep(Duration::from_millis(500));
        pool.execute(move || {
            println!("task: {}", i);
            mouse::move_mouse(0, -10);
            mouse::click(MOUSEEVENTF_LEFTDOWN);
            thread::sleep(Duration::from_millis(20));
            mouse::click(MOUSEEVENTF_LEFTUP);
        })
    }

    loop {

    }
}
