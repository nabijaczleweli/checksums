extern crate checksums;

use std::process::exit;
use std::io::{stdout, stderr};


fn main() {
    let result = actual_main();
    exit(result);
}

fn actual_main() -> i32 {
    let opts = checksums::Options::parse();

    let hashes = checksums::ops::create_hashes(&opts.dir,
                                               opts.ignored_files,
                                               opts.algorithm,
                                               opts.depth,
                                               opts.follow_symlinks,
                                               opts.jobs,
                                               stdout(),
                                               &mut stderr());
    if opts.verify {
        // Progress bar separator
        println!("");

        match checksums::ops::read_hashes(&mut stderr(), &opts.file) {
            Ok(loaded_hashes) => {
                let compare_result = checksums::ops::compare_hashes(&opts.file.0, hashes, loaded_hashes);
                checksums::ops::write_hash_comparison_results(&mut stdout(), &mut stderr(), compare_result)
            }
            Err(rval) => rval,
        }.exit_value()
    } else {
        checksums::ops::write_hashes(&opts.file, opts.algorithm, hashes);
        0
    }
}
