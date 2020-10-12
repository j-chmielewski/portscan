use std::env;
use std::process;
use std::net;

static PORT_START: i32 = 1;
static PORT_END: i32 = 100;

struct Target {
    host: String,
    port_start: i32,
    port_end: i32
}

fn parse_args() -> Target {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage:\n{} target", args[0]);
        process::exit(1);
    }
    let target = args[1].clone();
    Target {
        host: target,
        port_start: PORT_START,
        port_end: PORT_END
    }
}

fn main() {

    let target = parse_args();
    println!("Scanning host {} ports {}-{}", target.host, target.port_start, target.port_end);
    for port in PORT_START..PORT_END {
        if let Ok(_) = net::TcpStream::connect(format!("{}:{}", target.host, port)) {
            println!("Found open port: {}", port);
        }
    }
}
