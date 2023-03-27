use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{
        mpsc::{Receiver, Sender},
        Arc, Mutex,
    },
};

use bincode::deserialize;
use rpc_data::{DataType, MethodCall};

struct Worker {
    id: u32,
    thread: std::thread::JoinHandle<()>,
}
pub struct MathService {
    listener: TcpListener,
    workers: Vec<Worker>,
    sender: Sender<TcpStream>,
}

impl Worker {
    fn new(id: u32, receiver: Arc<Mutex<Receiver<TcpStream>>>) -> Self {
        let thread = std::thread::spawn(move || {
            println!("Worker {} is running", id);
            loop {
                let mut stream = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} gets a connection", id);
                loop {
                    let mut buf = [0; 1024];
                    let size = stream.read(&mut buf).unwrap();
                    if size == 0 {
                        println!("Worker {} got a disconnect", id);
                        break;
                    }
                    println!("Worker {} got a job; executing.", id);
                    let call: MethodCall = deserialize(&buf[..size]).unwrap();
                    assert!(call.params.len() == 2);
                    let result = match call.method {
                        rpc_data::MathOp::Add => MathService::add(&call.params[0], &call.params[1]),
                        rpc_data::MathOp::Sub => MathService::sub(&call.params[0], &call.params[1]),
                        rpc_data::MathOp::Mul => MathService::mul(&call.params[0], &call.params[1]),
                        rpc_data::MathOp::Div => MathService::div(&call.params[0], &call.params[1]),
                    };
                    println!("Worker {} finished job; sending result {:#?}.", id, result);
                    let result = bincode::serialize(&result).unwrap();
                    stream.write(&result).unwrap();
                }
            }
        });
        Self { id, thread }
    }
}

impl MathService {
    pub fn new(addr: &str) -> Self {
        let listener = TcpListener::bind(addr).unwrap();
        let (sender, receiver) = std::sync::mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = vec![];

        for i in 0..10 {
            let worker = Worker::new(i, Arc::clone(&receiver));
            workers.push(worker);
        }

        Self {
            listener,
            workers,
            sender,
        }
    }
    pub fn run(&self) {
        println!("Server is running on {}", self.listener.local_addr().unwrap());
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            println!("Server received a connection");
            self.sender.send(stream).unwrap();
        }
    }

    fn add(a: &DataType, b: &DataType) -> DataType {
        match (a, b) {
            (DataType::Integer(a), DataType::Integer(b)) => DataType::Integer(a + b),
            _ => DataType::Boolean(false),
        }
    }
    fn sub(a: &DataType, b: &DataType) -> DataType {
        match (a, b) {
            (DataType::Integer(a), DataType::Integer(b)) => DataType::Integer(a - b),
            _ => DataType::Boolean(false),
        }
    }
    fn mul(a: &DataType, b: &DataType) -> DataType {
        match (a, b) {
            (DataType::Integer(a), DataType::Integer(b)) => DataType::Integer(a * b),
            _ => DataType::Boolean(false),
        }
    }
    fn div(a: &DataType, b: &DataType) -> DataType {
        match (a, b) {
            (DataType::Integer(a), DataType::Integer(b)) => DataType::Integer(a / b),
            _ => DataType::Boolean(false),
        }
    }
}
