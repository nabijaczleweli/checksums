use self::super::super::{Algorithm, hash_file};
use std::collections::{BTreeSet, BTreeMap};
use walkdir::{WalkDir, WalkDirIterator};
use self::super::super::util::mul_str;
use futures::{Future, Task, Poll};
use futures_cpupool::CpuPool;
use std::time::Duration;
use std::path::PathBuf;
use pbr::ProgressBar;
use std::io::Write;
use std::thread;


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
