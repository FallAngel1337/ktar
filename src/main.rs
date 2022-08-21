use clap::Parser;
mod cli;

use ktar_lib::Archive;

fn main() {
    let cli = cli::Cli::parse();
    
    if let Some(dest) = cli.extract {
        println!("{dest:?} => {files:?}", files = cli.files);
    }

    if let Some(output) = cli.create {
        Archive::create(&cli.files, output).unwrap_or_else(|e| {
            eprintln!("Failed to create the archive! Error: {e}");
            std::process::exit(1);
        });
    }
}
