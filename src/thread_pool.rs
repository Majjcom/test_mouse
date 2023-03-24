use std::{thread, thread::JoinHandle};
use std::sync::{Mutex, Arc, mpsc::{Receiver, Sender, channel}};
use std::boxed::Box;

struct Worker {
    thread: Option<JoinHandle<()>>,
    id: usize
}

impl Worker {
    pub fn new(id: usize, revecer: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = revecer.lock().unwrap().recv().unwrap();
            match message {
                Message::Message(job) => {
                    job();
                },
                Message::End => {
                    break;
                }
            }
        });
        let thread = Some(thread);
        Worker {
            thread,
            id
        }
    }
}

type Job = Box<dyn FnOnce() + 'static + Send>;

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

enum Message {
    Message(Job),
    End
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    n: usize,
    sender: Sender<Message>
}

impl ThreadPool {
    pub fn new(n: usize) -> ThreadPool {
        let mut pool = Vec::new();
        let (sender, recever) = channel::<Message>();
        let recever = Arc::new(Mutex::new(recever));
        for id in 0..n {
            let worker = Worker::new(id, recever.clone());
            pool.push(worker);
        }
        
        ThreadPool{
            workers: pool,
            n,
            sender
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + 'static + Send
    {
        let msg = Message::Message(Box::new(f));
        self.sender.send(msg).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in 0..self.n {
            self.sender.send(Message::End).unwrap();
        }
        for i in self.workers.iter_mut() {
            if let Some(t) = i.thread.take() {
                t.join().unwrap();
                println!("thread {} down.", i.id);
            }
        }
    }
}
