# ByteBridge

**ByteBridge** is a high-performance desktop application for direct laptop-to-laptop file transfers over USB4, Thunderbolt 3/4, and high-speed USB-C physical links without requiring an internet connection, local Wi-Fi, or intermediary servers.

---

## Key Highlights

- **Point-to-Point Physical Link:** Transfers stream directly over host-to-host adapters without routing through third-party servers, local routers, or silent wireless fallbacks.
- **Mutual Authentication (Noise IK):** Built on `Noise_IK_25519_ChaChaPoly_BLAKE2s` with Short Authentication String (SAS) 6-digit confirmation to guarantee zero man-in-the-middle attacks.
- **High-Throughput Streaming Engine:** Pipelined streaming with 64 KiB to 4 MiB chunks, unbuffered Direct I/O (`FILE_FLAG_NO_BUFFERING` / `O_DIRECT`) support, and disk pre-allocation.
- **Atomic Staging & Publishing:** In-flight data writes to temporary `.bytebridge-partial` files and atomically renames only upon whole-file cryptographic hash verification (BLAKE3 / SHA-256).
- **Chunk-Level Resumption:** Interruptions (such as cable disconnects) resume smoothly from the last verified chunk via binary `.bytebridge-resume` journals.
- **Honest Hardware Reporting:** Displays exact negotiated adapter rates (e.g. 20.0 Gbps) and strictly labels unconfirmed metrics as `"Unknown / Not exposed by hardware or OS"`.
- **Unslopped Desktop UI:** Clean, dark-mode native desktop interface built with Tauri 2, React 19, TypeScript, and Tailwind CSS.

---

## Supported Hardware & Cables

| Hardware Category | Supported Standard | Notes |
| :--- | :--- | :--- |
| **Cables (Mandatory)** | USB4 / Thunderbolt 4 / Passive USB-C (10Gbps+) | **Standard phone charging cables will NOT work** as they only supply 480 Mbps USB 2.0 data lines. |
| **Host Ports (Windows)** | Thunderbolt 3/4, USB4 Type-C | Verified on Windows 10/11 with NDIS Thunderbolt Networking. |
| **Host Ports (Linux)** | Thunderbolt 3/4, USB4 (`thunderbolt-net`) | Requires Linux kernel 5.15+ with `thunderbolt` and `thunderbolt-net` modules. |

---

## Repository Structure

```text
ByteBridge/
  crates/
    bytebridge-core/       # Headless Rust engine (protocol, transport, crypto, storage)
    bytebridge-cli/        # Benchmark & simulated transfer test harness
  src-tauri/               # Tauri 2 native desktop backend shell (Windows & Linux)
  ui/                      # React 19 + TypeScript frontend with Tailwind CSS
  .github/workflows/       # Automated CI & cross-platform release pipeline
```

---

## Building from Source

### Prerequisites
- **Rust Toolchain:** Rust 1.75+ (`rustup default stable`)
- **Node.js:** Node.js 20+ and `npm`
- **Linux Packages (Linux only):** `libwebkit2gtk-4.1-dev`, `build-essential`, `curl`, `libssl-dev`

### Running the Desktop App in Development Mode
```bash
# In project root:
cd ui
npm install
cd ..
cargo tauri dev
```

### Running Headless Tests
```bash
cargo test --workspace
```

### CLI Benchmark & Simulated Direct-Link Transfer
```bash
cargo run -p bytebridge-cli -- mock-transfer --size-mb 100
```

### Building Production Installers
```bash
# Builds Windows NSIS/MSI or Linux AppImage/deb based on the host OS:
cargo tauri build
```
The output installers will be generated under `target/release/bundle/`.

---

## License
Apache-2.0 or MIT
