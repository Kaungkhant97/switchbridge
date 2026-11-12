use std::io::{self, Cursor};
use switchbridge::{
    framing::{Packet, write_packet},
    transport::{receive, send, validate_endpoint},
};

#[test]
fn transport_round_trip_preserves_annex_b_across_chunk_boundaries() {
    let mut input = vec![0, 0, 0, 1, 101];
    input.extend((0..150_000).map(|value| (value % 251) as u8));
    let mut wire = Vec::new();
    send(&mut Cursor::new(&input), &mut wire).unwrap();
    let mut output = Vec::new();
    receive(&mut &wire[..], &mut output).unwrap();
    assert_eq!(output, input);
}

#[test]
fn three_byte_annex_b_start_code_is_accepted() {
    let mut wire = Vec::new();
    send(&mut &[0, 0, 1, 101][..], &mut wire).unwrap();
    let mut output = Vec::new();
    receive(&mut &wire[..], &mut output).unwrap();
    assert_eq!(output, [0, 0, 1, 101]);
}

#[test]
fn sender_rejects_empty_short_or_non_annex_b_input() {
    for input in [&[][..], &[0][..], &[0, 0, 1][..], &[1, 2, 3, 4][..]] {
        let mut output = Vec::new();
        assert!(send(&mut Cursor::new(input), &mut output).is_err());
        assert!(output.is_empty());
    }
}

#[test]
fn receiver_rejects_missing_or_out_of_order_packets() {
    assert!(receive(&mut io::empty(), &mut Vec::new()).is_err());
    for sequences in [&[1][..], &[0, 0][..], &[0, 2][..]] {
        let mut wire = Vec::new();
        for sequence in sequences {
            write_packet(
                &mut wire,
                &Packet {
                    sequence: *sequence,
                    timestamp_us: 0,
                    payload: vec![0, 0, 1, 101],
                },
            )
            .unwrap();
        }
        assert_eq!(
            receive(&mut &wire[..], &mut Vec::new()).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
    }
}

#[test]
fn receiver_rejects_non_annex_b_first_packet() {
    let mut wire = Vec::new();
    write_packet(
        &mut wire,
        &Packet {
            sequence: 0,
            timestamp_us: 0,
            payload: vec![5, 6, 7, 8],
        },
    )
    .unwrap();
    assert_eq!(
        receive(&mut &wire[..], &mut Vec::new()).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
}

#[test]
fn only_numeric_loopback_endpoints_are_allowed() {
    for address in ["127.0.0.1:9000", "[::1]:9000", "127.0.0.2:9000"] {
        assert!(validate_endpoint(address).is_ok());
    }
    for address in [
        "0.0.0.0:9000",
        "[::]:9000",
        "192.168.1.2:9000",
        "localhost:9000",
        "invalid",
        "127.0.0.1:0",
    ] {
        assert!(validate_endpoint(address).is_err(), "{address}");
    }
}
