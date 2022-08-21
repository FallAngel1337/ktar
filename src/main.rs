use clap::Parser;
mod cli;

use ktar_lib::Archive;

fn main() {
    let cli = cli::Cli::parse();
    
    if cli.extract {
        Archive::extract(&cli.files[0]).unwrap_or_else(|e| {
            eprintln!("Failed to extract the archive! Error: {e}");
            std::process::exit(1);
        });
    }

    if let Some(output) = cli.create {
        Archive::create(&cli.files, output).unwrap_or_else(|e| {
            eprintln!("Failed to create the archive! Error: {e}");
            std::process::exit(1);
        });
    }
}
