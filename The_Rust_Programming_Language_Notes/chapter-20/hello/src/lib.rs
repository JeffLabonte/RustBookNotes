use std::thread;

pub struct ThreadPool{
    threads: Vec<thread::JoinHandle<()>>
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

        for _ in 0..size{
            threads.push(thread::Builder::new());
        }

        ThreadPool{
            threads
        }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static{
            
        }
}
