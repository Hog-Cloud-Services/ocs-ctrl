use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use self::content_index::BucketContentIndex;

mod content_index;

#[cfg(test)]
mod tests;

#[derive(Serialize, Deserialize)]
pub struct BucketMeta {
    name: String,
    content: BucketContentIndex,
}

impl BucketMeta {
    pub fn new(name: &str) -> Self {
        return BucketMeta{name: name.to_owned(), content: BucketContentIndex::new()}
    }

    pub fn add(&mut self, filename: &PathBuf) -> Vec<u8> {
        self.content.add_file(filename);
        self.content.get_checksum()
    }

    pub fn remove(&mut self, filename: &PathBuf) -> Vec<u8> {
        self.content.remove_file(filename);
        self.content.get_checksum()
    }

    pub fn rename(&mut self, filename: &PathBuf, new_name: &PathBuf) -> Vec<u8> {
        self.content.rename_file(filename, new_name);
        self.content.get_checksum()
    }
}
