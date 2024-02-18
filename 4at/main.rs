use std::io::Write;
use std::net::TcpListener;
use std::result;

type Result<T> = result::Result<T, ()>;

fn main() -> Result<()> {
    let address = "127.0.0.1:8080";
    let tcp_listener = TcpListener::bind(address).map_err(|err| {
        eprintln!("Error: Could not bind to address {address}: {err}");
    })?;

    println!("INFO: Server listening on {address}");

    for stream in tcp_listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let _ = writeln!(stream, "Hello mien friend!").map_err(|err| {
                    eprintln!("Error: Could not write to stream: {err}");
                });
            }
            Err(err) => {
                eprintln!("Error: Could not establish connection: {err}");
            }
        }
    }

    Ok(())
}
