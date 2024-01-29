use std::{sync::{mpsc, Arc, Mutex}, thread, fs}; 
use std::net::TcpStream;
use std::collections::HashMap;
use std::time::Duration;
use std::io::prelude::*;

fn authenticate_user(username: &str, password: &str, user_credentials: &HashMap<String, String>) -> bool {
    if let Some(saved_password) = user_credentials.get(username) {
        return saved_password == password;
    }
    false
}

fn handle_connection(mut stream: TcpStream, user_credentials: &HashMap<String, String>) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer);
    let (username, password) = extract_user_and_password(&request);

    if authenticate_user(&username, &password, user_credentials) {
        let (status_line, filename) =
            if request.contains("GET / HTTP/1.1") {
                ("HTTP/1.1 200 OK", "index.html")
            } else if request.contains("GET /sleep HTTP/1.1") {
                thread::sleep(Duration::from_secs(5));
                ("HTTP/1.1 200 OK", "index.html")
            } else {
                ("HTTP/1.1 404 NOT FOUND", "404.html")
            };

        let contents = fs::read_to_string(filename).unwrap();
        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let response = "HTTP/1.1 401 UNAUTHORIZED\r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn extract_user_and_password(request: &str) -> (String, String) {
    let mut username = String::new();
    let mut password = String::new();

    for part in request.split('&') {
        if part.starts_with("user=") {
            username = part.trim_start_matches("user=").to_string();
        } else if part.starts_with("pass=") {
            password = part.trim_start_matches("pass=").to_string();
        }
    }

    (username, password)
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,


}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
     assert!(size > 0);

     let (sender , receiver) =
      mpsc::channel(); 

      let receiver=Arc::new(Mutex::new(receiver));

     let mut workers = Vec::with_capacity(size);
     
     for id in 0..size{
           workers.push(Worker::new(
            id, Arc::clone(&receiver)
        ));
     }

      ThreadPool{ workers, sender }   
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
            self.sender.send(Message::Terminate).unwrap();
          }

          println!("Shutting down all workers.");

        for worker in &mut self.workers{
            println!("Shutting down worker{}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
struct  Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
           let message  = receiver
              .lock()
              .unwrap()
              .recv()
              .unwrap();

            
              match message {
                Message::NewJob(job) =>{
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.",id);
                    break;
              }
            }
        });

        Worker{id, thread:Some(thread)}
    }
}