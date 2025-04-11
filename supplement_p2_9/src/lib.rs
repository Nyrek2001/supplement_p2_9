use std::fs::File;
use std::io::{Read, Result};
use std::path::Path;
use sha2::{Sha256, Digest};

/// Computes the SHA-256 hash of a file's contents.
pub fn hash_file(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 1024];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let hash = hasher.finalize();
    Ok(format!("{:x}", hash))
}

use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{Write, BufReader};
use std::path::Path;
use serde::{Serialize, Deserialize};

const HASH_STORE: &str = "saved_hashes.json";

#[derive(Serialize, Deserialize)]
struct HashStore {
    hashes: HashMap<String, String>,
}

impl HashStore {
    fn load() -> Self {
        if let Ok(file) = File::open(HASH_STORE) {
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or(Self { hashes: HashMap::new() })
        } else {
            Self { hashes: HashMap::new() }
        }
    }

    fn save(&self) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(&self)?;
        fs::write(HASH_STORE, json)
    }
}

pub fn save_hash(path: &Path, hash: &str) -> std::io::Result<()> {
    let mut store = HashStore::load();
    store.hashes.insert(path.to_string_lossy().into(), hash.to_string());
    store.save()
}

pub fn load_hash(path: &Path) -> std::io::Result<Option<String>> {
    let store = HashStore::load();
    Ok(store.hashes.get(&path.to_string_lossy().into()).cloned())
}

pub fn clear_saved_hash(path: &Path) -> std::io::Result<()> {
    let mut store = HashStore::load();
    store.hashes.remove(&path.to_string_lossy().into());
    store.save()
}
