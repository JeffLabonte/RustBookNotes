use std::thread;

pub struct ThreadPool{
    threads: Vec<Worker>
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

        for id in 0..size{
            threads.push(Worker::new(id));
        }

        ThreadPool{
            threads
        }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static{
 
        }
}

pub struct Worker{
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker{
    fn new(id: usize) -> Worker{
        let thread = thread::spawn(|| {});

        Worker{
            id,
            thread,
        }
    }
}
