//! Main functions doing actual work.
//!
//!
//! Use `create_hashes()` to prepare the hashes for a path.
//!
//! Then use `write_hashes()` to save it to disk, or `read_hashes()` to get the saved hashes and compare them with
//! `compare_hashes()`.


mod compare;

use tabwriter::TabWriter;
use self::super::Algorithm;
use self::super::hash_file;
use std::io::{BufRead, BufReader, Write};
use self::super::options::DepthSetting;
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::path::PathBuf;
use regex::Regex;
use std::iter;

pub use self::compare::compare_hashes;


/// Create subpath->hash mappings for a given path using a given algorithm up to a given depth.
pub fn create_hashes(path: &PathBuf, algo: Algorithm, remaining_depth: DepthSetting) -> BTreeMap<String, String> {
    let mut hashes = BTreeMap::new();

    for file in fs::read_dir(&path).unwrap() {
        let file = file.unwrap();
        let file_type = file.file_type().unwrap();
        let file_name_s = file.file_name().into_string().unwrap();

        let mut subpath = path.clone();
        subpath.push(file.path());

        if file_type.is_file() {
            hashes.insert(file_name_s, hash_file(&subpath, algo));
        } else if remaining_depth.can_recurse() {
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

/// Read hashes saved with `write_hashes()` from the specified path.
pub fn read_hashes(in_file: &PathBuf) -> BTreeMap<String, String> {
    lazy_static! {
        static ref LINE_RGX: Regex = Regex::new(r"^(.+?)\s{2,}([[:xdigit:]-]+)$").unwrap();
    }

    let mut hashes = BTreeMap::new();

    let in_file = BufReader::new(File::open(in_file).unwrap());
    for line in in_file.lines().map(Result::unwrap) {
        if !line.is_empty() {
            // TODO: check if succeeded and return line number and error
            let captures = LINE_RGX.captures(&line).unwrap();
            hashes.insert(captures[1].to_string(), captures[2].to_string());
        }
    }

    hashes
}
