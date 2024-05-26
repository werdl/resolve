// a command line utility program that resolves a hostname to an IP address and vice versa, using the system's DNS resolver.
use std::env;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

use dns_lookup::{lookup_addr, lookup_host};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <hostname>", args[0]);
        std::process::exit(1);
    }

    let hostname = &args[1];
    // Resolve the hostname to an IP address
    // check if the hostname is an IP address - match it against IPv4 and IPv6 regexes
    if is_ip(hostname) {
        let ip = IpAddr::from_str(hostname);

        if ip.is_err() {
            eprintln!("Invalid IP address: {}", hostname);
            std::process::exit(1);
        }

        let hostname = lookup_addr(&ip.clone().unwrap());

        if hostname.is_err() {
            eprintln!("Failed to resolve IP address {}: {}", ip.unwrap(), hostname.err().unwrap());
            std::process::exit(1);
        }

        println!("{} -> {}", ip.unwrap(), hostname.unwrap());
    } else {
        let ips = lookup_host(hostname);

        match ips {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to resolve {}: {}", hostname, e);
                std::process::exit(1);
            }
        }

        println!("{}", hostname);
        for ip in ips.unwrap() {
            println!("\t-> {}", ip);
        }
    }
}

fn is_ip(ip: &str) -> bool {
    Ipv4Addr::from_str(ip).is_ok() || Ipv6Addr::from_str(ip).is_ok()
}
