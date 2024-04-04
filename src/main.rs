use clap::{error::ErrorKind, CommandFactory, Parser};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

mod hashing;

#[derive(Parser, Debug)]
#[clap(version)]
#[clap(author)]
#[clap(about)]
#[clap(long_about = None)]
#[command(name = "hasher", author, version, about)]
struct Args {
    /// The path to the File
    #[clap(short, long)]
    filename: PathBuf,

    /// The expected hash you want to check
    #[clap(short, long)]
    check: Option<String>,

    /// Get a specific hashing Algorithm
    #[clap(short, long)]
    algorithm: Option<String>,
}

fn main() {
    let args = Args::parse();

    let filename = &args.filename;
    let mut file = match File::open(&filename) {
        Ok(file) => file,
        Err(_) => {
            let mut cmd = Args::command();
            cmd.error(
                ErrorKind::Io,
                format!("Error: Unable to open file '{}'", filename.display()),
            )
            .exit();
        }
    };

    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Ok(_) => {}
        Err(_) => {
            let mut cmd = Args::command();
            cmd.error(
                ErrorKind::Io,
                format!("Error: Unable to read file '{}': ", filename.display()),
            )
            .exit();
        }
    }

    if let Some(hash) = &args.check {
        match hashing::check_hash(&buffer, hash) {
            Some(hash_algorithm) => {
                println!("Hash match: {} ({})", hash, hash_algorithm);
            }
            None => {
                println!(
                    "Hash does not match or unsupported hash algorithm: {}",
                    hash
                );
            }
        }
    } else if let Some(algorithm) = &args.algorithm {
        // Show only the specified hash algorithm
        match algorithm.to_lowercase().as_str() {
            "sha224" => {
                if let Ok(hash) = hashing::show_hash::<sha2::Sha224>(&buffer) {
                    println!("{}", hash);
                } else {
                    let mut cmd = Args::command();
                    cmd.error(ErrorKind::Io, format!("Error generating sha224 hash\n\t"))
                        .exit();
                }
            }
            "sha256" => {
                if let Ok(hash) = hashing::show_hash::<sha2::Sha256>(&buffer) {
                    println!("{}", hash);
                } else {
                    let mut cmd = Args::command();
                    cmd.error(ErrorKind::Io, format!("Error generating sha256 hash\n\t"))
                        .exit();
                }
            }
            "sha384" => {
                if let Ok(hash) = hashing::show_hash::<sha2::Sha384>(&buffer) {
                    println!("{}", hash);
                } else {
                    let mut cmd = Args::command();
                    cmd.error(ErrorKind::Io, format!("Error generating sha384 hash\n\t"))
                        .exit();
                }
            }
            "sha512" => {
                if let Ok(hash) = hashing::show_hash::<sha2::Sha512>(&buffer) {
                    println!("{}", hash);
                } else {
                    let mut cmd = Args::command();
                    cmd.error(ErrorKind::Io, format!("Error generating sha512 hash\n\t"))
                        .exit();
                }
            }
            _ => println!("Error: Unsupported hash algorithm"),
        }
    } else {
        // Show all hashes
        match hashing::show_all_hashes(&buffer) {
            Ok(hashes) => {
                for hash in hashes {
                    println!("{}", hash);
                }
            }
            Err(err) => {
                let mut cmd = Args::command();
                cmd.error(ErrorKind::Io, format!("Error creating Hashes\n\t{}", err))
                    .exit();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_show_hash_success() {
        let buffer: &[u8] = &b"test data"[..];
        if let Ok(hash) = hashing::show_hash::<sha2::Sha512>(&buffer) {
            assert_eq!(hash, "SHA-512: 0e1e21ecf105ec853d24d728867ad70613c21663a4693074b2a3619c1bd39d66b588c33723bb466c72424e80e3ca63c249078ab347bab9428500e7ee43059d0d");
        // Replace "expected_hash_value" with the actual expected hash value
        } else {
            panic!("Expected Ok(hash) but got an error");
        }
    }
}
