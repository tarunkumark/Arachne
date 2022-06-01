use std::fs;
use std::io::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let get_home = b"GET /home HTTP/1.1\r\n";
    let get_view = b"GET /view HTTP/1.1\r\n";
    let post = b"POST / HTTP/1.1\r\n";
    let patch = b"PATCH / HTTP/1.1\r\n";
    let delete = b"DELETE / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let contents = fs::read_to_string("templates/index.html").unwrap();
        let s = match str::from_utf8(&buffer) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        println!("{}", s);
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }else if buffer.starts_with(get_home){
        let contents = fs::read_to_string("templates/home.html").unwrap();
        let s = match str::from_utf8(&buffer) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        println!("{}", s);
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    else if buffer.starts_with(get_view){
        let contents = fs::read_to_string("templates/view.html").unwrap();
        let s = match str::from_utf8(&buffer) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        println!("{}", s);
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

     else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("templates/404.html").unwrap();
        // let uri = get_uri_from_request(&buffer);
        // println!("{}",uri);
        let s = match str::from_utf8(&buffer) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        println!("{}", s);
        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

// fn get_uri_from_request(request: &[u8]) -> &[u8] {
//     let mut i = 0;

//     for line in request.split(|c| c == b'\r' || c == b'\n') {
//         if line.starts_with(b"GET") {
//             i += 4;
//             while request[i] == b' ' {
//                 i += 1;
//             }
//             return &request[i..];
//         }
//         i += line.len();
//     }

//     b"/404"
// }
}
