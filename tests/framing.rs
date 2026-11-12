use std::io::{self, Cursor};
use switchbridge::framing::{MAX_PAYLOAD, Packet, read_packet, write_packet};

fn fixture() -> Vec<u8> {
    vec![
        83, 87, 66, 49, 1, 1, 0, 0, 0, 0, 0, 4, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 1, 101,
    ]
}

#[test]
fn writes_independently_specified_wire_bytes() {
    let packet = Packet {
        sequence: 2,
        timestamp_us: 9,
        payload: vec![0, 0, 1, 101],
    };
    let mut bytes = Vec::new();
    write_packet(&mut bytes, &packet).unwrap();
    assert_eq!(bytes, fixture());
}

#[test]
fn reads_wire_fixture_and_clean_boundary_eof() {
    let mut reader = Cursor::new(fixture());
    let packet = read_packet(&mut reader).unwrap().unwrap();
    assert_eq!(packet.sequence, 2);
    assert_eq!(packet.timestamp_us, 9);
    assert_eq!(packet.payload, [0, 0, 1, 101]);
    assert!(read_packet(&mut reader).unwrap().is_none());
    assert!(read_packet(&mut io::empty()).unwrap().is_none());
}

#[test]
fn every_truncated_prefix_is_rejected() {
    let fixture = fixture();
    for length in 1..fixture.len() {
        let error = read_packet(&mut &fixture[..length]).unwrap_err();
        assert_eq!(
            error.kind(),
            io::ErrorKind::UnexpectedEof,
            "length {length}"
        );
    }
}

#[test]
fn malformed_magic_version_kind_and_flags_are_rejected() {
    for index in [0, 1, 2, 3, 4, 5, 6, 7] {
        let mut bytes = fixture();
        bytes[index] ^= 0xff;
        assert_eq!(
            read_packet(&mut &bytes[..]).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
    }
}

#[test]
fn invalid_payload_sizes_are_rejected_before_reading_payload() {
    for length in [0, (MAX_PAYLOAD + 1) as u32, u32::MAX] {
        let mut bytes = fixture()[..24].to_vec();
        bytes[8..12].copy_from_slice(&length.to_be_bytes());
        assert_eq!(
            read_packet(&mut &bytes[..]).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
    }
}

#[test]
fn write_limits_and_maximum_round_trip_are_enforced() {
    for size in [0, MAX_PAYLOAD + 1] {
        let packet = Packet {
            sequence: 0,
            timestamp_us: 0,
            payload: vec![7; size],
        };
        let mut bytes = Vec::new();
        assert_eq!(
            write_packet(&mut bytes, &packet).unwrap_err().kind(),
            io::ErrorKind::InvalidInput
        );
        assert!(bytes.is_empty());
    }
    let packet = Packet {
        sequence: u32::MAX,
        timestamp_us: u64::MAX,
        payload: vec![7; MAX_PAYLOAD],
    };
    let mut bytes = Vec::new();
    write_packet(&mut bytes, &packet).unwrap();
    assert_eq!(read_packet(&mut &bytes[..]).unwrap().unwrap(), packet);
}

struct Fails;
impl io::Read for Fails {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        Err(io::ErrorKind::PermissionDenied.into())
    }
}
impl io::Write for Fails {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Err(io::ErrorKind::BrokenPipe.into())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn io_errors_are_propagated() {
    assert_eq!(
        read_packet(&mut Fails).unwrap_err().kind(),
        io::ErrorKind::PermissionDenied
    );
    let packet = Packet {
        sequence: 0,
        timestamp_us: 0,
        payload: vec![1],
    };
    assert_eq!(
        write_packet(&mut Fails, &packet).unwrap_err().kind(),
        io::ErrorKind::BrokenPipe
    );
}
