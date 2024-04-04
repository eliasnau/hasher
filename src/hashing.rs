use sha2::{Digest, Sha224, Sha256, Sha384, Sha512};

pub fn show_all_hashes(data: &[u8]) -> Result<Vec<String>, String> {
    let mut hashes = Vec::new();

    let hash_224 = show_hash::<Sha224>(data)?;
    let hash_256 = show_hash::<Sha256>(data)?;
    let hash_384 = show_hash::<Sha384>(data)?;
    let hash_512 = show_hash::<Sha512>(data)?;

    hashes.push(hash_224);
    hashes.push(hash_256);
    hashes.push(hash_384);
    hashes.push(hash_512);

    Ok(hashes)
}

pub fn show_hash<D: Digest>(data: &[u8]) -> Result<String, String> {
    let hash_length = match <D as Digest>::output_size() {
        28 => "SHA-224",
        32 => "SHA-256",
        48 => "SHA-384",
        64 => "SHA-512",
        _ => return Err("Unsupported hash algorithm".to_string()),
    };

    let hash = D::digest(data);
    let hash_hex = hash
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();

    Ok(format!("{}: {}", hash_length, hash_hex))
}

pub fn check_hash(data: &[u8], expected_hash: &str) -> Option<String> {
    if expected_hash.len() == 56 {
        let hash = Sha224::digest(data);
        let computed_hash = format!("{:x}", hash);
        if computed_hash == expected_hash {
            return Some("SHA-224".to_string());
        }
    } else if expected_hash.len() == 64 {
        let hash = Sha256::digest(data);
        let computed_hash = format!("{:x}", hash);
        if computed_hash == expected_hash {
            return Some("SHA-256".to_string());
        }
    } else if expected_hash.len() == 96 {
        let hash = Sha384::digest(data);
        let computed_hash = format!("{:x}", hash);
        if computed_hash == expected_hash {
            return Some("SHA-384".to_string());
        }
    } else if expected_hash.len() == 128 {
        let hash = Sha512::digest(data);
        let computed_hash = format!("{:x}", hash);
        if computed_hash == expected_hash {
            return Some("SHA-512".to_string());
        }
    }
    None
}
