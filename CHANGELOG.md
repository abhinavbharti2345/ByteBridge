# Changelog

All notable changes to the **ByteBridge** project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added - 2026-11-23
- **Phase 1: Streaming Transfer Engine & Resumption:**
  - Implemented `Sender` pipeline streaming files in configurable chunks (default: 64 KiB to 4 MiB) with per-chunk hashing and `ChunkAck` flow control.
  - Implemented `Receiver` pipeline verifying incoming chunks, staging them in `.bytebridge-partial` files, validating whole-file hashes (SHA-256 or BLAKE3), and atomically renaming to target destinations.
  - Implemented `ResumeJournal` (`.bytebridge-resume.<transfer_id>.bin`) tracking verified chunk progress for chunk-granular recovery after interruption.
  - Implemented Noise IK mutual handshake (`perform_noise_ik_handshake`) with Curve25519, ChaCha20-Poly1305, and SAS 6-digit verification code derivation.
  - Added end-to-end integration tests in `crates/bytebridge-core/tests/transfer_test.rs` covering single files, multi-file batches, and deep nested directories over simulated direct-link duplex transports.
  - Added interactive `mock-transfer` subcommand to `bytebridge-cli` to demonstrate in-process direct-link transfers with live throughput metrics.
- **Scalable Architecture & Repository Layout:**
  - Added [`.graphifyignore`](.graphifyignore) to scope Graphify AST indexing strictly to ByteBridge crates, pruning 2,923 noise nodes from third-party plugins.
  - Authored canonical repository taxonomy guide [`Docs/architecture/repository-layout.md`](Docs/architecture/repository-layout.md) defining strict module boundaries and architectural invariants.
  - Added comprehensive [`.gitignore`](.gitignore) covering Cargo, Vite, Tauri, and ByteBridge temporary staging artifacts.
  - Added [`.github/workflows/ci.yml`](.github/workflows/ci.yml) cross-platform CI matrix testing on `windows-latest` and `ubuntu-latest`.
  - Expanded `bytebridge-core` modular architecture to include `device/`, `hardware/`, and `performance/` domains with standardized traits and policy models.
- **Phase 1: Cargo Workspace & Core Engine Scaffolding:**
  - Configured workspace-level discovery manifests [`.agents/plugins.json`](.agents/plugins.json) and [`.agents/skills.json`](.agents/skills.json) to explicitly register all 15 plugins and 24 skills.
  - Initialized root Cargo workspace with `crates/bytebridge-core` and `crates/bytebridge-cli`.
  - Implemented binary length-prefixed framing and message serialization with postcard (`crates/bytebridge-core/src/protocol/`).
  - Implemented streaming hashing engine supporting SHA-256 and BLAKE3 (`crates/bytebridge-core/src/integrity/`).
  - Implemented storage management with strict path traversal prevention, illegal name rejection (`CON`, `NUL`), and atomic publishing (`crates/bytebridge-core/src/storage/`).
  - Implemented Short Authentication String (SAS) 6-digit verification code derivation for Noise IK pairing (`crates/bytebridge-core/src/security/`).
  - Added in-process mock duplex link abstraction for hardware-independent CI simulation (`crates/bytebridge-core/src/transport/`).
  - Built `bytebridge-cli` test harness CLI with benchmark, inspect, and mock transfer commands.
  - Added comprehensive unit test suite covering framing, serialization, hash streaming, path security, and SAS derivation (8/8 tests passing).
- **Architectural Refinements & Specification Updates:**
  - Selected and locked in **Noise Protocol (`Noise_IK_25519_ChaChaPoly_BLAKE2s`)** via the audited `snow` crate for peer-to-peer mutual authentication and encrypted transport sessions.
  - Specified zero-config peer discovery using **IPv6 Link-Local multicast (`fe80::%scope_id`)** and scoped mDNS (`_bytebridge._tcp.local.`) on the direct link.
  - Added instant disk pre-allocation (`SetFileInformationByHandle` / `posix_fallocate`) upon Manifest approval to catch disk-full conditions on chunk 0 and eliminate file fragmentation.
  - Added sector-aligned Direct I/O (`FILE_FLAG_NO_BUFFERING` / `O_DIRECT`) support for Maximum Performance Mode to prevent OS dirty page cache thrashing.
  - Added protocol-level hash capability negotiation supporting baseline SHA-256 and negotiable BLAKE3.
  - Added `--mock-direct-link` virtual transport simulator in debug configurations for CI and local UI testing without requiring physical laptops.
  - Authored canonical architecture specs: [`Docs/architecture/wire-protocol.md`](Docs/architecture/wire-protocol.md), [`Docs/architecture/storage-engine.md`](Docs/architecture/storage-engine.md), [`Docs/architecture/resume-protocol.md`](Docs/architecture/resume-protocol.md), and [`Docs/security/secure-channel.md`](Docs/security/secure-channel.md).
- **Documentation Structure & Governance:**
  - Created master documentation hub [`Docs/README.md`](Docs/README.md) organizing technical specifications, architecture diagrams, security assessments, hardware verification logs, and developer manuals.
  - Initialized modular documentation folders under `Docs/` (`architecture/`, `security/`, `hardware/`, `manuals/`).
  - Updated agent rule [`.agents/rules/change-logging-and-doc-maintenance.md`](.agents/rules/change-logging-and-doc-maintenance.md) mandating that all project documentation reside in `Docs/` with proper file structure, prohibiting root clutter.
  - Updated [`AGENTS.md`](AGENTS.md) with strict requirements on documentation placement, cross-linking, and spec synchronization.
- **Agent Governance & Customization:**
  - Installed **Graphify** knowledge graph integration, rules (`.agents/rules/graphify.md`), and workflow (`.agents/workflows/graphify.md`).
  - Installed **Caveman** token-compression and engineering skill suite (`.agents/skills/caveman*`).
  - Installed **Unslop** UI polishing and design skill (`.agents/skills/unslop/`).
  - Installed 15 required domain plugins from `wshobson/agents` tailored to ByteBridge architecture (`systems-programming`, `javascript-typescript`, `frontend-mobile-development`, `multi-platform-apps`, `ui-design`, `security-scanning`, `backend-api-security`, `frontend-mobile-security`, `tdd-workflows`, `unit-testing`, `debugging-toolkit`, `developer-essentials`, `conductor`, `c4-architecture`, `documentation-standards`).
  - Added project-level `AGENTS.md` defining strict developer and security guidelines for AI coding agents.
  - Created structured master and phase-specific plans in `plan/` covering Phase 0 through Phase 6.

### Verification
- Executed `cargo test --workspace`: 12/12 unit and integration tests passed across framing, serialization, hash streaming, path security, SAS derivation, Noise IK handshake, and end-to-end multi-file nested transfers.
- Executed `cargo run -p bytebridge-cli -- mock-transfer --size-mb 50`: successfully streamed, verified, and published 50 MB payload over simulated direct link at 21.18 MB/s.
- Executed `cargo run -p bytebridge-cli -- --help`: binary compiled and executed successfully (exit code 0).
- Validated directory structure and tool execution via local CLI checks.
- Confirmed skills, rules, and plugins are properly recognized by Antigravity.
- Rebuilt knowledge graph via `graphify update .`.
