/* xhci's Web Server Multi-Threaded
 * https://github.com/xhci1/webserver */

use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

mod json;

fn main() {
    let config = fs::read_to_string("config.json")
        .expect("Error : Config file not found \"config.json\"");
    let mut lexer = json::Lexer::new(config);
    let parsed = json::Parser::parse(lexer.lex());
    
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, parsed[0].data.clone(), parsed[1].data.clone());
    }
}

fn handle_connection(mut stream: TcpStream, main_page: String, err_page: String) {
    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let(status_line, filename) = if buf.starts_with(get) {
        ("HTTP/1.1 200 Ok", main_page)
    } else {
        ("HTTP/1.1 404 Not Found", err_page)
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
}
