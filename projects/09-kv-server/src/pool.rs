//! A fixed-size thread pool. Jobs go into a channel; idle workers take them.
//! Like a bank with N counters and one queue: whoever is free calls "next".

use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    sender: Option<Sender<Job>>,
    workers: Vec<JoinHandle<()>>,
}

impl ThreadPool {
    /// Starts `size` workers. Panics if size is 0, a programming mistake.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "a pool needs at least one worker");
        let (sender, receiver) = mpsc::channel::<Job>();
        // One receiver, shared by all workers, so it lives in Arc<Mutex<..>>.
        let receiver = Arc::new(Mutex::new(receiver));

        let workers = (0..size)
            .map(|id| {
                let receiver = Arc::clone(&receiver);
                thread::Builder::new()
                    .name(format!("worker-{id}"))
                    .spawn(move || loop {
                        // Hold the lock only while taking a job, not while running it.
                        let next = receiver.lock().unwrap().recv();
                        match next {
                            Ok(job) => job(),
                            Err(_) => break, // channel closed: time to go home
                        }
                    })
                    .expect("could not start worker")
            })
            .collect();

        ThreadPool { sender: Some(sender), workers }
    }

    pub fn size(&self) -> usize {
        self.workers.len()
    }

    pub fn execute(&self, job: impl FnOnce() + Send + 'static) {
        self.sender.as_ref().expect("pool is shutting down").send(Box::new(job)).expect("workers are gone");
    }
}

impl Drop for ThreadPool {
    /// Graceful shutdown: close the channel, then wait for each worker to
    /// finish the job it is on.
    fn drop(&mut self) {
        drop(self.sender.take());
        for w in self.workers.drain(..) {
            let _ = w.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn runs_every_job_before_drop_returns() {
        let done = Arc::new(AtomicUsize::new(0));
        {
            let pool = ThreadPool::new(3);
            assert_eq!(pool.size(), 3);
            for _ in 0..20 {
                let done = Arc::clone(&done);
                pool.execute(move || {
                    done.fetch_add(1, Ordering::SeqCst);
                });
            }
        } // pool dropped here, which waits for all jobs
        assert_eq!(done.load(Ordering::SeqCst), 20);
    }
}
