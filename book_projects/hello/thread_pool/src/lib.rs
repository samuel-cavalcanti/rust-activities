use std::thread;

use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver};


pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }


    pub fn execute<F>(&self, function: F)
        where F: FnOnce() + Send + 'static, {
        let job = Box::new(function);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        Worker {
            id,
            handle: Some(thread::spawn(move || loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job executing", id);
                        job()
                    }
                    Message::Terminate => {
                        println!("Worker {} was told to terminate", id);
                        break;
                    }
                };
            })),
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }


        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(handle) = worker.handle.take() {
                handle.join().unwrap()
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn panic_with_size_igual_to_zero() {
        let size = 0;
        let pool = ThreadPool::new(size);
    }

    #[test]
    fn execute_many_works() {
        let pool_size = 2;
        let pool = ThreadPool::new(pool_size);

        for _ in 0..pool_size * 10 {
            pool.execute(|| {
                assert_eq!(2, 2);
            });
        }
    }
}
