use clap::Parser;
use std::{
    collections::HashMap,
    path::{self, PathBuf},
};
#[derive(Parser)]
struct Args {
    path: Option<PathBuf>,

    #[arg(short, long)]
    recursive: bool,
}
fn main() {
    let args = Args::parse();
    let folder = args.path.unwrap_or_else(|| match std::env::current_dir() {
        Ok(path) => path,
        Err(err) => {
            println!("Error {}", err);
            std::process::exit(1);
        }
    });

    let mut map: HashMap<u64, Vec<PathBuf>> = HashMap::new();
    println!("{:?}", folder);
}
