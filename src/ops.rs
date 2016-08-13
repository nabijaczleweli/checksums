//! Main functions doing actual work.
//!
//!
//! Use `create_hashes()` to prepare the hashes for a path.
//!
//! Then use `write_hashes` to save it to disk, or
//! `read_hashes` to get the saved hashes and compare them.


use tabwriter::TabWriter;
use self::super::options::DepthSetting;
use self::super::Algorithm;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::iter;
use std::fs::{self, File};
use std::io::Write;


/// Create subpath->hash mappings for a given path using a given algorithm up to a given depth.
pub fn create_hashes(path: &PathBuf, algo: Algorithm, remaining_depth: DepthSetting) -> BTreeMap<String, String> {
    let mut hashes = BTreeMap::new();

    for file in fs::read_dir(&path).unwrap() {
        let file = file.unwrap();
        let file_type = file.file_type().unwrap();
        let file_name_s = file.file_name().into_string().unwrap();

        if file_type.is_file() {
            hashes.insert(file_name_s, format!("{:?}", remaining_depth));  // todo
        } else if remaining_depth.can_recurse() {
            let mut subpath = path.clone();
            subpath.push(file.path());

            // TODO: replace with passing prefixes as argument and `append()`ing directly once `btree_append` is stabilised
            for kv in create_hashes(&subpath, algo, remaining_depth.next_level().unwrap()) {
                hashes.insert(format!("{}/{}", file_name_s, kv.0), kv.1);
            }
        }
    }

    hashes
}

/// Serialise the specified hashes to the specified output file.
pub fn write_hashes(out_file: &(String, PathBuf), algo: Algorithm, mut hashes: BTreeMap<String, String>) {
    let mut out = TabWriter::new(File::create(&out_file.1).unwrap());

    hashes.insert(out_file.0.clone(), iter::repeat("-").take(algo.size()).collect::<String>());
    for (fname, hash) in hashes {
        writeln!(&mut out, "{}\t{}", fname, hash).unwrap();
    }

    out.flush().unwrap();
}
