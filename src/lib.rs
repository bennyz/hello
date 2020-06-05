use std::fmt;
use std::thread;
use std::sync::{Arc, mpsc, Mutex};

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size <= 0 {
            return Err(PoolCreationError)
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
    
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool{ workers, sender })
    }

    pub fn execute<F>(&self, f: F) 
    where
        // FnOnce because the thread only execute the code block we pass once
        // Send is required to pass the closures between threads
        // 'static because we do not now how long the thread will take to execute
        // so the closure has to leave as long as the program
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move||{
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("executing job with worker {}", id);
                job();
            }
        });
        Worker { id, thread }
    }
}

#[derive(Debug)]
pub struct PoolCreationError;

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pool size cannot be <= 0")
    }
}
