use bincode::{deserialize, serialize};
use rpc_data::{DataType, MathOp, MethodCall};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub struct MathService {
    stream: TcpStream,
}

impl MathService {
    pub fn new(addr: &str) -> MathService {
        let stream = TcpStream::connect(addr).unwrap();
        MathService { stream }
    }
    pub fn add(&mut self, a: i32, b: i32) -> i32 {
        let call = MethodCall::new(
            MathOp::Add,
            vec![DataType::Integer(a), DataType::Integer(b)],
        );
        let result = self.call(call);
        match result {
            DataType::Integer(i) => i,
            _ => panic!("Invalid return type"),
        }
    }
    pub fn sub(&mut self, a: i32, b: i32) -> i32 {
        let call = MethodCall::new(
            MathOp::Sub,
            vec![DataType::Integer(a), DataType::Integer(b)],
        );
        let result = self.call(call);
        match result {
            DataType::Integer(i) => i,
            _ => panic!("Invalid return type"),
        }
    }
    pub fn mul(&mut self, a: i32, b: i32) -> i32 {
        let call = MethodCall::new(
            MathOp::Mul,
            vec![DataType::Integer(a), DataType::Integer(b)],
        );
        let result = self.call(call);
        match result {
            DataType::Integer(i) => i,
            _ => panic!("Invalid return type"),
        }
    }
    pub fn div(&mut self, a: i32, b: i32) -> i32 {
        let call = MethodCall::new(
            MathOp::Div,
            vec![DataType::Integer(a), DataType::Integer(b)],
        );
        let result = self.call(call);
        match result {
            DataType::Integer(i) => i,
            _ => panic!("Invalid return type"),
        }
    }
    fn call(&mut self, call: MethodCall) -> DataType {
        let encoded = serialize(&call).unwrap();
        self.stream.write(&encoded).unwrap();
        let mut buffer = [0; 1024];
        let size = self.stream.read(&mut buffer).unwrap();
        let decoded: DataType = deserialize(&buffer[..size]).unwrap();
        decoded
    }
}
