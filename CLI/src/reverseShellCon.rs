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
    let mut stream = TcpStream::connect(&addr)?;

    println!("Connected to {}!", addr);

    loop {
     println!("Enter command:");
     let mut command = String::new();
     io::stdin().read_line(&mut command).expect("Failed to read command");

     if command.trim() == "exit" {
         break;
     }

     stream.write_all(command.as_bytes())?;
     let mut response = String::new();
     stream.read_to_string(&mut response)?;
     println!("{}", response);
 }
    Ok(())
}
