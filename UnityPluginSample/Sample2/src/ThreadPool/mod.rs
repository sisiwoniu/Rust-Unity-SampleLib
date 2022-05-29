use std::{
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex,
    },
    thread,
};

#[allow(dead_code)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
    receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
    taskCount: Arc<Mutex<i32>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let taskCount = Arc::new(Mutex::new(0));

        for id in 0..size {
            //スレッド作成
            workers.push(Worker::new(
                id,
                Arc::clone(&receiver),
                Arc::clone(&taskCount),
            ));
        }

        ThreadPool {
            workers,
            sender,
            receiver,
            taskCount,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        *self.taskCount.lock().unwrap() += 1;
        self.sender.send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        loop {
            if *self.taskCount.lock().unwrap() <= 0 {
                break;
            }
        }
    }
}

#[allow(dead_code)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
    // state: Arc<Mutex<WorkerState>>,
}

// #[derive(PartialEq, Copy, Clone)]
// enum WorkerState {
//     Idle,
//     Working,
// }

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>, taskCount: Arc<Mutex<i32>>) -> Worker {
        // let stateArc = Arc::new(Mutex::new(WorkerState::Idle));

        // let stateArcClone = Arc::clone(&stateArc);
        let thread = thread::spawn(move || loop {
            // *stateArcClone.lock().unwrap() = WorkerState::Idle;
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job;", id);

            // *stateArcClone.lock().unwrap() = WorkerState::Working;
            job();

            *taskCount.lock().unwrap() -= 1;
        });

        Worker {
            id,
            thread: Some(thread),
            // state: stateArc,
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
