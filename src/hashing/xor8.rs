use self::super::hash_string;
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufReader, Read};


// Pseudocode: https://en.wikipedia.org/wiki/Longitudinal_redundancy_check
pub fn hash(path: &PathBuf) -> String {
    let mut lrc = 0u16;
    for b in BufReader::new(File::open(path).unwrap()).bytes() {
        lrc = (lrc + b.unwrap() as u16) & 0xFF;
    }
    let lrc = (((lrc ^ 0xFF) + 1) & 0xFF) as u8;

    hash_string(&[lrc])
}
