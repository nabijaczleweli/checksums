#[macro_use]
extern crate clap;

mod options;
mod algorithms;

pub use options::Options;
pub use algorithms::Algorithm;


fn main() {
    let opts = Options::parse();

    println!("{:?}", opts);
}
