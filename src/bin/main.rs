use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::io::Read;
use std::io;
use webserver::ThreadPool;
use webserver::directory_list;

fn main() {

    let mut hostname = String::new();
    let mut root = String::new();

    println!("Insert the hostname of the server and the port [Example = \"127.0.0.1:80]\"");
    io::stdin().read_line(&mut hostname).expect("Failed to read line");
    let hostname = hostname.trim();

    println!("Insert the root of the server");
    io::stdin().read_line(&mut root).expect("Failed to read the line");
    let root = root.trim();

    let listener = TcpListener::bind(hostname).unwrap();
    let pool = ThreadPool::new(4);
    
    for stream in listener.incoming() {
        let file_structure = directory_list(String::from(root));
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream, file_structure);
        });
    }
}

fn handle_connection(mut stream: TcpStream, file_structure: Vec<String>) {

    let mut file_structure = file_structure;

    let mut buffer = [0; 1024];

    let mut status_line = "HTTP/1.1 200 OK";
    let mut filename = "websiteroot/index.html";

    stream.read(&mut buffer).unwrap();

    for entry in (file_structure.iter_mut()).rev() {

        if entry.contains("404.html") {
            continue;
        }
        let mut site = entry.replace("websiteroot/", "").replace("index.html", "");
        site = format!("GET /{site} HTTP/1.1\r\n");
        let site = site.as_bytes();

        if buffer.starts_with(site) {
            status_line = "HTTP/1.1 200 OK";
            filename = entry;
            break;
        } else {
            status_line = "HTTP/1.1 404 NOT FOUND";
            filename = "websiteroot/404.html";
        }
    }

    if filename.contains(".png") || filename.contains(".jpg") || filename.contains(".gif") || filename.contains(".jpeg") {
        let contents = fs::read(filename);
        let contents = match contents {
            Ok(contents ) => contents,
            Err(error) => {
                println!("Problem opening file: {:?}", error);
                Vec::<u8>::new()
            }
        };

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n",
            status_line, &contents[..].len()
        );

        stream.write(response.as_bytes()).unwrap();
        stream.write(&contents[..]).unwrap();
        stream.flush().unwrap();
    } else {
        let contents = fs::read_to_string(filename);
        let contents = match contents {
            Ok(contents) => contents,
            Err(error) => {
                println!("Problem opening file: {:?}", error);
                String::new()
            }
        };

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line, contents.len(), contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}