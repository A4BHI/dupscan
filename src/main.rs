use clap::Parser;
use sha2::digest::typenum::Len;
use std::{
    collections::HashMap,
    fs::{self, Metadata},
    path::{Path, PathBuf},
};
#[derive(Parser)]
struct Args {
    path: Option<PathBuf>,

    #[arg(short, long)]
    recursive: bool,
}

fn format_size(len: &u64) -> String {
    let kb = 1024.0;
    let mb = kb * 1024.0;
    let gb = mb * 1024.0;

    let size = *len as f64;

    if size >= gb {
        format!("{:.2} GB", size / gb)
    } else if size >= mb {
        format!("{:.2} MB", size / mb)
    } else if size >= kb {
        format!("{:.2} KB", size / kb)
    } else {
        format!("{:.2} B", size)
    }
}

fn calculate_hash(files: &Vec<PathBuf>, file_hash_map: &mut HashMap<String, Vec<PathBuf>>) {}

fn main() -> Result<(), std::io::Error> {
    let mut files_map: HashMap<u64, Vec<PathBuf>> = HashMap::new();
    let mut file_hash_map: HashMap<String, Vec<PathBuf>> = HashMap::new();
    let args = Args::parse();
    let folder = args.path.unwrap_or_else(|| match std::env::current_dir() {
        Ok(path) => path,
        Err(err) => {
            println!("Error {}", err);
            std::process::exit(1);
        }
    });

    let open_folder = Path::read_dir(&folder).expect("Error reading the folder");
    for entries in open_folder {
        let path = entries.unwrap().path();
        let metadata = fs::metadata(&path)?;
        println!("{}", metadata.len());
        map.entry(metadata.len()).or_insert(Vec::new()).push(path);
    }

    for (key, files) in &map {
        if files.len() > 1 {}
    }
    println!("{:?}", folder);

    Ok(())
}
