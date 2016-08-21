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
use std::path::{Path, PathBuf};
use self::super::util::mul_str;
use self::super::Algorithm;
use tabwriter::TabWriter;
use std::fs::File;
use regex::Regex;

pub use self::compare::*;
pub use self::create::*;
pub use self::write::*;


/// Serialise the specified hashes to the specified output file.
pub fn write_hashes(out_file: &(String, PathBuf), algo: Algorithm, mut hashes: BTreeMap<String, String>) {
    let mut out = TabWriter::new(File::create(&out_file.1).unwrap());

    hashes.insert(out_file.0.clone(), mul_str("-", algo.size()));
    for (fname, hash) in hashes {
        writeln!(&mut out, "{}\t{}", fname, hash).unwrap();
    }

    out.flush().unwrap();
}

/// Read hashes saved with `write_hashes()` from the specified path or fail with line numbers not matching pattern.
pub fn read_hashes(in_file: &Path) -> Result<BTreeMap<String, String>, Vec<usize>> {
    lazy_static! {
        static ref LINE_RGX: Regex = Regex::new(r"^(.+?)\s{2,}([[:xdigit:]-]+)$").unwrap();
    }

    let mut hashes = BTreeMap::new();
    let mut wrong_lines = Vec::new();

    let in_file = BufReader::new(File::open(in_file).unwrap());
    for (n, line) in in_file.lines().map(Result::unwrap).enumerate() {
        if !line.is_empty() {
            match LINE_RGX.captures(&line) {
                Some(captures) => {
                    hashes.insert(captures[1].to_string(), captures[2].to_string());
                }
                None => wrong_lines.push(n),
            };
        }
    }

    if wrong_lines.is_empty() {
        Ok(hashes)
    } else {
        Err(wrong_lines)
    }
}
