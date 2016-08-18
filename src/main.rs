extern crate checksums;

use std::process::exit;
use std::io::{stdout, stderr};


fn main() {
    let result = actual_main();
    exit(result);
}

fn actual_main() -> i32 {
    let opts = checksums::options::Options::parse();

    let hashes = checksums::ops::create_hashes(&opts.dir, opts.ignored_files, opts.algorithm, opts.depth, opts.follow_symlinks);
    if opts.verify {
        let loaded_hashes = checksums::ops::read_hashes(&opts.file.1);

        let compare_result = checksums::ops::compare_hashes(&opts.file.0, hashes, loaded_hashes);
        checksums::ops::write_hash_comparison_results(&mut stdout(), &mut stderr(), compare_result)
    } else {
        checksums::ops::write_hashes(&opts.file, opts.algorithm, hashes);
        0
    }
}
