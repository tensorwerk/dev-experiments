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

use log::info;
use md5;

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
    assert_eq!(mem::size_of::<usize>(), 8);

    let file = fs::read(path)?;

    let mut now = std::time::Instant::now();

    let digest = md5::compute(&file[..]);

    let mut duration = now.elapsed();
    let mut seconds = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;
    info!("Hash SHA 256 of {} bytes took {} sec.", file.len(), seconds);

    now = std::time::Instant::now();

        let stream = TcpStream::connect(addr)?;
        let mut stream = BufWriter::new(stream);

    duration = now.elapsed();
    seconds = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;
    info!("Opening the connection took {} sec.", seconds);

    now = std::time::Instant::now();

        // Write digest size and digest content
        let digest: &[u8] = digest.as_ref();
        stream.write_u64::<LittleEndian>(digest.len() as u64)?;
        stream.write_all(digest)?;

    duration = now.elapsed();
    seconds = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;
    info!("Sending digest took {} sec.", seconds);

    now = std::time::Instant::now();

        let file_size = file.len();
        let chunks = file_size / chunk_size;
        let remainder = file_size % chunk_size;

        stream.write_u64::<LittleEndian>(file_size as u64)?;
        stream.write_u64::<LittleEndian>(chunk_size as u64)?;

        stream.flush()?;

        // TODO: Perhaps the server should determine the chunk size, and client's chunk size is only
        // the maximum supported?

        // Write the file to the socket
        let mut buffer = &file[..];

        for _ in 0..chunks {
            stream.write_all(&buffer[..chunk_size])?;
            buffer = &buffer[chunk_size..];
            stream.flush()?;
        }

        if remainder != 0 {
            stream.write_all(buffer)?;
            // TODO: pad?
            stream.flush()?;
        }
    
    duration = now.elapsed();
    seconds = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;
    info!("Sending the file took {} sec.", seconds);

    Ok(())
}

pub fn receive_file(addr: SocketAddr) -> Result<(), io::Error> {
    // TODO: Maybe move logging outside this function...
    use log::{debug, error};

    assert_eq!(mem::size_of::<usize>(), 8);

    let server = TcpListener::bind(addr)?;

    info!("Waiting for connection...");

    let (stream, _) = server.accept()?;
    let mut stream = BufReader::new(stream);

    info!("Receiving a new file...");

    // Read the digest
    let digest_size = stream.read_u64::<LittleEndian>()?;
    let mut expected_digest = vec![0u8; digest_size as usize];
    stream.read_exact(&mut expected_digest)?;

    debug!("Read digest with size: {}", digest_size);

    let file_size = stream.read_u64::<LittleEndian>()?;
    let chunk_size = stream.read_u64::<LittleEndian>()?;

    info!("Expecting {} bytes with chunk size {}", file_size, chunk_size);

    // Read the file
    let mut buffer = vec![0u8; file_size as usize];
    stream.read_exact(&mut buffer)?;

    info!("File received.");
    info!("Expecting hash: {:X?}", &expected_digest);

    // hash the file
    let actual_digest = md5::compute(&buffer[..]);

    if actual_digest.0 == &expected_digest[..] {
        info!("File received and verification successful!");
        info!("Expected:      {:X?}", &expected_digest);
        info!("Actual digest: {:x}", actual_digest);
    } else {
        error!("File verification failed!");
        error!("Expected:      {:X?}", &expected_digest);
        error!("Actual digest: {:x}", actual_digest);
    }

    Ok(())
}

