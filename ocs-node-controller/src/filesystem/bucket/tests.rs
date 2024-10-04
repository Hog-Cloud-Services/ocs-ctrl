use std::str::FromStr;

use super::*;

#[test]
fn add_file() {
    let mut index = BucketMeta::new("test1");
    let checksum1 = index.add(&PathBuf::from_str("stuff.txt").unwrap());
    let checksum2 = index.add(&PathBuf::from_str("first/stuff.txt").unwrap());
    let checksum3 = index.add(&PathBuf::from_str("second/ny.txt").unwrap());
    let checksum4 = index.add(&PathBuf::from_str("first/stuff_other.txt").unwrap());
    _ = index.add(&PathBuf::from_str("stuff_other.txt").unwrap());

    assert_ne!(checksum1, checksum2, "First checksums are different");
    assert_ne!(checksum3, checksum4, "Othes checksums are different");
}

#[test]
fn add_and_remove_file() {
    let mut index = BucketMeta::new("test2");
    let checksum1 = index.add(&PathBuf::from_str("stuff.txt").unwrap());
    let checksum2 = index.remove(&PathBuf::from_str("nonexistent.txt").unwrap());
    assert_eq!(checksum1, checksum2, "Removing nonexistent file should yield the same checksum");
    _ = index.add(&PathBuf::from_str("test/stuff2.txt").unwrap());
    let checksum3 = index.remove(&PathBuf::from_str("test/stuff2.txt").unwrap());
    assert_eq!(checksum1, checksum3, "Addition and removal should not change checksum");
}

#[test]
fn rename_file() {
    let mut index = BucketMeta::new("test2");
    _ = index.add(&PathBuf::from_str("stuff.txt").unwrap());
    let checksum1 = index.rename(&PathBuf::from_str("stuff.txt").unwrap(), &PathBuf::from_str("stuff_other.txt").unwrap());
    let checksum2 = index.add(&PathBuf::from_str("stuff_other.txt").unwrap());
    assert_eq!(checksum1, checksum2, "Adding to renamed file should yield the same checksum")
}
