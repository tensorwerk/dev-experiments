use byteorder::{
    LittleEndian,
    ReadBytesExt,
    WriteBytesExt,
};
use ring::digest::{
    Context,
    SHA256,
};

use std::str;
use std::mem;
use std::fs;
use std::path::Path;
use std::io;
use std::io::{
    Read,
    Write,
    BufReader,
    BufWriter,
};
use std::net::{
    SocketAddr,
    TcpStream,
    TcpListener,
};

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

    let file_size = file.len();
    let chunks = file_size / chunk_size;
    let remainder = file_size % chunk_size;

    // Write file size
    stream.write_u64::<LittleEndian>(file_size as u64)?;
    // Write chunk size
    stream.write_u64::<LittleEndian>(chunk_size as u64)?;

    stream.flush()?;

    Ok(())
}

pub fn receive_file(addr: SocketAddr) -> Result<(), io::Error> {
    // TODO: Maybe move logging outside this function...
    use log::{info, debug};

    let server = TcpListener::bind(addr)?;

    let (stream, _) = server.accept()?;
    let mut stream = BufReader::new(stream);

    assert_eq!(mem::size_of::<usize>(), 8);

    // Read the digest
    let digest_size = stream.read_u64::<LittleEndian>()?;
    let mut digest = vec![0u8; digest_size as usize];
    stream.read_exact(&mut digest)?;

    debug!("Read digest with size: {}", digest_size);

    // Read file size
    let file_size = stream.read_u64::<LittleEndian>()?;
    // Read chunk size
    let chunk_size = stream.read_u64::<LittleEndian>()?;

    debug!("Read file and chunk size");

    info!("Receiving a new file...");
    info!("Expecting hash: {:X?}", digest);
    info!("Expecting {} bytes with chunk size {}", file_size, chunk_size);

    Ok(())
}
