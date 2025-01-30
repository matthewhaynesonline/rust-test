use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};

pub fn decode_http_request(buf_reader: BufReader<&TcpStream>) -> Vec<String> {
    let http_request = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    return http_request;
}
