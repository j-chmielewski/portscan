use std::env;
use std::process;
use std::net;

static PORT_START: i32 = 1;
static PORT_END: i32 = 100;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage:\n{} target", args[0]);
        process::exit(1);
    }
    let target = &args[1];
    println!("Scanning host {} ports {}-{}", target, PORT_START, PORT_END);
    for port in PORT_START..PORT_END {
        match net::TcpStream::connect(format!("{}:{}", target, port)) {
            Ok(_) => println!("Found open port: {}", port),
            Err(_) => {}
        }
    }
}
