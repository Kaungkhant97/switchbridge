//! Bounded-memory H.264 Annex B chunk transport, independent of console hardware.
use crate::framing::{Packet, read_packet, write_packet};
use std::io::{self, Read, Write};
use std::net::SocketAddr;
use std::time::Instant;

const CHUNK_SIZE: usize = 64 * 1024;

pub fn validate_endpoint(address: &str) -> io::Result<SocketAddr> {
    let address: SocketAddr = address
        .parse()
        .map_err(|_| invalid("expected numeric loopback IP:port"))?;
    if !address.ip().is_loopback() || address.port() == 0 {
        return Err(invalid(
            "only loopback endpoints with a nonzero port are allowed",
        ));
    }
    Ok(address)
}

pub fn send(reader: &mut impl Read, writer: &mut impl Write) -> io::Result<()> {
    let started = Instant::now();
    let mut prefix = [0_u8; 4];
    reader.read_exact(&mut prefix)?;
    if !is_annex_b(&prefix) {
        return Err(invalid("input must begin with an H.264 Annex B start code"));
    }
    let mut sequence = 0_u32;
    let mut first = true;
    loop {
        let mut payload = vec![0; CHUNK_SIZE];
        let offset = if first {
            payload[..4].copy_from_slice(&prefix);
            4
        } else {
            0
        };
        let length = read_chunk(reader, &mut payload[offset..])? + offset;
        if length == 0 {
            break;
        }
        payload.truncate(length);
        let timestamp_us = u64::try_from(started.elapsed().as_micros()).unwrap_or(u64::MAX);
        write_packet(
            writer,
            &Packet {
                sequence,
                timestamp_us,
                payload,
            },
        )?;
        sequence = sequence
            .checked_add(1)
            .ok_or_else(|| invalid("too many transport chunks"))?;
        first = false;
    }
    writer.flush()
}

pub fn receive(reader: &mut impl Read, writer: &mut impl Write) -> io::Result<()> {
    let mut expected = 0_u32;
    while let Some(packet) = read_packet(reader)? {
        if packet.sequence != expected {
            return Err(invalid("missing, duplicated, or out-of-order SWB1 chunk"));
        }
        if expected == 0 && !is_annex_b(&packet.payload) {
            return Err(invalid(
                "first chunk must begin with an H.264 Annex B start code",
            ));
        }
        writer.write_all(&packet.payload)?;
        expected = expected
            .checked_add(1)
            .ok_or_else(|| invalid("too many transport chunks"))?;
    }
    if expected == 0 {
        return Err(invalid("stream contained no H.264 chunks"));
    }
    writer.flush()
}

fn read_chunk(reader: &mut impl Read, output: &mut [u8]) -> io::Result<usize> {
    let mut filled = 0;
    while filled < output.len() {
        match reader.read(&mut output[filled..]) {
            Ok(0) => break,
            Ok(length) => filled += length,
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error) => return Err(error),
        }
    }
    Ok(filled)
}

fn is_annex_b(bytes: &[u8]) -> bool {
    bytes.len() >= 4 && (bytes.starts_with(&[0, 0, 1]) || bytes.starts_with(&[0, 0, 0, 1]))
}

fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
