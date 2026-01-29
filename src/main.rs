use anyhow::Result;
use clap::Parser;

/// A template Rust CLI application
#[derive(Parser, Debug)]
#[command(name = "rust-cli-template")]
#[command(version, about, long_about = None)]
struct Cli {
    /// Name to greet
    #[arg(short, long, default_value = "World")]
    name: String,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.verbose {
        println!("Running in verbose mode...");
    }

    println!("Hello, {}!", cli.name);

    Ok(())
}
