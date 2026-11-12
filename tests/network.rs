use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Receiver(Child);
impl Drop for Receiver {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
fn receiver() -> (Receiver, String, std::path::PathBuf) {
    let path = std::env::temp_dir().join(format!(
        "switchbridge-test-{}-{}.h264",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let reserved = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = reserved.local_addr().unwrap().to_string();
    drop(reserved);
    let mut child = Command::new(env!("CARGO_BIN_EXE_switchbridge"))
        .args(["receive", &address, path.to_str().unwrap()])
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let stderr = child.stderr.take().unwrap();
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let mut line = String::new();
        let result = BufReader::new(stderr).read_line(&mut line);
        let _ = tx.send((result, line));
    });
    let mut guard = Receiver(child);
    let (result, line) = rx
        .recv_timeout(Duration::from_secs(5))
        .expect("receiver startup timeout");
    result.unwrap();
    assert!(line.starts_with("Listening on"), "{line}");
    // The guard kills a failed or stalled receiver when the test exits.
    guard.0.try_wait().unwrap();
    (guard, address, path)
}
fn wait(receiver: &mut Receiver) -> bool {
    for _ in 0..100 {
        if let Some(status) = receiver.0.try_wait().unwrap() {
            return status.success();
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    panic!("receiver did not terminate");
}
#[test]
fn cli_tcp_round_trip_preserves_h264_and_existing_output_is_protected() {
    let (mut receiver, address, output) = receiver();
    let input = output.with_extension("input.h264");
    let bytes = [0, 0, 0, 1, 101, 88, 10, 20];
    fs::write(&input, bytes).unwrap();
    let sent = Command::new(env!("CARGO_BIN_EXE_switchbridge"))
        .args(["send", &address, input.to_str().unwrap()])
        .status()
        .unwrap();
    assert!(sent.success());
    assert!(wait(&mut receiver));
    assert_eq!(fs::read(&output).unwrap(), bytes);
    let result = Command::new(env!("CARGO_BIN_EXE_switchbridge"))
        .args(["receive", &address, output.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert_eq!(fs::read(&output).unwrap(), bytes);
    fs::remove_file(input).unwrap();
    fs::remove_file(output).unwrap();
}
#[test]
fn malformed_tcp_stream_fails_and_removes_partial_output() {
    let (mut receiver, address, output) = receiver();
    let mut socket = TcpStream::connect(address).unwrap();
    socket.write_all(b"SWB").unwrap();
    drop(socket);
    assert!(!wait(&mut receiver));
    assert!(!output.exists());
}
