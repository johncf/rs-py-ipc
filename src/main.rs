extern crate uuid;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json as json;

use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpListener};
use std::process::{Command, Stdio};

use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct Handshake(u16, Uuid);

fn main() {
    // bind
    let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
    let port = match listener.local_addr() {
        Ok(SocketAddr::V4(addr)) => {
            println!("bind: {}", addr);
            addr.port()
        }
        _ => panic!("Bad address!"),
    };

    // spawn
    let plug_path = std::env::current_exe().map(|mut p| {
        p.pop();
        p.join("../../plug.py")
    }).unwrap();
    let c = Command::new("/bin/python3")
                    .arg(plug_path)
                    .stdin(Stdio::piped())
                    .spawn()
                    .expect("plug.py command failed to start");

    // handshake send (via stdin)
    let plug_uuid = Uuid::new_v4();
    let json_str = json::to_string(&Handshake(port, plug_uuid)).unwrap();
    writeln!(c.stdin.unwrap(), "{}", json_str).unwrap();

    // listen
    let sock = match listener.accept() {
        Ok((sock, addr)) => {
            println!("accept: {}", addr);
            sock
        }
        _ => panic!("Bad accept!"),
    };

    // handshake recv (via sock)
    let mut reader = BufReader::new(sock);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let response: Handshake = json::from_str(&line).unwrap();
    println!("handshake: {}", response.1 == plug_uuid);
}
