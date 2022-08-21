use std::path::PathBuf;
use clap::{Parser, ValueHint};

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short = 'x', name = "dest", help = "Extract *.tar file(s)")]
    pub extract: bool,
    
    #[clap(short = 't', name = "output", help = "Create a tar file called <output>")]
    pub create: Option<PathBuf>,

    #[clap(name = "FILE", parse(from_os_str), value_hint = ValueHint::AnyPath, required = true)]
    pub files: Vec<PathBuf>,
}