use rust_file_transfer::{
    get_required_arg,
    transfer_file,
};

use log::{info, error};

use std::env;
use std::net::SocketAddr;

const USAGE: &str =
"File transfer benchmarking utility.

USAGE:
    <exe> <path> <address>

Where address can be parsed by the std::net::SocketAddr Parse implementation.

There are several additional env vars that can be used to configure operation:

    CHUNK_SIZE - defaults to 1024, used to determine how often to flush the
        file to the TCP stream.
";

fn main() -> Result<(), i32> {
    env_logger::builder()
        .format_timestamp(None)
        .format_module_path(false)
        .init();

    let mut args = env::args().skip(1);

    let path = get_required_arg(args.next(), "path", USAGE)?;
    let addr = get_required_arg(args.next(), "address", USAGE)?;
    let addr: SocketAddr = match addr.parse() {
        Ok(addr) => addr,
        Err(_) => {
            error!("Could not parse address");
            return Err(-1);
        }
    };

    let chunk_size = env::var("CHUNK_SIZE")
        .unwrap_or_else(|_| "1024".to_string())
        .parse();
    let chunk_size = match chunk_size {
        Ok(n) => n,
        Err(_) => {
            error!("Could not parse CHUNK_SIZE");
            return Err(-1);
        }
    };

    info!("Transferring file '{}' to address '{}'.", &path, &addr);
    info!("Using CHUNK_SIZE of '{}'", chunk_size);

    if let Err(e) = transfer_file(path, addr, chunk_size) {
        error!("Error while transferring file:\n{}", e);
        return Err(-1);
    }

    Ok(())
}
