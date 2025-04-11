use file_hash_checker::hash_file;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use file_hash_checker::{save_hash, load_hash, clear_saved_hash};
use std::path::Path;

#[test]
fn test_hash_file_consistency() {
    let path = "test_file.txt";
    let mut file = File::create(path).unwrap();
    write!(file, "Hello, world!").unwrap();

    let hash1 = hash_file(Path::new(path)).unwrap();
    let hash2 = hash_file(Path::new(path)).unwrap();

    assert_eq!(hash1, hash2);

    std::fs::remove_file(path).unwrap();
}

#[test]
fn test_save_and_load_hash() {
    let file_path = Path::new("sample.txt");
    let hash = "abc123";

    save_hash(file_path, hash).unwrap();
    let loaded = load_hash(file_path).unwrap();

    assert_eq!(Some(hash.to_string()), loaded);

    clear_saved_hash(file_path).unwrap();
}
