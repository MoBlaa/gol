use crate::{Strategy, Task, Update};
use crossbeam_deque::{Injector, Stealer};
use gol_lib::Field;
use std::sync::{mpsc, Arc, RwLock};
use std::thread::JoinHandle;

pub struct Worker {
    id: usize,
    global_queue: Arc<Injector<Task>>,
    worker: crossbeam_deque::Worker<Task>,
    stealers: Vec<Stealer<Task>>,
    field: Arc<RwLock<Field>>,
    output: mpsc::SyncSender<Vec<Update>>,
}

impl Worker {
    pub fn new(
        id: usize,
        injector: Arc<Injector<Task>>,
        stealers: Vec<Stealer<Task>>,
        field: Arc<RwLock<Field>>,
        output: mpsc::SyncSender<Vec<Update>>,
    ) -> Self {
        Worker {
            id,
            global_queue: injector,
            worker: crossbeam_deque::Worker::new_fifo(),
            stealers,
            field,
            output,
        }
    }

    fn find_task(&self) -> Option<Task> {
        // Pop a task from the local queue, if not empty.
        self.worker.pop().or_else(|| {
            // Otherwise, we need to look for a task elsewhere.
            std::iter::repeat_with(|| {
                // Try stealing a batch of tasks from the global queue.
                self.global_queue
                    .steal_batch_and_pop(&self.worker)
                    // Or try stealing a task from one of the other threads.
                    .or_else(|| self.stealers.iter().map(|s| s.steal()).collect())
            })
            // Loop while no task was stolen and any steal operation needs to be retried.
            .find(|s| !s.is_retry())
            // Extract the stolen task, if there is one.
            .and_then(|s| s.success())
        })
    }

    pub fn start(&self) {
        loop {
            match self.find_task() {
                Some(Task::Row(index)) => {
                    let field = match self.field.read() {
                        Err(why) => {
                            eprintln!(
                                "Worker #{} failed to get read lock on field: {:?}",
                                self.id, why
                            );
                            break;
                        }
                        Ok(lock) => lock,
                    };
                    let updates = Strategy::advance_row(index, &field);
                    if let Err(why) = self.output.send(updates) {
                        eprintln!("Worker #{} failed to send updates: {:?}", self.id, why);
                        break;
                    }
                }
                Some(Task::Stop) => break,
                None => (),
            }
        }
    }
}

pub struct Scheduler {
    handles: Vec<JoinHandle<()>>,
}

impl Scheduler {
    pub fn new(
        injector: Arc<Injector<Task>>,
        field: Arc<RwLock<Field>>,
        sender: mpsc::SyncSender<Vec<Update>>,
    ) -> Self {
        let mut num_cpus = if cfg!(test) { 1 } else { num_cpus::get() };

        if num_cpus > 1 {
            num_cpus -= 1;
        }

        Self::workers(injector, field, sender, num_cpus)
    }

    pub fn worker_count(&self) -> usize {
        self.handles.len()
    }

    pub fn workers(
        injector: Arc<Injector<Task>>,
        field: Arc<RwLock<Field>>,
        sender: mpsc::SyncSender<Vec<Update>>,
        count: usize,
    ) -> Self {
        let mut workers_deques = Vec::with_capacity(count);
        for _ in 0..count {
            workers_deques.push(crossbeam_deque::Worker::<Task>::new_fifo());
        }
        let mut workers = Vec::with_capacity(count);
        for id in 0..count {
            let mut stealers = Vec::with_capacity(count - 1);
            for worker_deque in &workers_deques {
                stealers.push(worker_deque.stealer());
            }
            workers.push(Worker::new(
                id,
                Arc::clone(&injector),
                stealers,
                Arc::clone(&field),
                sender.clone(),
            ));
        }

        let handles = workers
            .into_iter()
            .map(|worker| {
                std::thread::spawn(move || {
                    worker.start();
                })
            })
            .collect::<Vec<_>>();

        Self { handles }
    }
}

impl Drop for Scheduler {
    fn drop(&mut self) {
        for handle in self.handles.pop() {
            if let Err(why) = handle.join() {
                eprintln!("Failed to join worker thread: {:?}", why);
            }
        }
    }
}
