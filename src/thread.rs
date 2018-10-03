use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut threads = Vec::with_capacity(size);

        for id in 0..size {
            threads.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { threads, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: Send + FnOnce() + 'static,
    {
        let job = Box::new(f);

        self.sender
            .send(Message::NewJob(job))
            .expect("Error sending job");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.threads {
            println!("Shutting down worker {}", worker.id);

            worker.thread.unwrap().join().unwrap();
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job. Executing...", id);
                    job.call_box();
                }
                Message::Terminate => {
                    println!(
                        "Worker {} was instructed to terminate. Breaking loop...",
                        id
                    );
                    break; // Breaks out of the loop to prevent endless blocking on thread join
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

// BUG: in `dyn XYZ` syntax, `XYZ` is a trait, not a type; can't call it using `Self` or type parameter `F`
trait FnBox {
    fn call_box(self: Box<dyn Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<dyn F>) {
        (*self)()
    }
}
