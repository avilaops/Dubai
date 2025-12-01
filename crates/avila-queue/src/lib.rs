// AvilaQueue - Native Job Queue
// Zero External Dependencies 🦀

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Job {
    pub id: String,
    pub data: Vec<u8>,
    pub retry_count: u32,
    pub max_retries: u32,
}

pub struct Queue {
    jobs: Arc<Mutex<VecDeque<Job>>>,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            jobs: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn push(&self, job: Job) {
        let mut jobs = self.jobs.lock().unwrap();
        jobs.push_back(job);
    }

    pub fn pop(&self) -> Option<Job> {
        let mut jobs = self.jobs.lock().unwrap();
        jobs.pop_front()
    }

    pub fn len(&self) -> usize {
        self.jobs.lock().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.jobs.lock().unwrap().is_empty()
    }
}

impl Default for Queue {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let queue = Queue::new();
        queue.push(Job {
            id: "1".to_string(),
            data: vec![],
            retry_count: 0,
            max_retries: 3,
        });
        assert_eq!(queue.len(), 1);
        assert!(queue.pop().is_some());
    }
}
