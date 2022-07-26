//! Implementation of [`TaskManager`]
//!
//! It is only used to manage processes and schedule process based on ready queue.
//! Other CPU process monitoring functions are in Processor.


use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::vec::Vec;
use alloc::sync::Arc;
use lazy_static::*;
use crate::config::BIG_STRIDE;

pub struct TaskManager {
    ready_queue: Vec<Arc<TaskControlBlock>>,
}

// YOUR JOB: FIFO->Stride
/// A simple FIFO scheduler.
impl TaskManager {
    pub fn new() -> Self {
        Self {
            ready_queue: Vec::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push(task);
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        let index = self.ready_queue.iter().enumerate().min_by(|x, y| {
            x.1.inner_exclusive_access().pass.cmp(&y.1.inner_exclusive_access().pass)
        }).unwrap().0;

        let task = self.ready_queue.remove(index);
        let mut task_inner = task.inner_exclusive_access();
        let pass = task_inner.pass;
        task_inner.pass += BIG_STRIDE / task_inner.prio as usize;
        if pass > task_inner.pass {
            panic!("pass overflow {} => {}", pass, task_inner.pass);
        }
        drop(task_inner);
        Some(task)
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

pub fn add_task(task: Arc<TaskControlBlock>) {
    TASK_MANAGER.exclusive_access().add(task);
}

pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    TASK_MANAGER.exclusive_access().fetch()
}
