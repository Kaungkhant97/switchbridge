use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::time::Duration;
use switchbridge::transport::{receive, send, validate_endpoint};

const TIMEOUT: Duration = Duration::from_secs(10);
const HELP: &str = "switchbridge send 127.0.0.1:PORT INPUT.h264\nswitchbridge receive 127.0.0.1:PORT OUTPUT.h264\nswitchbridge usb-snapshot PATH\nswitchbridge usb-diff BEFORE AFTER\n\nPrerecorded H.264 Annex B transport over loopback only.\nChunk timestamps are not frame timing. Switch 2 video capture is UNVERIFIED.\nReceive accepts one connection; OUTPUT must not already exist.\n";

fn main() {
    if let Err(error) = run(&std::env::args().skip(1).collect::<Vec<_>>()) {
        eprintln!("switchbridge: {error}");
        std::process::exit(1);
    }
}

fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
    if args.is_empty() || args == ["--help"] || args == ["-h"] {
        print!("{HELP}");
        return Ok(());
    }
    if matches!(args[0].as_str(), "usb-snapshot" | "usb-diff") {
        return switchbridge::usb::run(args);
    }
    if args.len() != 3 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, HELP).into());
    }
    let address = validate_endpoint(&args[1])?;
    match args[0].as_str() {
        "send" => {
            let mut input = File::open(&args[2])?;
            let mut socket = TcpStream::connect_timeout(&address, TIMEOUT)?;
            socket.set_write_timeout(Some(TIMEOUT))?;
            send(&mut input, &mut socket)?;
            socket.shutdown(std::net::Shutdown::Write)?;
        }
        "receive" => receive_file(TcpListener::bind(address)?, Path::new(&args[2]))?,
        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, HELP).into()),
    }
    Ok(())
}

fn receive_file(listener: TcpListener, path: &Path) -> io::Result<()> {
    let output = OpenOptions::new().write(true).create_new(true).open(path)?;
    let result = (|| {
        eprintln!("Listening on {}", listener.local_addr()?);
        let (mut socket, peer) = listener.accept()?;
        if !peer.ip().is_loopback() {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "non-loopback peer",
            ));
        }
        socket.set_read_timeout(Some(TIMEOUT))?;
        let mut output = BufWriter::new(output);
        receive(&mut socket, &mut output)?;
        output.flush()
    })();
    if result.is_err() {
        fs::remove_file(path)?;
    }
    result
}
