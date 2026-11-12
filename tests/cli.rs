use std::process::Command;

#[test]
fn cli_rejects_invalid_commands_and_non_loopback_endpoints() {
    for args in [
        vec!["unknown"],
        vec!["send", "0.0.0.0:9000", "/nonexistent"],
        vec!["receive", "192.168.1.1:9000", "/nonexistent"],
        vec!["send"],
        vec!["receive"],
    ] {
        assert!(
            !Command::new(env!("CARGO_BIN_EXE_switchbridge"))
                .args(args)
                .output()
                .unwrap()
                .status
                .success()
        );
    }
}

#[test]
fn help_describes_synthetic_transport_and_chunk_timing() {
    let output = Command::new(env!("CARGO_BIN_EXE_switchbridge"))
        .arg("--help")
        .output()
        .unwrap();
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.contains("receive"));
    assert!(text.contains("send"));
    assert!(text.contains("not frame timing"));
    assert!(text.contains("UNVERIFIED"));
}
