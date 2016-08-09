extern crate md5;
extern crate crc;
#[macro_use]
extern crate clap;
extern crate crc8;
extern crate crc16;
extern crate blake;
extern crate regex;
extern crate shaman;
extern crate tabwriter;
extern crate blake2_rfc;
#[macro_use]
extern crate lazy_static;
extern crate tiny_keccak;

mod hashing;
mod algorithms;

pub mod ops;
pub mod options;

pub use hashing::hash_file;
pub use algorithms::Algorithm;


fn main() {
    let opts = options::Options::parse();

    let hashes = ops::create_hashes(&opts.dir, opts.algorithm, opts.depth);
    if opts.verify {
        let loaded_hashes = ops::read_hashes(&opts.file.1);
        ops::compare_hashes(&opts.file.0, hashes, loaded_hashes);
    } else {
        ops::write_hashes(&opts.file, opts.algorithm, hashes);
    }
}
