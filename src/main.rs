use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use website::ThreadPool;

fn main() {
    let listner = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4).unwrap();

    for stream in listner.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request_line = buf_reader.lines().next().unwrap().unwrap();

    let status;
    let html;
    let html_lenght;
    match http_request_line.as_str() {
        "GET /home HTTP/1.1" => {
            status = "HTTP/1.1 200 OK";
            html = fs::read_to_string("html/home.html").unwrap();
            html_lenght = html.len();
        }

        "GET /products HTTP/1.1" => {
            status = "HTTP/1.1 200 OK";
            html = fs::read_to_string("html/home.html").unwrap();
            html_lenght = html.len();
        }

        "GET /about_us HTTP/1.1" => {
            status = "HTTP/1.1 200 OK";
            html = fs::read_to_string("html/home.html").unwrap();
            html_lenght = html.len();
        }

        // "GET /favicon/favicon.ico HTTP/1.1" => {
        //     status = "HTTP/1.1 200 OK";
        //     html = fs::read_to_string("favicon/favicon.ico").unwrap();
        //     html_lenght = html.len();
        // }
        "GET /style.css HTTP/1.1" => {
            status = "HTTP/1.1 200 OK";
            html = fs::read_to_string("html/style.css").unwrap();
            html_lenght = html.len();
        }

        _ => {
            println!("{}", http_request_line);
            status = "HTTP/1.1 404 NOT FOUND";
            html = fs::read_to_string("html/404.html").unwrap();
            html_lenght = html.len();
        }
    }
    let response = format!("{status}\r\nContent-Length: {html_lenght}\r\n\r\n{html}");
    stream.write_all(response.as_bytes()).unwrap();
}
