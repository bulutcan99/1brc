use std::thread;

pub struct Worker {
    pub id: u16,
    pub thread: Option<thread::JoinHandle<()>>,
}
