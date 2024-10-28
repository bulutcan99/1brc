use std::{
    error::Error,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Condvar, Mutex,
    },
    thread,
    time::Duration,
};

use crossbeam::queue::SegQueue;

pub enum Job {
    Task(Box<dyn FnOnce() -> Result<(), Box<dyn Error>> + Send + 'static>),
    Shutdown,
}

pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(
        id: usize,
        job_queue: Arc<SegQueue<Job>>,
        job_signal: Arc<(Mutex<bool>, Condvar)>,
        job_running: Arc<AtomicBool>,
    ) -> Self {
        let thread = thread::spawn(move || loop {
            match job_queue.pop() {
                Some(Job::Task(task)) => if task().is_err() {},
                Some(Job::Shutdown) => {
                    break;
                }
                None => {
                    let (mx, cvar) = &*job_signal;
                    let mut job_available = mx.lock().unwrap();
                    while !*job_available && job_running.load(Ordering::Relaxed) {
                        let result = cvar
                            .wait_timeout(job_available, Duration::from_millis(100))
                            .unwrap();
                        job_available = result.0;
                    }
                    *job_available = false;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
