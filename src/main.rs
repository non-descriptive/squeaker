use atty::Stream;
use terminal_size::{Width, Height, terminal_size};

use std::net::TcpStream;
use std::io::{Read, Write};

#[derive(Debug)]
struct Location<'a> {
    domain: &'a str,
    port: u32
}
impl<'a> Location<'a> {
    fn new(domain: &'a str, port: u32) -> Location {
        Location{domain, port}
    }
}

fn visit(domain: &str, port: u32, file_path: &str) -> Result<String, std::io::Error> {
    let gopher_hole = format!("{}:{}", domain, port);
    let stream = TcpStream::connect(&gopher_hole);

    match stream {
        Ok(mut stream) => {
            let selector = format!("{}\r\n", file_path);
            stream.write(selector.as_bytes()).unwrap();
            stream.flush().unwrap();

            let mut data: Vec<u8> = vec![];
            stream.read_to_end(&mut data).unwrap();
            Ok(String::from_utf8_lossy(&data).to_string())
        },
        Err(e) => {
            Err(e)
        }
    }
}
mod types;
use types::LineType;

fn render(data: &String) {
    let (Width(_width), Height(height)) = terminal_size().unwrap();
    let window_height = height as usize;
    let lines: Vec<&str> = data.split("\r\n").collect();
    let mut current_pos = 0;
    let mut done = false;
    let mut cnt = 0;
    for line in lines {
        if line.is_empty() { continue; }
        let line_type = LineType::from(&line[..1]);
        let split : Vec<&str>= line[1..].split("\t").collect();
        cnt +=1;
        let name = split[0];
        match line_type {
            LineType::RenderLine => {
                 println!("{}", name);
            }
            LineType::Dir => {
                let name = split[0];
                println!("{}. {}",cnt, name);
            }
            LineType::File => {
                let name = split[0];
                println!("{}. {}",cnt, name);
            }
            _ => continue,
        }
    }

        // myp!("\x1b[92m[Press Enter for Next page]\x1b[0m");
        // let mut command = String::new();
        // std::io::stdin().read_line(&mut command).unwrap();
        // if command.trim() == "q" {
        //     done = true;
        // }

}


fn main() {
    if !atty::is(Stream::Stdout) {
        println!("Not a tty");
        return;
    }
    let domain = "baud.baby";
    let port = 70;
    match visit(domain, port,"") {
        Ok(data)  => render(&data),
        _ => (),
    };
}
