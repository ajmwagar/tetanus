use std::net::TcpListener;

fn main(){
    listener()
}

fn listener(){
let s = TcpListener::bind("0.0.0.0:6666").unwrap();

match s.accept() {
    Ok((_socket, addr)) => println!("new client: {:?}", addr),
    Err(e) => println!("couldn't get client: {:?}", e),
}

}
