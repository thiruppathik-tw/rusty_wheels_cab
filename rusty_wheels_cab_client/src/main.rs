use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};
use std::env;

fn main() -> std::io::Result<()> {
    let arg = std::env::args().nth(1);

    let message = match arg {
        Some(msg) => msg,
        None => String::from("Hello!"),
    };

    let ip = env::var("IP").unwrap();
    let mut stream = TcpStream::connect(ip)?;

    writeln!(stream, "{}", message)?;

    stream.shutdown(Shutdown::Write)?;

    let mut buffer = String::new();

    stream.read_to_string(&mut buffer)?;

    println!("{}", buffer);

    Ok(())
}
