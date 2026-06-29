use std::{
    fs::{self,}, io::{BufReader,prelude::*}, net::{TcpListener,TcpStream}, path::Path, 
};
use clap::Parser;

#[derive(Parser,Debug)]
struct Args{
    //// mention port
    #[arg(long,short)] 
    port:Option<u16>,

    //// network
    #[arg(long,short)]
    ip:Option<String>,
}

const NOT_FOUND: &str = r#"<!DOCTYPE html>
<html>
<head><title>404</title></head>
<body>
<h1>404 Not Found</h1>
</body>
</html>"#;
fn main() {
    let args:Args = Args::parse();
    let ip = args.ip.unwrap_or("0.0.0.0".to_owned());
    let port = args.port.unwrap_or(1245);
 
    let addr = format!("{}:{}",ip,port);


    let listener = match TcpListener::bind(&addr){
        Ok(listener)=>{
            listener
        },
        Err(error)=>{
            println!("Error Occured NOOO {}",error);
            return;
        }
    };

    println!("Started a local file host on {}",addr);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
// --snip--

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    println!("{request_line:#?}");

    let request_path = request_line
    .split_whitespace()
    .nth(1)
    .unwrap_or("/");


    let file_path = if request_path == "/" {
        "."
    } else {
        &request_path[1..] 
    };

    let path = Path::new(file_path);
    // println!("{:?}",path);

    if !path.exists(){
        let response = format!(
            "HTTP/1.1 404 Not Found\r\n\
            Content-Length: {}\r\n\
            Content-Type: text/html\r\n\r\n{}",
            NOT_FOUND.len(),
            NOT_FOUND
        );


        stream.write_all(response.as_bytes()).unwrap();
        return;

    }
    
    if path.is_file(){
        // serving file
        // let status_line = "HTTP/1.1 200 OK";
        let filename = file_path;
        // println!("request_path = {:?}", request_path);
        // println!("file_path    = {:?}", file_path);
        // println!("exists       = {}", Path::new(filename).exists());

        match fs::read(filename) {
            Ok(contents) => {
                // println!("Read {} bytes", contents.len());

                let response = format!(
                    "HTTP/1.1 200 OK\r\n\
                    Content-Length: {}\r\n\
                    Content-Type: application/octet-stream\r\n\r\n",
                    contents.len()
                );

                stream.write_all(response.as_bytes()).unwrap();
                stream.write_all(&contents).unwrap();
            }
            Err(e) => {
                eprintln!("fs::read({:?}) failed: {}", filename, e);
                return;
            }
        }
    } else if path.is_dir(){
        // serve directory
        let contents = make_dir_content(path,request_path);
        let length = contents.len();
        let status_line = "HTTP/1.1 200 OK";

        let response =
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();

    }
}

fn make_dir_content(path:&Path,url_path: &str)->String{
    let head = r#"<!DOCTYPE HTML>
        <html lang="en">
        <head>
        <meta charset="utf-8">
        <title>Directory listing for /</title>
        </head>
        <body>
        <h1>Directory listing for /</h1>
        <hr>
        <ul>"#.to_owned();
    let footer = r#"</ul>
        <hr>
        </body>
        </html>"#.to_owned();
    let mut content = String::new();
    content.push_str(&head);


    for entry in fs::read_dir(path).unwrap(){
        let entry = entry.unwrap();

        let mut name = entry.file_name().to_string_lossy().to_string();

        if entry.path().is_dir(){
            name.push('/'); 
        }
        let href = if url_path == "/" {
            format!("/{}", name)
        } else {
            format!("{}/{}", url_path.trim_end_matches('/'), name)
        };

        let tag = format!(
        r#"<li><a href="{href}">{name}</a></li>"#
        );
        content.push_str(&tag);
    }
    content.push_str(&footer);
    content

}