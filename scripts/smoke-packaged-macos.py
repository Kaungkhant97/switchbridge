"""Execute a packaged receiver/sender and verify byte transport, with timeouts."""

import hashlib
import os
from pathlib import Path
import select
import socket
import subprocess
import sys
import tempfile


def main():
    binary = Path(sys.argv[1]).resolve(strict=True)
    with socket.socket() as listener:
        listener.bind(("127.0.0.1", 0))
        port = listener.getsockname()[1]
    address = f"127.0.0.1:{port}"
    with tempfile.TemporaryDirectory(prefix="switchbridge-smoke-") as directory:
        source = Path(directory) / "input.bin"
        output = Path(directory) / "output.bin"
        # Transport sentinel only: an Annex B prefix followed by arbitrary bytes.
        # This checks byte transport and does not claim video decoding.
        source.write_bytes(b"\x00\x00\x00\x01" + os.urandom(65536))
        receiver = subprocess.Popen(
            [str(binary), "receive", address, str(output)],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.PIPE,
        )
        try:
            readable, _, _ = select.select([receiver.stderr], [], [], 10)
            if not readable or b"Listening on" not in os.read(receiver.stderr.fileno(), 4096):
                raise RuntimeError("Receiver did not report readiness within 10 seconds")
            subprocess.run(
                [str(binary), "send", address, str(source)], check=True, timeout=10
            )
            if receiver.wait(timeout=10) != 0:
                raise RuntimeError("Packaged receiver failed")
            if output.read_bytes() != source.read_bytes():
                raise RuntimeError("Packaged loopback output differs from input")
            print("Packaged loopback SHA256:", hashlib.sha256(output.read_bytes()).hexdigest())
        finally:
            if receiver.poll() is None:
                receiver.kill()
            receiver.wait()
            receiver.stderr.close()


if __name__ == "__main__":
    main()
