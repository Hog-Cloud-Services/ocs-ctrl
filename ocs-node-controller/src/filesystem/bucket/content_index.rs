use std::{collections::{BTreeMap, BTreeSet}, path::PathBuf};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
pub struct BucketContentIndex {
    files: BTreeSet<String>,
    subdirs: BTreeMap<String, Box<BucketContentIndex>>,
    own_checksum: Vec<u8>
}

impl BucketContentIndex {
    fn strip_oldest_dir(&self, path: &PathBuf) -> Option<(PathBuf, String)> {
        let mut piter = path.components();
        let ancestor = piter.next()?.as_os_str().to_str()?.to_owned();
        Some((piter.as_path().to_path_buf(), ancestor))
    }

    fn calculate_hash(&self, children_content: &[u8], self_content: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(children_content);
        hasher.update(self_content);
        hasher.finalize().to_vec()
    }

    fn recalculate_checksum(&mut self) {
        let child_sum = self.subdirs.values()
            .map(|child_index| {
                return child_index.own_checksum.clone();
            })
            .reduce(|acc, e| {
                acc.iter()
                    .zip(e)   
                    .map(|(a, b)| {
                        a^b
                    })
                    .collect()
            });
        dbg!(&child_sum);
        let own_sum = self.files.iter()
            .map(|e| {
                e.as_bytes().to_vec()
            })
            .reduce(|acc, e| {
                acc.iter()
                    .zip(e)
                    .map(|(a, b)| {
                        a^b
                    })
                    .collect()
            });
        self.own_checksum = self.calculate_hash(&child_sum.unwrap_or_default(), &own_sum.unwrap_or_default());
        dbg!(&self.own_checksum);
    }

    pub fn new() -> Self {
        return BucketContentIndex { files: BTreeSet::new(), subdirs: BTreeMap::new(), own_checksum: Vec::new()}
    }

    pub fn add_file(&mut self, name: &PathBuf) {
        if name.parent() == None {
            self.files.insert(name.to_str().unwrap().to_string());
        } else {
            let (child_path, subdirectory) = self.strip_oldest_dir(name).unwrap();
            let child_index = self.subdirs.entry(subdirectory).or_insert(Box::new(BucketContentIndex::new()));
            child_index.add_file(&child_path);
        }
        self.recalculate_checksum();
    }

    pub fn remove_file(&mut self, name: &PathBuf) {
        if name.parent() == None {
            self.files.remove(name.to_str().unwrap());
        } else {
            let (child_path, subdirectory) = self.strip_oldest_dir(name).unwrap();
            let child_index = self.subdirs.get_mut(&subdirectory);
            if let Some(child) = child_index {
                child.remove_file(&child_path);
                if child.subdirs.is_empty() {
                    self.subdirs.remove(&subdirectory);
                }
            }
        }
        self.recalculate_checksum();
    }

    pub fn rename_file(&mut self, file: &PathBuf, new_name: &PathBuf) {
        self.remove_file(file);
        self.add_file(new_name);
    }

    pub fn get_checksum(&self) -> Vec<u8> {
        return self.own_checksum.clone();
    }
}
