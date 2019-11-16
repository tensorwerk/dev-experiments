use byteorder::{LittleEndian, WriteBytesExt};
use ring::digest::{Context, SHA256};
use log::{info, debug, error};

use std::mem;
use std::env;
use std::fs;
use std::path::Path;
use std::io;
use std::io::{Write, BufWriter};
use std::net::{SocketAddr, TcpStream};

const USAGE: &str =
"File transfer benchmarking utility.

USAGE:
    <exe> <path> <address>

Where address can be parsed by the std::net::SocketAddr Parse implementation.
";

// TODO: impl Termination for a special error type
pub fn run() -> Result<(), i32> {
    env_logger::builder()
        .format_timestamp(None)
        .format_module_path(false)
        .init();

    let mut args = env::args().skip(1);

    let path = get_required_arg(args.next(), "path")?;
    let addr = get_required_arg(args.next(), "address")?;
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

fn get_required_arg(arg: Option<String>, name: &str) -> Result<String, i32> {
    match arg {
        Some(arg) => Ok(arg),
        None => {
            eprintln!("error: missing required argument <{}>", name);
            println!("{}", USAGE);
            Err(-1)
        }
    }
}

fn transfer_file<P: AsRef<Path>>(
    path: P,
    addr: SocketAddr,
    chunk_size: usize
) -> Result<(), io::Error> {

    let file = fs::read(path)?;

    let digest = {
        let mut context = Context::new(&SHA256);
        context.update(&file[..]);
        context.finish()
    };

    let stream = TcpStream::connect(addr)?;
    let mut stream = BufWriter::new(stream);

    assert_eq!(mem::size_of::<usize>(), 8);

    // Write digest size and digest content
    let digest: &[u8] = digest.as_ref();
    stream.write_u64::<LittleEndian>(digest.len() as u64)?;
    stream.write_all(digest)?;

    Ok(())
}
