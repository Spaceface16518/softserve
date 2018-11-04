use std::sync::{mpsc, Arc, Mutex};
use std::thread as std_thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Initializes a ThreadPool. A thread pool will spawn a specified amount
    /// of threads, called `Worker`s, and will have these workers execute
    /// `Job`s. Message passing is achieved using a mpsc channel, of which the
    /// sender is owned by the ThreadPool and the receiver is shared by all of
    /// the Workers. The workers are stored in a vector which is declared with
    /// an initial capacity for efficiency. Access to the sender is lost once
    /// this function is done executing; because of this, the ThreadPool is
    /// limited in that it cannot grow the amount of Workers it owns.
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

    /// Gets the size of the `ThreadPool`. Because the size of a `ThreadPool`
    /// cannot be changed, its size accurately represents the amount of
    /// threads/`Worker`s it has.
    pub fn size(&self) -> usize {
        self.workers.len()
    }

    /// Tells the `ThreadPool`'s `Worker`s to execute a given closure. This
    /// closure must implement `Send` because the closure will be sent as a
    /// message by the `ThreadPool` to the `Workers`. It is first boxed and then
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
            self.sender
                .send(Message::Terminate)
                .expect("Error sending message");
        }

        // Then join all of the threads
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // `take()` replaces the thread with a None value
            if let Some(thread) = worker.thread.take() {
                thread.join().expect(
                    format!("Could not join thread {}", worker.id()).as_str(),
                );
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
            let message = receiver
                .lock()
                .expect("Could not lock receiver")
                .recv()
                .expect("Error receiving message");
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

    pub fn id(&self) -> usize {
        self.id
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
