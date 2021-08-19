use crate::scheduler::Scheduler;
use crossbeam_deque::Injector;
use gol_lib::Field;
use std::sync::mpsc::RecvTimeoutError;
use std::sync::{mpsc, Arc, RwLock};

mod scheduler;

pub enum Task {
    Row(usize),
    Stop,
}

pub type Update = ((usize, usize), char);

pub struct Strategy {
    injector: Arc<Injector<Task>>,
    field: Arc<RwLock<Field>>,
    scheduler: Scheduler,
    worker_output: mpsc::Receiver<Vec<Update>>,
}

impl Strategy {
    pub fn new(field: Field) -> Self {
        let worker_input = Arc::new(Injector::<Task>::new());
        let field = Arc::new(RwLock::new(field));
        let (sender, receiver) = mpsc::sync_channel(1000);

        let scheduler = Scheduler::new(Arc::clone(&worker_input), Arc::clone(&field), sender);

        Strategy {
            injector: worker_input,
            field,
            scheduler,
            worker_output: receiver,
        }
    }
}

impl Iterator for Strategy {
    type Item = Field;

    fn next(&mut self) -> Option<Self::Item> {
        let mut field = match self.field.read() {
            Err(why) => {
                panic!("Failed to get read lock on field: {:?}", why);
            }
            Ok(lock) => Field::clone(&lock),
        };

        for row in 0..field.height() {
            self.injector.push(Task::Row(row));
        }

        let mut updated_any = false;
        let mut received_results_from = 0;
        while received_results_from < field.height() {
            let updates = match self
                .worker_output
                .recv_timeout(std::time::Duration::from_secs(1))
            {
                Err(RecvTimeoutError::Disconnected) => {
                    eprintln!("Channel closed abruptly");
                    Vec::new()
                }
                Err(RecvTimeoutError::Timeout) => {
                    eprintln!("Response took to long");
                    Vec::new()
                }
                Ok(result) => result,
            };
            received_results_from += 1;
            for (cords, value) in updates {
                *field.value_mut(cords) = value;
                updated_any = true;
            }
        }

        if !updated_any {
            return None;
        }

        match self.field.write() {
            Err(why) => {
                panic!("Failed to get write lock on field: {:?}", why);
            }
            Ok(mut lock) => *lock = field.clone(),
        };

        Some(field)
    }
}

impl Drop for Strategy {
    fn drop(&mut self) {
        for _ in 0..self.scheduler.worker_count() {
            self.injector.push(Task::Stop);
        }
    }
}
