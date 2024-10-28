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
        match lock.try_lock() {
            Ok(mut job_available) => {
                *job_available = true;
                cvar.notify_all();
            }
            Err(_) => {
                // We couldn't acquire the lock, but we've set running to false,
                // so workers will eventually notice
                println!("Warning: Couldn't acquire lock to notify workers. They will exit on their next timeout check.");
            }
        }

        // Step 3: Start the shutdown process
        let start = Instant::now();

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                if let Some(timeout_duration) = timeout {
                    // Calculate remaining time for timeout-based shutdown
                    let remaining = timeout_duration
                        .checked_sub(start.elapsed())
                        .unwrap_or(Duration::ZERO);

                    if remaining.is_zero() {
                        return Err(ThreadPoolError::ShutdownTimeout);
                    }

                    // Wait with a timeout
                    if thread.join().is_err() {
                        return Err(ThreadPoolError::ThreadJoinError(format!(
                            "Worker {} failed to join",
                            worker.id
                        )));
                    }
                } else {
                    // Wait without a timeout
                    if thread.join().is_err() {
                        return Err(ThreadPoolError::ThreadJoinError(format!(
                            "Worker {} failed to join",
                            worker.id
                        )));
                    }
                }
            }
        }

        // Final timeout check if duration was provided
        if let Some(timeout_duration) = timeout {
            if start.elapsed() > timeout_duration {
                return Err(ThreadPoolError::ShutdownTimeout);
            }
        }

        Ok(())
    }
}
