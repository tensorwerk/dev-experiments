use byteorder::{LittleEndian, WriteBytesExt};
use ring::digest::{Context, SHA256};

use std::mem;
use std::fs;
use std::path::Path;
use std::io;
use std::io::{Write, BufWriter};
use std::net::{SocketAddr, TcpStream};

// TODO: impl Termination for a special error type

pub fn get_required_arg(
    arg: Option<String>,
    name: &str,
    usage: &str,
) -> Result<String, i32> {
    match arg {
        Some(arg) => Ok(arg),
        None => {
            eprintln!("error: missing required argument <{}>", name);
            println!("{}", usage);
            Err(-1)
        }
    }
}

pub fn transfer_file<P: AsRef<Path>>(
    path: P,
    addr: SocketAddr,
    chunk_size: usize,
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
