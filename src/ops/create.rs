use self::super::super::{Algorithm, hash_file};
use self::super::super::options::DepthSetting;
use std::collections::{BTreeSet, BTreeMap};
use futures_cpupool::{CpuPool, CpuFuture};
use self::super::super::util::mul_str;
use futures::{Future, Task, Poll};
use std::time::Duration;
use std::path::PathBuf;
use pbr::ProgressBar;
use std::io::Write;
use std::thread;
use std::fs;


lazy_static! {
    // TODO: customisation point?
    static ref SLEEP_LEN: Duration = Duration::from_millis(1);
}


/// Create subpath->hash mappings for a given path using a given algorithm up to a given depth.
pub fn create_hashes<W>(path: &PathBuf, ignored_files: BTreeSet<String>, algo: Algorithm, remaining_depth: DepthSetting, follow_symlinks: bool, jobs: u32,
                        pb_out: W)
                        -> BTreeMap<String, String>
    where W: Write
{
    let pool = CpuPool::new(jobs);
    let mut hashes_f = BTreeMap::new();
    create_hashes_p(&mut hashes_f,
                    &path,
                    &ignored_files,
                    String::new(),
                    &pool,
                    algo,
                    remaining_depth,
                    follow_symlinks);

    let mut pb = ProgressBar::on(pb_out, hashes_f.len() as u64);
    pb.set_width(Some(80));
    pb.show_speed = false;
    pb.show_tick = true;

    let hashes = hashes_f.into_iter()
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
        })
        .collect();

    pb.show_tick = false;
    pb.tick();
    pb.finish_print("");
    hashes
}

fn create_hashes_p(hashes: &mut BTreeMap<String, CpuFuture<String>>, path: &PathBuf, ignored_files: &BTreeSet<String>, prefix: String, pool: &CpuPool,
                   algo: Algorithm, remaining_depth: DepthSetting, follow_symlinks: bool) {
    for file in fs::read_dir(&path).unwrap().map(Result::unwrap) {
        let file_type = file.file_type().unwrap();
        let file_name_s = prefix.clone() + file.file_name().to_str().unwrap();
        let ignored = ignored_files.contains(&file_name_s);

        let mut subpath = path.clone();
        subpath.push(file.path());

        if file_type.is_file() {
            let hash = if ignored {
                // TODO: ideally, this'd be a futures::done() but I was unable to generalise it to do both CpuFuture and Future
                pool.execute(move || mul_str("-", algo.hexlen()))
            } else {
                pool.execute(move || hash_file(&subpath, algo))
            };
            hashes.insert(file_name_s, hash);
        } else if !ignored && remaining_depth.can_recurse() && (follow_symlinks || !file_type.is_symlink()) {
            create_hashes_p(hashes,
                            &subpath,
                            ignored_files,
                            file_name_s + "/",
                            pool,
                            algo,
                            remaining_depth.next_level().unwrap(),
                            follow_symlinks)
        }
    }
}
