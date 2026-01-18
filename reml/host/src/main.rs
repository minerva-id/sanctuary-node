//! # Re-ML Host/Prover
//!
//! Off-chain component for generating STARK proofs of ML-DSA signature batches.
//!
//! ## Components
//!
//! - **Proof Generation**: Invokes SP1 prover on signature batches
//! - **Test Data Generation**: Creates valid ML-DSA signatures for testing
//! - **Local Verification**: Verifies proofs before on-chain submission
//! - **Aggregator Server**: HTTP server for receiving signature requests
//!
//! ## Usage
//!
//! ```bash
//! # Generate test signatures
//! reml-prover gen-test --count 10 --output batch.json
//!
//! # Generate proof
//! reml-prover prove --input batch.json --output proof.json
//!
//! # Verify proof locally
//! reml-prover verify --proof proof.json
//!
//! # Run aggregator server
//! reml-prover serve --port 8080
//! ```

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use pqcrypto_dilithium::dilithium2;
use pqcrypto_traits::sign::{PublicKey, SecretKey};
use reml_lib::{
    RemlProofBundle, RemlProofInput, RemlProofOutput, SignatureRequest,
    compute_requests_root, MLDSA_SIGNATURE_SIZE, MLDSA_PUBLIC_KEY_SIZE,
};
use sp1_sdk::{ProverClient, SP1Stdin, HashableKey};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};

/// The ELF binary of the guest program
const GUEST_ELF: &[u8] = include_bytes!("../../target/elf/riscv32im-succinct-zkvm-elf");

// ═══════════════════════════════════════════════════════════════════════════
// CLI INTERFACE
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Parser)]
#[command(name = "reml-prover")]
#[command(about = "Re-ML Prover - Generate STARK proofs for ML-DSA signatures")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Log level (trace, debug, info, warn, error)
    #[arg(short, long, default_value = "info")]
    log_level: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a proof for a batch of signature requests
    Prove {
        /// Input file containing signature requests (JSON)
        #[arg(short, long)]
        input: PathBuf,
        
        /// Output file for the proof bundle (JSON)
        #[arg(short, long)]
        output: PathBuf,
        
        /// Batch ID for this proof
        #[arg(short, long, default_value = "1")]
        batch_id: u64,
        
        /// Use mock prover (faster, for testing)
        #[arg(long)]
        mock: bool,
    },
    
    /// Verify a proof locally
    Verify {
        /// Proof bundle file (JSON)
        #[arg(short, long)]
        proof: PathBuf,
    },
    
    /// Generate a test batch with real ML-DSA signatures
    GenTest {
        /// Number of signatures to generate
        #[arg(short, long, default_value = "10")]
        count: usize,
        
        /// Output file (JSON)
        #[arg(short, long)]
        output: PathBuf,
        
        /// Include some invalid signatures for testing
        #[arg(long)]
        include_invalid: bool,
    },
    
    /// Run the aggregator server
    Serve {
        /// Port to listen on
        #[arg(short, long, default_value = "8080")]
        port: u16,
        
        /// Batch size before generating proof
        #[arg(long, default_value = "100")]
        batch_size: usize,
        
        /// Output directory for proofs
        #[arg(long, default_value = "./proofs")]
        output_dir: PathBuf,
    },
    
    /// Get verification key hash for the guest program
    VKeyHash,
}

// ═══════════════════════════════════════════════════════════════════════════
// MAIN
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(format!("reml_host={}", cli.log_level).parse()?)
        )
        .init();
    
    match cli.command {
        Commands::Prove { input, output, batch_id, mock } => {
            prove_batch(&input, &output, batch_id, mock).await?;
        }
        Commands::Verify { proof } => {
            verify_proof(&proof).await?;
        }
        Commands::GenTest { count, output, include_invalid } => {
            generate_test_batch(count, &output, include_invalid)?;
        }
        Commands::Serve { port, batch_size, output_dir } => {
            run_server(port, batch_size, output_dir).await?;
        }
        Commands::VKeyHash => {
            print_vkey_hash()?;
        }
    }
    
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// PROOF GENERATION
// ═══════════════════════════════════════════════════════════════════════════

async fn prove_batch(
    input_path: &PathBuf,
    output_path: &PathBuf,
    batch_id: u64,
    use_mock: bool,
) -> Result<()> {
    info!("Loading signature requests from {:?}", input_path);
    
    let input_json = fs::read_to_string(input_path)
        .context("Failed to read input file")?;
    let requests: Vec<SignatureRequest> = serde_json::from_str(&input_json)
        .context("Failed to parse input JSON")?;
    
    info!("Loaded {} signature requests", requests.len());
    
    let proof_input = RemlProofInput::new(requests, batch_id);
    let bundle = generate_proof(proof_input, use_mock).await?;
    
    // Save proof
    let output_json = serde_json::to_string_pretty(&bundle)
        .context("Failed to serialize proof bundle")?;
    fs::write(output_path, output_json)
        .context("Failed to write output file")?;
    
    info!("✅ Proof saved to {:?}", output_path);
    info!("   Verified: {} signatures", bundle.output.verified_count);
    info!("   Proof size: {} bytes", bundle.proof_size());
    info!("   Compression ratio: {:.1}x", bundle.compression_ratio());
    
    Ok(())
}

async fn generate_proof(input: RemlProofInput, use_mock: bool) -> Result<RemlProofBundle> {
    info!("Initializing SP1 prover client...");
    
    let client = if use_mock {
        info!("Using mock prover for faster testing");
        ProverClient::builder().mock().build()
    } else {
        info!("Using real SP1 prover (this may take a while)");
        ProverClient::from_env()
    };
    
    // Prepare stdin
    let mut stdin = SP1Stdin::new();
    stdin.write(&input);
    
    info!("Generating proof for batch {} ({} signatures)...", 
          input.batch_id, input.batch_size());
    
    // Setup
    let (pk, vk) = client.setup(GUEST_ELF);
    
    info!("Verification key hash: 0x{}", hex::encode(vk.hash_bytes()));
    
    // Generate proof
    let proof = client.prove(&pk, &stdin)
        .run()
        .context("Proof generation failed")?;
    
    // Extract output
    let output: RemlProofOutput = proof.public_values.read();
    
    // Get vkey hash
    let vkey_hash_bytes = vk.hash_bytes();
    let mut vkey_hash = [0u8; 32];
    vkey_hash.copy_from_slice(&vkey_hash_bytes[..32]);
    
    // Serialize proof
    let proof_bytes = bincode::serialize(&proof)
        .context("Failed to serialize proof")?;
    
    info!("✅ Proof generated successfully!");
    info!("   Public output: {} verified, root: 0x{}",
          output.verified_count,
          hex::encode(&output.requests_root[..8]));
    
    Ok(RemlProofBundle::new(proof_bytes, output, vkey_hash))
}

// ═══════════════════════════════════════════════════════════════════════════
// PROOF VERIFICATION
// ═══════════════════════════════════════════════════════════════════════════

async fn verify_proof(proof_path: &PathBuf) -> Result<()> {
    info!("Loading proof from {:?}", proof_path);
    
    let proof_json = fs::read_to_string(proof_path)
        .context("Failed to read proof file")?;
    let bundle: RemlProofBundle = serde_json::from_str(&proof_json)
        .context("Failed to parse proof JSON")?;
    
    info!("Proof details:");
    info!("  Batch ID: {}", bundle.output.batch_id);
    info!("  Verified signatures: {}", bundle.output.verified_count);
    info!("  Requests root: 0x{}", hex::encode(&bundle.output.requests_root[..8]));
    info!("  Proof size: {} bytes", bundle.proof_size());
    info!("  VKey hash: 0x{}", hex::encode(&bundle.vkey_hash[..8]));
    
    // Verify with SP1
    let client = ProverClient::from_env();
    let (_, vk) = client.setup(GUEST_ELF);
    
    // Check vkey matches
    let expected_vkey = vk.hash_bytes();
    if bundle.vkey_hash[..] != expected_vkey[..32] {
        bail!("VKey hash mismatch! Proof was generated with different program version.");
    }
    
    // Deserialize and verify
    let proof: sp1_sdk::SP1ProofWithPublicValues = bincode::deserialize(&bundle.proof)
        .context("Failed to deserialize proof")?;
    
    info!("Verifying proof...");
    client.verify(&proof, &vk)
        .context("Proof verification failed")?;
    
    info!("✅ Proof is VALID!");
    info!("   All {} signatures have been correctly verified in zkVM", bundle.output.verified_count);
    
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// TEST DATA GENERATION
// ═══════════════════════════════════════════════════════════════════════════

fn generate_test_batch(count: usize, output_path: &PathBuf, include_invalid: bool) -> Result<()> {
    info!("Generating {} test signatures...", count);
    
    let mut requests = Vec::with_capacity(count);
    let invalid_count = if include_invalid { count / 10 } else { 0 };
    
    for i in 0..count {
        // Generate keypair
        let (pk, sk) = dilithium2::keypair();
        
        // Create message (simulated transaction hash)
        let mut message = [0u8; 32];
        message[0..8].copy_from_slice(&(i as u64).to_le_bytes());
        // Add some entropy
        for j in 8..32 {
            message[j] = ((i * 7 + j * 13) % 256) as u8;
        }
        
        // Sign
        let signed_message = dilithium2::sign(&message, &sk);
        let signed_bytes = signed_message.as_bytes();
        
        // Extract detached signature (message is prepended)
        let signature = if i < invalid_count {
            // Create invalid signature for testing
            let mut invalid_sig = signed_bytes[32..].to_vec();
            invalid_sig[0] ^= 0xFF; // Corrupt first byte
            invalid_sig
        } else {
            signed_bytes[32..].to_vec()
        };
        
        assert_eq!(signature.len(), MLDSA_SIGNATURE_SIZE,
                   "Unexpected signature size: {} (expected {})", 
                   signature.len(), MLDSA_SIGNATURE_SIZE);
        
        let request = SignatureRequest::new(
            message,
            pk.as_bytes().to_vec(),
            signature,
            i as u64,
        );
        
        requests.push(request);
        
        if (i + 1) % 10 == 0 || i + 1 == count {
            info!("  Generated {}/{} signatures", i + 1, count);
        }
    }
    
    // Calculate expected sizes
    let raw_size = count * (32 + MLDSA_PUBLIC_KEY_SIZE + MLDSA_SIGNATURE_SIZE);
    
    // Save
    let json = serde_json::to_string_pretty(&requests)
        .context("Failed to serialize requests")?;
    fs::write(output_path, json)
        .context("Failed to write output file")?;
    
    info!("✅ Test batch saved to {:?}", output_path);
    info!("   Total signatures: {} ({} valid, {} invalid)", 
          count, count - invalid_count, invalid_count);
    info!("   Raw signature data: {} KB", raw_size / 1024);
    info!("   Expected compression: ~{:.0}x after proof generation", 
          raw_size as f64 / 50_000.0); // Rough estimate
    
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// AGGREGATOR SERVER
// ═══════════════════════════════════════════════════════════════════════════

/// Aggregator state
struct AggregatorState {
    pending_requests: Vec<SignatureRequest>,
    batch_size: usize,
    output_dir: PathBuf,
    batch_counter: u64,
}

async fn run_server(port: u16, batch_size: usize, output_dir: PathBuf) -> Result<()> {
    use tokio::net::TcpListener;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    
    info!("🚀 Starting Re-ML Aggregator Server on port {}...", port);
    info!("   Batch size: {} signatures", batch_size);
    info!("   Output directory: {:?}", output_dir);
    
    // Create output directory
    fs::create_dir_all(&output_dir)?;
    
    let state = Arc::new(RwLock::new(AggregatorState {
        pending_requests: Vec::new(),
        batch_size,
        output_dir,
        batch_counter: 0,
    }));
    
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    info!("Server listening on 0.0.0.0:{}", port);
    info!("");
    info!("Endpoints:");
    info!("  POST /submit - Submit a signature request");
    info!("  GET /status  - Get aggregator status");
    info!("  GET /batch   - Get current batch info");
    info!("");
    
    loop {
        let (mut socket, addr) = listener.accept().await?;
        let state = Arc::clone(&state);
        
        tokio::spawn(async move {
            let mut buf = vec![0u8; 65536];
            
            match socket.read(&mut buf).await {
                Ok(n) if n > 0 => {
                    let request = String::from_utf8_lossy(&buf[..n]);
                    
                    // Parse HTTP request
                    let response = if request.starts_with("POST /submit") {
                        handle_submit(&request, &state).await
                    } else if request.starts_with("GET /status") {
                        handle_status(&state).await
                    } else if request.starts_with("GET /batch") {
                        handle_batch_info(&state).await
                    } else {
                        http_response(404, "Not Found", r#"{"error": "Not found"}"#)
                    };
                    
                    if let Err(e) = socket.write_all(response.as_bytes()).await {
                        error!("Failed to write response to {}: {}", addr, e);
                    }
                }
                Ok(_) => {}
                Err(e) => {
                    error!("Failed to read from {}: {}", addr, e);
                }
            }
        });
    }
}

async fn handle_submit(request: &str, state: &Arc<RwLock<AggregatorState>>) -> String {
    // Extract body from HTTP request
    let body = request.split("\r\n\r\n").nth(1).unwrap_or("");
    
    // Parse signature request
    let sig_request: SignatureRequest = match serde_json::from_str(body) {
        Ok(r) => r,
        Err(e) => {
            return http_response(400, "Bad Request", 
                &format!(r#"{{"error": "Invalid JSON: {}"}}"#, e));
        }
    };
    
    // Validate
    if !sig_request.validate_sizes() {
        return http_response(400, "Bad Request", 
            r#"{"error": "Invalid signature or public key size"}"#);
    }
    
    let mut state = state.write().await;
    let request_id = sig_request.request_id;
    state.pending_requests.push(sig_request);
    
    let pending = state.pending_requests.len();
    let batch_size = state.batch_size;
    
    info!("Received signature request {} (pending: {}/{})", request_id, pending, batch_size);
    
    // Check if batch is complete
    if pending >= batch_size {
        // Trigger proof generation
        let requests = std::mem::take(&mut state.pending_requests);
        state.batch_counter += 1;
        let batch_id = state.batch_counter;
        let output_dir = state.output_dir.clone();
        drop(state); // Release lock before async work
        
        tokio::spawn(async move {
            info!("Batch {} complete, generating proof...", batch_id);
            
            let input = RemlProofInput::new(requests, batch_id);
            
            match generate_proof(input, false).await {
                Ok(bundle) => {
                    let output_path = output_dir.join(format!("proof_{}.json", batch_id));
                    match serde_json::to_string_pretty(&bundle) {
                        Ok(json) => {
                            if let Err(e) = fs::write(&output_path, json) {
                                error!("Failed to save proof: {}", e);
                            } else {
                                info!("✅ Proof {} saved to {:?}", batch_id, output_path);
                            }
                        }
                        Err(e) => error!("Failed to serialize proof: {}", e),
                    }
                }
                Err(e) => {
                    error!("Failed to generate proof for batch {}: {}", batch_id, e);
                }
            }
        });
        
        return http_response(200, "OK", 
            &format!(r#"{{"status": "accepted", "request_id": {}, "batch_triggered": {}}}"#, 
                     request_id, batch_id));
    }
    
    http_response(200, "OK", 
        &format!(r#"{{"status": "accepted", "request_id": {}, "pending": {}}}"#, 
                 request_id, pending))
}

async fn handle_status(state: &Arc<RwLock<AggregatorState>>) -> String {
    let state = state.read().await;
    
    let json = format!(r#"{{
  "status": "running",
  "pending_requests": {},
  "batch_size": {},
  "batches_completed": {}
}}"#, 
        state.pending_requests.len(),
        state.batch_size,
        state.batch_counter
    );
    
    http_response(200, "OK", &json)
}

async fn handle_batch_info(state: &Arc<RwLock<AggregatorState>>) -> String {
    let state = state.read().await;
    
    let request_ids: Vec<u64> = state.pending_requests.iter()
        .map(|r| r.request_id)
        .collect();
    
    let json = format!(r#"{{
  "pending_count": {},
  "batch_size": {},
  "request_ids": {:?}
}}"#,
        state.pending_requests.len(),
        state.batch_size,
        request_ids
    );
    
    http_response(200, "OK", &json)
}

fn http_response(status: u16, status_text: &str, body: &str) -> String {
    format!(
        "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        status, status_text, body.len(), body
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// UTILITY
// ═══════════════════════════════════════════════════════════════════════════

fn print_vkey_hash() -> Result<()> {
    info!("Computing verification key hash for guest program...");
    
    let client = ProverClient::from_env();
    let (_, vk) = client.setup(GUEST_ELF);
    
    let hash = vk.hash_bytes();
    
    println!();
    println!("═══════════════════════════════════════════════════════════════════");
    println!("  Re-ML Guest Program Verification Key");
    println!("═══════════════════════════════════════════════════════════════════");
    println!();
    println!("  VKey Hash (hex): 0x{}", hex::encode(&hash));
    println!();
    println!("  VKey Hash (array):");
    print!("    [");
    for (i, byte) in hash.iter().enumerate() {
        if i > 0 && i % 8 == 0 {
            print!("\n     ");
        }
        print!("0x{:02x}", byte);
        if i < hash.len() - 1 {
            print!(", ");
        }
    }
    println!("]");
    println!();
    println!("  Use this hash in runtime/src/configs/mod.rs:");
    println!();
    println!("    pub ExpectedVKeyHash: [u8; 32] = [");
    for i in (0..32).step_by(8) {
        print!("        ");
        for j in i..(i + 8).min(32) {
            print!("0x{:02x}", hash[j]);
            if j < 31 {
                print!(", ");
            }
        }
        println!();
    }
    println!("    ];");
    println!();
    println!("═══════════════════════════════════════════════════════════════════");
    
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_signature() {
        let (pk, sk) = dilithium2::keypair();
        let message = [42u8; 32];
        let signed = dilithium2::sign(&message, &sk);
        
        assert_eq!(pk.as_bytes().len(), MLDSA_PUBLIC_KEY_SIZE);
        assert!(signed.as_bytes().len() >= MLDSA_SIGNATURE_SIZE);
    }
    
    #[test]
    fn test_request_serialization() {
        let request = SignatureRequest::new(
            [0u8; 32],
            vec![1u8; MLDSA_PUBLIC_KEY_SIZE],
            vec![2u8; MLDSA_SIGNATURE_SIZE],
            1,
        );
        
        let json = serde_json::to_string(&request).unwrap();
        let parsed: SignatureRequest = serde_json::from_str(&json).unwrap();
        
        assert_eq!(parsed.request_id, 1);
        assert_eq!(parsed.public_key.len(), MLDSA_PUBLIC_KEY_SIZE);
    }
    
    #[test]
    fn test_http_response() {
        let response = http_response(200, "OK", r#"{"test": true}"#);
        assert!(response.contains("HTTP/1.1 200 OK"));
        assert!(response.contains("Content-Type: application/json"));
        assert!(response.contains(r#"{"test": true}"#));
    }
}
