use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    // 监听地址: 127.0.0.1:7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // let response = "HTTP/1.1 200 OK\r\n\r\n";
    // stream.write_all(response.as_bytes()).unwrap();

    let status_line = "HTTP/1.1 200 OK\r\n\r\n";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();
    // TODO 不渲染 html
    let content_type = "Content-Type:text/html; charset=utf-8";
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n{content_type}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
