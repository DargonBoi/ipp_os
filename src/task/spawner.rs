use core::future::Future;
use lazy_static::lazy_static;

use alloc::sync::Arc;
use crossbeam_queue::ArrayQueue;

use spin::Mutex;

use super::Task;

lazy_static! {
    pub static ref SPAWNER: Mutex<Spawner> = Mutex::new(Spawner::new(100));
}

#[derive(Clone)]
pub struct Spawner(Arc<ArrayQueue<Task>>);
impl Spawner {
    pub fn new(capacity: usize) -> Self {
        Self(Arc::new(ArrayQueue::new(capacity)))
    }
    pub fn add(&self, future: impl Future<Output = ()> + 'static) {
        let _ = self.0.push(Task::new(future));
    }
    pub fn pop(&self) -> Option<Task> {
        self.0.pop()
    }
}

unsafe impl Send for Spawner {}
