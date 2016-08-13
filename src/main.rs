#[macro_use]
extern crate clap;
extern crate tabwriter;

mod algorithms;

pub mod ops;
pub mod options;

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
