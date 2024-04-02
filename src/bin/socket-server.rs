use common_library::socket::server::Server;
use ctrlc;
use std::env;
use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::mpsc::channel;

fn main() {
    println!("process start");

    let job = |mut stream: TcpStream| {
        match stream.write("--- greeting ---\r\n".as_bytes()) {
            Ok(_) => (),
            Err(e) => assert!(false, "{}", e),
        }

        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(_) => (),
            Err(e) => assert!(false, "{}", e),
        }

        println!("read data : ({})", String::from_utf8_lossy(&buffer));

        match stream.write(&buffer) {
            Ok(_) => (),
            Err(e) => assert!(false, "{}", e),
        }

        println!("write data : ({})", String::from_utf8_lossy(&buffer));

        match stream.flush() {
            Ok(_) => (),
            Err(e) => {
                println!("{}", e);
                return;
            }
        }
    };

    let mut server = Server::new();
    match server.start(String::from(&env::args().collect::<Vec<_>>()[1]), job) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    };

    let (tx, rx) = channel();
    ctrlc::set_handler(move || tx.send(()).expect("")).expect("");
    rx.recv().expect("");

    server.stop();

    println!("process end");
}
