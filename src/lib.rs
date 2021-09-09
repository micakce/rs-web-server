use std::{sync::{Arc, Mutex, mpsc::{self, Sender}}, thread::{self, JoinHandle}};

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: Sender<Job>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        // TODO implement error handling
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut threads = Vec::with_capacity(size);

        for id in 0..size {
            threads.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { threads, sender }

    }

    pub fn execute<F>(&self, f: F)
        where
        F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);
            self.sender.send(job).unwrap();
        }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a new job; precessing", id);

            job();
        });

        Worker { id, thread }
    }
}
