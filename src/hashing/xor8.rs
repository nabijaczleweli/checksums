use std::io::{BufReader, Read};
use self::super::hash_string;


// Pseudocode: https://en.wikipedia.org/wiki/Longitudinal_redundancy_check
pub fn hash<R: Read>(reader: &mut R) -> String {
    let mut lrc = 0u16;
    for b in BufReader::new(reader).bytes() {
        lrc = (lrc + b.unwrap() as u16) & 0xFF;
    }
    let lrc = (((lrc ^ 0xFF) + 1) & 0xFF) as u8;

    hash_string(&[lrc])
}
