use crate::{Strategy, Task, Update};
use crossbeam_deque::Injector;
use gol_lib::Field;
use std::sync::{mpsc, Arc, RwLock};
use std::thread::{sleep, JoinHandle};
use std::time::Duration;

pub struct Worker {
    id: usize,
    global_queue: Arc<Injector<Task>>,
    field: Arc<RwLock<Field>>,
    output: mpsc::SyncSender<Vec<Update>>,
}

impl Worker {
    pub fn new(
        id: usize,
        injector: Arc<Injector<Task>>,
        field: Arc<RwLock<Field>>,
        output: mpsc::SyncSender<Vec<Update>>,
    ) -> Self {
        Worker {
            id,
            global_queue: injector,
            field,
            output,
        }
    }

    fn find_task(&self) -> Option<Task> {
        std::iter::repeat_with(|| self.global_queue.steal())
            // Loop while no task was stolen and any steal operation needs to be retried.
            .find(|s| !s.is_retry())
            // Extract the stolen task, if there is one.
            .and_then(|s| s.success())
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
                None => sleep(Duration::from_millis(10)),
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
        let mut num_cpus = num_cpus::get();

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
