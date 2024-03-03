use std::net::UdpSocket;
use std::io;
use std::str;
use std::thread;


use clap::Parser;

#[derive(Parser)]
struct Cli {
    own_ip: String,
    peer_ip: String,
}

fn main() -> std::io::Result<()> {
    // Get the command line arguments.
    let args = Cli::parse();

    // Create UDP socket.
    let socket = UdpSocket::bind(args.own_ip.clone())?;
    socket.set_nonblocking(true)?;
    let socket_clone = socket.try_clone()?;
    println!("Binding to {}", args.own_ip);

    // Thread for listening to incoming messages.
    thread::spawn(move || loop {
        let mut buf = [0; 1024];
        match socket_clone.recv_from(&mut buf) {
            Ok((amt, src)) => {
                println!("Recieved {} bytes from {}: {}", amt, src, str::from_utf8(&buf[..amt]).unwrap());
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // If no data is available. This is expected since the socket is non-blocking.
                // Do nothing and try again.
                continue;
            }
            Err(e) => {
                // If there is an error, print it and break the loop.
                eprintln!("Couldn't recieve a datagram: {}", e);
                break;
            }
        }
    });

    // Thread for sending messages.
    loop {
        let mut input = String::new();
        println!("Enter a message to send: ");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match socket.send_to(input.as_bytes(), &args.peer_ip) {
                    Ok(_) => println!("Sent message to {}", args.peer_ip),
                    Err(error) => println!("Failed to send message: {}", error),
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
