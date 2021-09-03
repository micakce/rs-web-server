use std::{fs, io::{Read, Write}, net::{TcpListener, TcpStream}, thread, time::Duration};

fn main() {
    let mut count = 0;
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
        println!("Connection {} established", count);
        count+=1;
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let get_sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, template) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "./templates/home.html")
    } else if buffer.starts_with(get_sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "./templates/sleep.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND","./templates/404.html")
    };
    let contents = fs::read_to_string(template).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
