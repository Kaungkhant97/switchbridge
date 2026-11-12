"""Reproduce a real synthetic H.264 roundtrip with the extracted package."""
import hashlib
import os
from pathlib import Path
import select
import socket
import subprocess
import tempfile

root = Path(__file__).resolve().parents[2]
with tempfile.TemporaryDirectory(prefix="switchbridge-synthetic-") as directory:
    work = Path(directory)
    subprocess.run(["tar", "-xzf", str(root / "dist/switchbridge-macos-universal.tar.gz"), "-C", directory], check=True)
    binary = work / "switchbridge"
    source, output = work / "synthetic.h264", work / "received.h264"
    subprocess.run(["ffmpeg", "-hide_banner", "-loglevel", "error", "-f", "lavfi", "-i", "testsrc2=size=160x90:rate=30", "-t", "2", "-an", "-c:v", "libx264", "-preset", "ultrafast", "-pix_fmt", "yuv420p", "-f", "h264", str(source)], check=True, timeout=30)
    with socket.socket() as reservation:
        reservation.bind(("127.0.0.1", 0))
        address = f"127.0.0.1:{reservation.getsockname()[1]}"
    receiver = subprocess.Popen([str(binary), "receive", address, str(output)], stderr=subprocess.PIPE)
    try:
        readable, _, _ = select.select([receiver.stderr], [], [], 10)
        if not readable or b"Listening on" not in os.read(receiver.stderr.fileno(), 4096):
            raise RuntimeError("receiver readiness failed")
        subprocess.run([str(binary), "send", address, str(source)], check=True, timeout=10)
        assert receiver.wait(timeout=10) == 0
        hashes = [hashlib.sha256(path.read_bytes()).hexdigest() for path in [source, output]]
        assert hashes[0] == hashes[1]
        print(f"synthetic.h264 SHA256 {hashes[0]}", flush=True)
        print(f"received.h264  SHA256 {hashes[1]}", flush=True)
        print(f"Transported {source.stat().st_size} bytes via extracted universal executable", flush=True)
        subprocess.run(["ffmpeg", "-hide_banner", "-loglevel", "error", "-f", "h264", "-i", str(output), "-f", "null", "-"], check=True, timeout=30)
        probe = subprocess.run(["ffprobe", "-v", "error", "-count_frames", "-select_streams", "v:0", "-show_entries", "stream=codec_name,width,height,nb_read_frames", "-of", "default=noprint_wrappers=1", str(output)], capture_output=True, text=True, check=True, timeout=30)
        print(probe.stdout, end="", flush=True)
        assert "nb_read_frames=60" in probe.stdout
        print("PASS: synthetic 60-frame H.264 decoded after byte-identical loopback. No console or projector observation.", flush=True)
    finally:
        if receiver.poll() is None:
            receiver.kill()
        receiver.wait()
        receiver.stderr.close()
