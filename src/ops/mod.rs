//! Main functions doing actual work.
//!
//!
//! Use `create_hashes()` to prepare the hashes for a path.
//!
//! Then use `write_hashes()` to save it to disk, or `read_hashes()` to get the saved hashes, them with
//! `compare_hashes()` and print them with `write_hash_comparison_results()`.


mod compare;
mod create;
mod write;

use std::io::{BufRead, BufReader, Write};
use std::collections::BTreeMap;
use self::super::Algorithm;
use tabwriter::TabWriter;
use std::path::PathBuf;
use std::fs::File;
use regex::Regex;
use std::iter;

pub use self::compare::*;
pub use self::create::*;
pub use self::write::*;


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
