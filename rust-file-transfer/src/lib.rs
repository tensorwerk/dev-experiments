use std::net::SocketAddr;
use std::env;

const USAGE: &str =
"File transfer benchmarking utility.

USAGE:
    <exe> <path> <address>

Where address can be parsed by the std::net::SocketAddr Parse implementation.
";

// TODO: impl Termination for a special error type
pub fn run() -> Result<(), i32> {
    let mut args = env::args().skip(1);

    let path = get_required_arg(args.next(), "path")?;
    let addr = get_required_arg(args.next(), "address")?;
    let addr: SocketAddr = match addr.parse() {
        Ok(addr) => addr,
        Err(_) => {
            eprintln!("Could not parse address");
            return Err(-1);
        }
    };

    println!("Got args: <{}>, <{}>", &path, &addr);

    //listen(&addr)
    //    .expect("Failed to bind listener");

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
