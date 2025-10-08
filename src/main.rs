// src/main.rs
/*
 * Main executable for FeeEstimator
 */

use clap::Parser;
use feeestimator::{Result, run};

/// CLI arguments for FeeEstimator
#[derive(Parser)]
#[command(version, about = "FeeEstimator - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Path to input file
    #[arg(short = 'i', long, default_value = "")]
    input: Option<String>,
    
    /// Path to output file
    #[arg(short = 'o', long, default_value = "")]
    output: Option<String>,
}

fn main() -> Result<()> {
    // Parse CLI arguments
    let args = Cli::parse();
    
    // Run FeeEstimator with parsed arguments
    run(args.verbose, args.input, args.output)
}