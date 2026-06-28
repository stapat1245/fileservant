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

    let path = request_line
    .split_whitespace()
    .nth(1)
    .unwrap_or("/");


    let file_path = if path == "/" {
        "."
    } else {
        &path[1..] 
    };

    let path = Path::new(file_path);
    // println!("{:?}",path);

    if !path.exists(){
        let status_line = "HTTP/1.1 404 Not Found";
        let contents = fs::read("web/404.html").unwrap();
        let length = contents.len();

        let response =
            format!("{status_line}\r\nContent-Length: {length}\r\n Content-Type: application/octet-stream\r\n\r\n");

        stream.write_all(response.as_bytes()).unwrap();
        stream.write_all(&contents).unwrap();
        return;

    }
    
    if path.is_file(){
        // serving file
        let status_line = "HTTP/1.1 200 OK";
        let filename = file_path;

        let contents = fs::read(filename).unwrap();
        let length = contents.len();

        let response =
            format!("{status_line}\r\nContent-Length: {length}\r\n Content-Type: application/octet-stream\r\n\r\n");

        stream.write_all(response.as_bytes()).unwrap();
        stream.write_all(&contents).unwrap();
    } else if path.is_dir(){
        // serve directory
        let contents = make_dir_content(path);
        let length = contents.len();
        let status_line = "HTTP/1.1 200 OK";

        let response =
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();

    }
}

fn make_dir_content(path:&Path)->String{
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

        let tag = format!(r#"<li><a href="{0}">{0}</a></li>"#,name);
        content.push_str(&tag);
    }
    content.push_str(&footer);
    content

}