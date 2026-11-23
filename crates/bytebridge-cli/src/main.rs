use bytebridge_core::integrity::HashAlgorithm;
use bytebridge_core::transfer::{Receiver, Sender};
use bytebridge_core::transport::mock::create_mock_link;
use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use tempfile::tempdir;
use tracing::info;

#[derive(Parser)]
#[command(name = "bytebridge-cli")]
#[command(about = "ByteBridge Direct File Transfer Engine CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Benchmark raw loopback and storage throughput
    Benchmark {
        #[arg(short, long, default_value = "100")]
        size_mb: usize,
    },
    /// Inspect available network and host-to-host adapters
    Inspect,
    /// Verify an end-to-end simulated transfer using the in-process mock transport
    MockTransfer {
        #[arg(short, long, default_value = "10")]
        size_mb: usize,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Benchmark { size_mb } => {
            info!("Running benchmark with size: {} MB", size_mb);
            println!("ByteBridge benchmark engine: placeholder ready");
        }
        Commands::Inspect => {
            info!("Inspecting network adapters...");
            println!("ByteBridge adapter inspector: placeholder ready");
        }
        Commands::MockTransfer { size_mb } => {
            println!("============================================================");
            println!("ByteBridge Simulated Direct-Link Transfer ({} MB)", size_mb);
            println!("============================================================");

            let source_dir = tempdir()?;
            let dest_dir = tempdir()?;

            let test_file_path = source_dir.path().join("benchmark_test.dat");
            print!("Generating {} MB test payload in memory... ", size_mb);
            let payload = vec![0x5Au8; size_mb * 1024 * 1024];
            let mut f = File::create(&test_file_path)?;
            f.write_all(&payload)?;
            drop(f);
            println!("Done.");

            println!("Setting up in-process duplex transport...");
            let (mut sender_stream, mut receiver_stream) = create_mock_link(256 * 1024);

            let chunk_size = 1024 * 1024; // 1 MiB chunks
            let sender = Sender::new(chunk_size, HashAlgorithm::Blake3);
            let receiver = Receiver::new(dest_dir.path(), HashAlgorithm::Blake3);

            let files_to_send = vec![(test_file_path.clone(), "benchmark_test.dat".to_string())];

            let receiver_handle = tokio::spawn(async move {
                receiver.receive_all(&mut receiver_stream).await
            });

            println!("Streaming chunks with per-chunk hashing and ChunkAck window...");
            let start = Instant::now();

            sender
                .send_files(&mut sender_stream, &files_to_send)
                .await?;

            let received_files = receiver_handle.await??;
            let elapsed = start.elapsed();

            let mb_transferred = size_mb as f64;
            let seconds = elapsed.as_secs_f64();
            let mb_per_sec = mb_transferred / seconds;

            println!("Transfer completed successfully in {:.3} seconds ({:.2} MB/s)", seconds, mb_per_sec);
            println!("Receiver atomically published {} file(s)", received_files.len());
            println!("Whole-file cryptographic verification: PASSED (BLAKE3)");
            println!("============================================================");
        }
    }

    Ok(())
}
