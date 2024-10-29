use std::{
    error::Error,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Condvar, Mutex,
    },
    time::{Duration, Instant},
};

use crossbeam::queue::SegQueue;

use super::{
    error::ThreadPoolError,
    worker::{Job, Worker},
};

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

    pub fn execute<F>(&self, job: F) -> Result<(), ThreadPoolError>
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

    pub fn shutdown(&mut self, timeout: Option<Duration>) -> Result<(), ThreadPoolError> {
        // Step 1: Signal all workers to stop
        self.job_running.store(false, Ordering::SeqCst);

        // Step 2: Wake up all waiting threads
        let (lock, cvar) = &*self.job_signal;
        if let Ok(mut job_available) = lock.try_lock() {
            *job_available = true;
            cvar.notify_all();
        } else {
            return Err(ThreadPoolError::LockAcquireFailure);
        }

        // Step 3: Track shutdown start time if a timeout is specified
        let start = Instant::now();

        // Step 4: Wait for each worker thread to finish
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                let should_timeout = timeout
                    .map(|duration| start.elapsed() >= duration)
                    .unwrap_or(false);

                if should_timeout {
                    return Err(ThreadPoolError::ShutdownTimeout);
                }

                if thread.join().is_err() {
                    return Err(ThreadPoolError::ThreadJoinError(format!(
                        "Worker {} failed to join",
                        worker.id
                    )));
                }
            }
        }

        // Step 5: Final timeout check, if a duration was provided
        if timeout.map_or(false, |duration| start.elapsed() > duration) {
            Err(ThreadPoolError::ShutdownTimeout)
        } else {
            Ok(())
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        if !self.workers.is_empty() {
            let _ = self
                .shutdown(Some(Duration::from_secs(2)))
                .map_err(|e| eprintln!("ThreadPool shutdown failed: {:?}", e));
        }
    }
}
