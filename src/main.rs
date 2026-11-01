use clap::Parser;
use std::{collections::HashMap, path::PathBuf};
#[derive(Parser)]
struct Args {
    path: Option<PathBuf>,

    #[arg(short, long)]
    recursive: bool,
}
fn main() {
    let mut map: HashMap<u64, Vec<PathBuf>> = HashMap::new();
}
