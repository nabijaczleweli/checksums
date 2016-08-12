#[macro_use]
extern crate clap;

mod algorithms;

pub mod ops;
pub mod options;

pub use algorithms::Algorithm;


fn main() {
    let opts = options::Options::parse();

    println!("{:?}", opts);
    let hashes = ops::create_hashes(&opts.dir, opts.algorithm, opts.depth);
    println!("{:#?}", hashes);
    if opts.verify {
        // todo
    } else {
        //ops::create(&opts.dir, opts.algorithm, opts.depth)
    }
}
