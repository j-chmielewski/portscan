use std::{env, process, net};
use colored::*;

static PORT_START: i32 = 1;
static PORT_END: i32 = 65535;

struct Target {
    host: String,
    port_start: i32,
    port_end: i32
}

fn parse_args() -> Target {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => Target {
            host: args[1].clone(),
            port_start: PORT_START,
            port_end: PORT_END
        },
        4 => Target {
            host: args[1].clone(),
            port_start: args[2].parse::<i32>().unwrap(),
            port_end: args[3].parse::<i32>().unwrap()
        },
        _ => {
            println!("Usage:\n{} target [port_start port_end]", args[0]);
            process::exit(1);
        }
    }
}

fn main() {
    let target = parse_args();
    println!("Scanning host {} ports {}-{}", target.host.bright_green(), target.port_start.to_string().bright_green(), target.port_end.to_string().bright_green());
    for port in target.port_start..target.port_end {
        if let Ok(_) = net::TcpStream::connect(format!("{}:{}", target.host, port)) {
            println!("Found open port: {}", port.to_string().bright_green());
        }
    }
}
