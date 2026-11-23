# ByteBridge

ByteBridge is a high-performance desktop utility designed for direct laptop-to-laptop file transfers over wired USB4, Thunderbolt, and compatible host-to-host links.

## Key Highlights

- **Direct Wired Connection:** Transfers run point-to-point over direct links without routing through internet servers or falling back to Wi-Fi.
- **Mutual Authentication & Encryption:** Built on the Noise Protocol (`Noise_IK_25519_ChaChaPoly_BLAKE2s`) with short authentication string (SAS) pairing confirmation.
- **Streaming Transfer Engine:** Pipelined chunk streaming with per-chunk hashing and whole-file cryptographic verification (SHA-256 and BLAKE3).
- **Atomic Publishing:** In-flight data writes to temporary staging files and is published atomically only upon successful verification.
- **Chunk-Granular Resumption:** Interrupted transfers automatically resume from the last verified chunk without restarting from zero.
- **Modular Architecture:** Standalone headless core engine (`bytebridge-core`) decoupled from desktop and UI dependencies.

## Repository Layout

- `crates/bytebridge-core`: Standalone Rust engine implementing protocol framing, transport, storage, and security.
- `crates/bytebridge-cli`: Command-line testing and diagnostic utility.
- `.github/workflows`: Automated multi-platform CI pipeline (Windows and Linux).

## Building & Testing

### Prerequisites
- Rust 1.75+ (Cargo)

### Running Tests
```bash
cargo test --workspace
```

### Simulated Direct-Link Transfer
Run a local simulated direct-link transfer using the in-process mock transport:
```bash
cargo run -p bytebridge-cli -- mock-transfer --size-mb 50
```
