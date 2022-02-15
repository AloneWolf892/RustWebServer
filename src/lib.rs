use std::{thread, sync::{mpsc, Arc, Mutex}};
use walkdir::WalkDir;

pub fn directory_list(root: String) -> Vec<String> {
    let mut file_structure: Vec<String> = Vec::new(); 
    let mut reduced_file_structure: Vec<String> = Vec::new();
    for entry in WalkDir::new(&root) {
        match entry {
            Ok(path) => file_structure.push(String::from(path.path().to_str().unwrap())),
            Err(err) => println!("{}", err)
        }
    }

    for entry in file_structure.iter_mut() {
        if entry.contains(".") {
            *entry = entry.replace("\\", "/");
            reduced_file_structure.push(String::from(entry.clone()));
        }
    }

    reduced_file_structure

}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap()
        }

        for worker in &mut self.workers {
            println!("Shutting down worker: {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop{
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                },
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        Worker { id, thread: Some(thread) }
    }
}