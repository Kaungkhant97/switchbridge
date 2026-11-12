//! SWB1 v1 packets use a fixed 24-byte, network-byte-order header.
use std::io::{self, Read, Write};

pub const MAX_PAYLOAD: usize = 1024 * 1024;
const HEADER_SIZE: usize = 24;

#[derive(Debug, PartialEq, Eq)]
pub struct Packet {
    pub sequence: u32,
    /// Transport chunk timestamp, never an H.264 frame presentation timestamp.
    pub timestamp_us: u64,
    pub payload: Vec<u8>,
}

pub fn read_packet(reader: &mut impl Read) -> io::Result<Option<Packet>> {
    let mut header = [0_u8; HEADER_SIZE];
    loop {
        match reader.read(&mut header[..1]) {
            Ok(0) => return Ok(None),
            Ok(_) => break,
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error) => return Err(error),
        }
    }
    reader.read_exact(&mut header[1..])?;
    if &header[..4] != b"SWB1" || header[4..8] != [1, 1, 0, 0] {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid SWB1 header",
        ));
    }
    let length = u32::from_be_bytes(header[8..12].try_into().unwrap()) as usize;
    if !(1..=MAX_PAYLOAD).contains(&length) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid SWB1 payload size",
        ));
    }
    let mut payload = vec![0; length];
    reader.read_exact(&mut payload)?;
    Ok(Some(Packet {
        sequence: u32::from_be_bytes(header[12..16].try_into().unwrap()),
        timestamp_us: u64::from_be_bytes(header[16..24].try_into().unwrap()),
        payload,
    }))
}

pub fn write_packet(writer: &mut impl Write, packet: &Packet) -> io::Result<()> {
    if !(1..=MAX_PAYLOAD).contains(&packet.payload.len()) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "invalid SWB1 payload size",
        ));
    }
    let mut header = [0_u8; HEADER_SIZE];
    header[..8].copy_from_slice(b"SWB1\x01\x01\0\0");
    header[8..12].copy_from_slice(&(packet.payload.len() as u32).to_be_bytes());
    header[12..16].copy_from_slice(&packet.sequence.to_be_bytes());
    header[16..24].copy_from_slice(&packet.timestamp_us.to_be_bytes());
    writer.write_all(&header)?;
    writer.write_all(&packet.payload)
}
