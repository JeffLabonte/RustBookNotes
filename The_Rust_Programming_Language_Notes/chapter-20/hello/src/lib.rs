use std::thread;
use std::sync::mpsc;

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
};

impl ThreadPool{
    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool
    ///
    /// # Panics
    ///
    /// The 'new' will panic if size  is 0
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError>{
        assert(size > 0);
        let mut threads = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        for id in 0..size{
            workers.push(Worker::new(id, receiver));
        }

        ThreadPool{
            workers,
            sender
        }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static{

        }
}

struct Job;

struct Worker{
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker{
    fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker{
        let thread = thread::spawn(|| {
            receiver;
        });

        Worker{
            id,
            thread,
        }
    }
}


