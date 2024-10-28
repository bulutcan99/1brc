use std::{
    error::Error,
    sync::{atomic::AtomicBool, Arc, Condvar, Mutex},
};

use crossbeam::queue::SegQueue;

use super::worker::{Job, Worker};

pub struct ThreadPool {
    workers: Vec<Worker>,
    job_queue: Arc<SegQueue<Job>>,
    job_signal: Arc<(Mutex<bool>, Condvar)>,
    job_running: Arc<AtomicBool>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let job_queue = Arc::new(SegQueue::new());
        let job_signal = Arc::new((Mutex::new(false), Condvar::new()));
        let mut workers = Vec::with_capacity(size);
        let job_running = Arc::new(AtomicBool::new(true));
        for id in 0..size {
            let worker = Worker::new(
                id,
                Arc::clone(&job_queue),
                Arc::clone(&job_signal),
                Arc::clone(&job_running),
            );
            workers.push(worker);
        }
        ThreadPool {
            workers,
            job_queue,
            job_running,
            job_signal,
        }
    }

    pub fn execute<F>(&self, job: F) -> Result<(), Box<dyn Error>>
    where
        F: FnOnce() -> Result<(), Box<dyn std::error::Error>> + Send + 'static,
    {
        let task = Job::Task(Box::new(job));
        self.job_queue.push(task);
        let (mx, cvar) = &*self.job_signal;
        let mut job_available = mx.lock().unwrap();
        *job_available = true;
        cvar.notify_all();
        Ok(())
    }
}
