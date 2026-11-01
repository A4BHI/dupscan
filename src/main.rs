use clap::Parser;
use hex;
// use sha2::digest::typenum::Len;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
// use std::str::Bytes;
use std::io;
use std::{
    collections::HashMap,
    fs::{self},
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

fn calculate_hash(
    files: &Vec<PathBuf>,
    file_hash_map: &mut HashMap<String, Vec<PathBuf>>,
) -> io::Result<()> {
    for path in files {
        let mut file = File::open(&path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let hash = hex::encode(Sha256::digest(&buffer));

        file_hash_map
            .entry(hash.clone()) //used clone because without clone the hash values ownership goes to entry() and later we cant println!() hash
            .or_insert(Vec::new())
            .push(path.clone());
        // println!("{:?}", hash);
    }
    Ok(())
}

fn display_duplicates(folder: &PathBuf, file_hash_map: &HashMap<String, Vec<PathBuf>>) {
    let mut flag = false;
    for (hash, files) in file_hash_map {
        println!("====== Duplicate Files Found In : {:?} ======", folder);
        println!();
        if files.len() > 1 {
            flag = true;
            for f in files {
                let metadata = fs::metadata(f).expect("Error reading file metadata");
                println!(
                    "[Filepath ==> File Size ==> Hash] : {:?} ==> {} ==> {} ",
                    f,
                    format_size(&metadata.len()),
                    hash
                );
            }
        }
    }
    if !flag {
        println!("=========No Duplicate Files Found =========");
    }
}

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

        // println!("{}", metadata.len());
        files_map
            .entry(metadata.len())
            .or_insert(Vec::new())
            .push(path);
    }

    for (_key, files) in &files_map {
        if files.len() > 1 {
            let _ = calculate_hash(files, &mut file_hash_map);
        }
    }

    display_duplicates(&folder, &file_hash_map);
    // println!("{:?}", folder);

    Ok(())
}
