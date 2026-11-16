# Bankai

A suite of offensive-security tools built for learning: a Rust CLI of individual
attack techniques, a Flask collector for the keylogger, and a deliberately
vulnerable TCP service to practise against. Built by Diogo Andrade and Tiago Pereira.

> **For authorised testing and education only.** Several of these tools are live
> attack techniques and one target service is intentionally vulnerable. Run them
> only on machines you own or have written permission to test, on an isolated
> network. See [SECURITY.md](SECURITY.md).

## Repository layout

| Path | Role |
|------|------|
| `CLI/` | Rust command-line tool; each technique is a module in `CLI/src/` |
| `Server/` | `server.py`: Flask endpoint that receives and stores keylogger output |
| `VulnerableCode/` | `VulnServerRevOver.py`: an intentionally vulnerable TCP service, the practice target for the buffer-overflow and reverse-shell work |
| `LICENSE` | GNU GPL v3 |

## Tools in the CLI

Each is a subcommand or menu entry in `CLI/src/`:

| Tool | Source | What it does |
|------|--------|--------------|
| Buffer Overflow | `overflow.rs` | Sends an overflowing payload at the vulnerable service |
| Reverse Shell | `reverse_shell.rs` | Opens a reverse shell back to a listener |
| Port Scanner | `port_scanner.rs` | Scans a host for open TCP ports |
| Keylogger | `keylogger.rs` | Captures keystrokes; can run in the background and POST to the Flask server |
| DLL Hijacking | `dll_hijacking_c.rs` | Demonstrates a DLL search-order hijack on Windows |
| Phishing | `phishing.rs` | Generates a phishing email and a QR code |
| Decrypt | `decrypt.rs` | File/string decryption utilities |

## Requirements

- **Rust** (stable, edition 2021) and Cargo — for the CLI
- **Python 3** with **Flask** — for `Server/server.py`
- **Windows** for the DLL-hijacking and keylogger tools (the CLI depends on `winapi`); the keylogger's Linux path uses `evdev`

## Build and run

### CLI

```bash
cd CLI
cargo build --release
```

Run the interactive menu:

```bash
cargo run --release
```

Or call a technique directly (see `cargo run -- --help` for the current list):

```bash
cargo run --release -- reverse-shell
```

### Keylog collector

```bash
pip install flask
python Server/server.py
```

Listens on `0.0.0.0:5000`; received logs are appended to `received_keylog.txt`.

### Vulnerable practice service

```bash
python VulnerableCode/VulnServerRevOver.py
```

Listens on `0.0.0.0:9999`. Run it only inside an isolated lab.

## License

GNU General Public License v3.0 — see [LICENSE](LICENSE).

## References

- [Wireshark documentation](https://www.wireshark.org/docs/)
- Reverse shells — <https://www.imperva.com/learn/application-security/reverse-shell/>
- Rust for malware development — <https://github.com/Whitecat18/Rust-for-Malware-Development>
- Black Hat Rust — <https://github.com/skerkour/black-hat-rust>
