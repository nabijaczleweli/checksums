//! Main functions doing actual work.
//!
//!
//! Use `create_hashes()` to prepare the hashes for a path.
//!
//! Then use `write_hashes` to save it to disk, or
//! `read_hashes` to get the saved hashes and compare them.

use self::super::options::DepthSetting;
use self::super::Algorithm;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::fs;


/// Create subpath->hash mappings for a given path using a given algorithm up to a given depth.
pub fn create_hashes(path: &PathBuf, algo: Algorithm, remaining_depth: DepthSetting) -> BTreeMap<String, String> {
    let mut hashmap = BTreeMap::new();

    for file in fs::read_dir(&path).unwrap() {
        let file = file.unwrap();
        let file_type = file.file_type().unwrap();
        let file_name_s = file.file_name().into_string().unwrap();

        if file_type.is_dir() && remaining_depth.can_recurse() {
            let mut subpath = path.clone();
            subpath.push(file.path());

            // TODO: replace with passing prefixes as argument and `append()`ing directly once `btree_append` is stabilised
            for kv in create_hashes(&subpath, algo, remaining_depth.next_level().unwrap()) {
                hashmap.insert(format!("{}/{}", file_name_s, kv.0), kv.1);
            }
        } else if file_type.is_file() {
            hashmap.insert(file_name_s, format!("{:?}", remaining_depth));  // todo
        }
    }

    hashmap
}
