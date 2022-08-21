use clap::Parser;
mod cli;

fn main() {
    let cli = cli::Cli::parse();
    
    if cli.extract {
        println!("Extracting => {files:?}", files = cli.files)
    }

    if cli.create {
        println!("Compressing => {files:?}", files = cli.files)
    }
}
