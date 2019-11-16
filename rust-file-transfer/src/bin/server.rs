use rust_file_transfer::{
    get_required_arg,
    receive_file,
};

use log::{info, error};

use std::env;
use std::net::SocketAddr;

const USAGE: &str =
"File transfer benchmarking utility server.

Accepts files sent from, hashes them, and verifies them against the hash that
was sent.

USAGE:
    <exe> <address>

Where address can be parsed by the std::net::SocketAddr Parse implementation.
";

fn main() -> Result<(), i32> {
    env_logger::builder()
        .format_timestamp(None)
        .format_module_path(false)
        .init();

    let mut args = env::args().skip(1);

    let addr = get_required_arg(args.next(), "address", USAGE)?;
    let addr: SocketAddr = match addr.parse() {
        Ok(addr) => addr,
        Err(_) => {
            error!("Could not parse address");
            return Err(-1);
        }
    };

    info!("File transfer server listening on {}.", &addr);

    if let Err(e) = receive_file(addr) {
        error!("Error while attempting to receive a file:\n{}", e);
        return Err(-1);
    }

    info!("Connection served. Server shutting down.");

    Ok(())
}
