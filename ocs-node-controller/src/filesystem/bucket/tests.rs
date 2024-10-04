use std::str::FromStr;

use super::*;

#[test]
fn add_file() {
    let mut index = BucketMeta::new("test1");
    let checksum1 = index.add(&PathBuf::from_str("stuff.txt").unwrap());
    let checksum2 = index.add(&PathBuf::from_str("first/stuff.txt").unwrap());
    let checksum3 = index.add(&PathBuf::from_str("second/ny.txt").unwrap());
    let checksum4 = index.add(&PathBuf::from_str("first/stuff_other.txt").unwrap());
    let checksum5 = index.add(&PathBuf::from_str("stuff_other.txt").unwrap());

    assert_ne!(checksum1, checksum2, "First checksums are different");
    assert_ne!(checksum3, checksum4, "Othes checksums are different");
}
