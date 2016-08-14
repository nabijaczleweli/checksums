extern crate md5;
extern crate crc;
#[macro_use]
extern crate clap;
extern crate crc8;
extern crate crc16;
extern crate shaman;
extern crate tabwriter;
extern crate blake2_rfc;
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
        // todo
        println!("Verification unimplemented!");
        println!("{:#?}", hashes);
    } else {
        ops::write_hashes(&opts.file, opts.algorithm, hashes);
    }
}
