use std::io;
use std::net::TcpStream;

pub fn main() ->  Result<(), io::Error> {
    println!("Enter the IP address of the remote server:");
    let mut ip = String::new();
    io::stdin().read_line(&mut ip).expect("Failed to read IP address");

    println!("Enter the port number of the remote server:");
    let mut port = String::new();
    io::stdin().read_line(&mut port).expect("Failed to read port number");

    let ip = ip.trim();
    let port = port.trim().parse::<u16>().expect("Invalid port number");

    let addr = format!("{}:{}", ip, port);
    TcpStream::connect(&addr)?;
    println!("Connected to {}!", addr);
    Ok(())
}
