use std::net::TcpListener;
use test_mouse::thread_pool::ThreadPool;
use test_mouse::network_handle::handle_stream;


fn main() {
    let listener = TcpListener::bind("0.0.0.0:1248").unwrap();
    let threads = ThreadPool::new(3);
    println!("Srever prepared.");
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                threads.execute(move || {
                    println!("Handle connection.");
                    handle_stream(s);
                });
            },
            Err(e) => {
                println!("Stream Error occur: {}", e.to_string());
            }
        }
    }
}
