use std::{
    io::{BufReader, Write},
    net::{TcpListener, TcpStream},
};

mod request;
use request::decode_http_request;

mod response;
use response::get_http_response;

mod thread_pool;
use thread_pool::ThreadPool;

pub fn server(addr: &str, port: &str) {
    let bind_addr = format!("{addr}:{port}");
    let listener = TcpListener::bind(&bind_addr).unwrap();
    println!("Listening on {bind_addr}");

    let thread_pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread_pool.execute(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {e}")
            }
        }
    }

    println!("Shutting down.");
}

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request = decode_http_request(buf_reader);
    println!("Request: {http_request:#?}");

    let request_line = &http_request[0];
    let http_response = get_http_response(request_line);
    stream.write_all(http_response.as_bytes()).unwrap();
}
