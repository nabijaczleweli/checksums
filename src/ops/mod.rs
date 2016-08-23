//! Main functions doing actual work.
//!
//!
//! Use `create_hashes()` to prepare the hashes for a path.
//!
//! Then use `write_hashes()` to save it to disk, or `read_hashes()` to get the saved hashes, them with
//! `compare_hashes()` and print them with `write_hash_comparison_results()`.


mod compare;
mod write;

use std::collections::{BTreeSet, BTreeMap};
use std::io::{BufRead, BufReader, Write};
use self::super::{Algorithm, hash_file};
use walkdir::{WalkDir, WalkDirIterator};
use futures::{Future, Task, Poll};
use self::super::util::mul_str;
use futures_cpupool::CpuPool;
use tabwriter::TabWriter;
use std::time::Duration;
use std::path::PathBuf;
use pbr::ProgressBar;
use std::fs::File;
use regex::Regex;
use std::thread;

pub use self::compare::*;
pub use self::write::*;


lazy_static! {
    // TODO: customisation point?
    static ref SLEEP_LEN: Duration = Duration::from_millis(1);
}


/// Create subpath->hash mappings for a given path using a given algorithm up to a given depth.
pub fn create_hashes<W>(path: &PathBuf, ignored_files: BTreeSet<String>, algo: Algorithm, depth: Option<usize>, follow_symlinks: bool, jobs: u32, pb_out: W)
                        -> BTreeMap<String, String>
    where W: Write
{
    let mut walkdir = WalkDir::new(path).follow_links(follow_symlinks);
    if let Some(depth) = depth {
        walkdir = walkdir.max_depth(depth + 1);
    }

    let mut hashes = BTreeMap::new();
    let mut hashes_f = BTreeMap::new();

    let pool = CpuPool::new(jobs);
    let mut walkdir = walkdir.into_iter();
    while let Some(entry) = walkdir.next() {
        // TODO: don't panic on symlink loops
        let entry = entry.unwrap();
        let ignored = entry.file_name().to_str().map(|f| ignored_files.contains(f)).unwrap_or(false);

        if entry.file_type().is_file() {
            let filename_string = entry.path().strip_prefix(path).unwrap().to_str().unwrap().to_string();

            if ignored {
                hashes.insert(filename_string, mul_str("-", algo.hexlen()));
            } else {
                hashes_f.insert(filename_string, pool.execute(move || hash_file(entry.path(), algo)));
            }
        } else if entry.file_type().is_dir() && ignored {
            walkdir.skip_current_dir();
        }
    }

    let mut pb = ProgressBar::on(pb_out, hashes_f.len() as u64);
    pb.set_width(Some(80));
    pb.show_speed = false;
    pb.show_tick = true;

    hashes.extend(hashes_f.into_iter()
        .map(|(k, mut f)| {
            pb.message(&format!("{} ", k));
            pb.inc();

            let mut task = Task::new();

            for i in 0.. {
                match f.poll(&mut task) {
                    Poll::NotReady => {
                        thread::sleep(*SLEEP_LEN);
                        if i % 100 == 0 {
                            pb.tick();
                        }
                    }
                    Poll::Ok(result) => return (k, result),
                    Poll::Err(error) => panic!("Failed to hash file \"{}\": {:?}", k, error),
                }
            }

            unreachable!();
        }));

    pb.show_tick = false;
    pb.tick();
    pb.finish_print("");
    hashes
}

/// Serialise the specified hashes to the specified output file.
pub fn write_hashes(out_file: &(String, PathBuf), algo: Algorithm, mut hashes: BTreeMap<String, String>) {
    let mut out = TabWriter::new(File::create(&out_file.1).unwrap());

    hashes.insert(out_file.0.clone(), mul_str("-", algo.hexlen()));
    for (fname, hash) in hashes {
        writeln!(&mut out, "{}\t{}", fname, hash).unwrap();
    }

    out.flush().unwrap();
}

/// Read hashes saved with `write_hashes()` from the specified path or fail with line numbers not matching pattern.
pub fn read_hashes(err: &mut Write, file: &(String, PathBuf)) -> Result<BTreeMap<String, String>, i32> {
    lazy_static! {
        static ref LINE_RGX: Regex = Regex::new(r"^(.+?)\s{2,}([[:xdigit:]-]+)$").unwrap();
    }

    let mut hashes = BTreeMap::new();
    let mut failed = false;

    let in_file = BufReader::new(File::open(&file.1).unwrap());
    for (n, line) in in_file.lines().map(Result::unwrap).enumerate() {
        if !line.is_empty() {
            match LINE_RGX.captures(&line) {
                Some(captures) => {
                    hashes.insert(captures[1].to_string(), captures[2].to_string());
                }
                None => {
                    failed = true;
                    writeln!(err, "{}:{}: Line doesn't match accepted pattern", file.0, n).unwrap();
                }
            };
        }
    }

    if !failed {
        Ok(hashes)
    } else {
        Err(3)
    }
}
