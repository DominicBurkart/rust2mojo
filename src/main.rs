//! Rust to Mojo Compiler CLI
//!
//! Command-line interface for the rust2mojo compiler.

use clap::{Parser, Subcommand};
use rust2mojo::{Compiler, Result};
use std::path::PathBuf;
use tracing::{error, info};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a Rust file to Mojo
    Compile {
        /// Input Rust file
        #[arg(value_name = "FILE")]
        input: PathBuf,
        
        /// Output Mojo file (defaults to input.mojo)
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Print output to stdout instead of writing to file
        #[arg(long)]
        stdout: bool,
    },
    
    /// Check if Rust code can be compiled without generating output
    Check {
        /// Input Rust file
        #[arg(value_name = "FILE")]
        input: PathBuf,
    },
    
    /// Show version information
    Version,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize tracing
    let filter = if cli.verbose {
        "rust2mojo=debug"
    } else {
        "rust2mojo=info"
    };
    
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();

    match cli.command {
        Commands::Compile { input, output, stdout } => {
            info!("Compiling Rust file: {:?}", input);
            
            let compiler = Compiler::new();
            let mojo_code = compiler.compile_file(&input)?;
            
            if stdout {
                print!("{}", mojo_code);
            } else {
                let output_path = output.unwrap_or_else(|| {
                    input.with_extension("mojo")
                });
                
                std::fs::write(&output_path, mojo_code)
                    .map_err(|e| rust2mojo::Error::IoError(format!("Failed to write output: {}", e)))?;
                
                info!("Generated Mojo code: {:?}", output_path);
            }
        }
        
        Commands::Check { input } => {
            info!("Checking Rust file: {:?}", input);
            
            let compiler = Compiler::new();
            let _mojo_code = compiler.compile_file(&input)?;
            
            info!("✓ Rust code can be successfully compiled to Mojo");
        }
        
        Commands::Version => {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            println!("Rust to Mojo compiler");
        }
    }
    
    Ok(())
}
