use std::sync::{mpsc, Arc, Mutex};
use std::thread as std_thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        debug_assert!(size > 0); // Redundant but oh well

        let (sender, receiver) = mpsc::channel(); // Initialize the message passing system
        let receiver = Arc::new(Mutex::new(receiver)); // This reciever will be passed as a paramter to all the workers (they
                                                       // must share a receiver)

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver))); // Send out some new workers
        }

        ThreadPool { workers, sender }
    }

    pub fn size(&self) -> usize {
        self.workers.len()
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
        println!("Instructing all workers to terminate...");
        // First send the Terminate message to all the workers
        for _ in 0..self.workers.len() {
            self.sender.send(Message::Terminate).unwrap()
        }
        // Then join all of the threads
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // `take()` replaces the thread with a None value
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<std_thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
    ) -> Worker {
        // Spawn a thread that loops, looking for messages
        let thread = std_thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("[Worker {}] Got a job. Executing...", id);
                    job.call_box();
                }
                Message::Terminate => {
                    println!(
                        "[Worker {}] Instructed to terminate. Breaking loop...",
                        id
                    );
                    break; // Breaks out of the loop to prevent endless blocking on
                           // thread join
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

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}
