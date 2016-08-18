use self::super::super::util::{btreemap_append, mul_str};
use self::super::super::{Algorithm, hash_file};
use self::super::super::options::DepthSetting;
use std::collections::{BTreeSet, BTreeMap};
use std::path::PathBuf;
use std::fs;


/// Create subpath->hash mappings for a given path using a given algorithm up to a given depth.
pub fn create_hashes(path: &PathBuf, ignored_files: BTreeSet<String>, algo: Algorithm, remaining_depth: DepthSetting, follow_symlinks: bool)
                     -> BTreeMap<String, String> {
    create_hashes_p(path, &ignored_files, String::new(), algo, remaining_depth, follow_symlinks)
}

fn create_hashes_p(path: &PathBuf, ignored_files: &BTreeSet<String>, prefix: String, algo: Algorithm, remaining_depth: DepthSetting, follow_symlinks: bool)
                   -> BTreeMap<String, String> {
    let mut hashes = BTreeMap::new();

    for file in fs::read_dir(&path).unwrap().map(Result::unwrap) {
        let file_type = file.file_type().unwrap();
        let file_name_s = prefix.clone() + file.file_name().to_str().unwrap();
        let ignored = ignored_files.contains(&file_name_s);

        let mut subpath = path.clone();
        subpath.push(file.path());

        if file_type.is_file() {
            let hash = if ignored {
                mul_str("-", algo.size())
            } else {
                hash_file(&subpath, algo)
            };
            hashes.insert(file_name_s, hash);
        } else if !ignored && remaining_depth.can_recurse() && (follow_symlinks || !file_type.is_symlink()) {
            // TODO: replace with stock `append()` call once `btree_append` is stabilised.
            //       Tracked by https://github.com/nabijaczleweli/checksums/issues/7
            btreemap_append(&mut hashes,
                            create_hashes_p(&subpath,
                                            ignored_files,
                                            file_name_s + "/",
                                            algo,
                                            remaining_depth.next_level().unwrap(),
                                            follow_symlinks));
        }
    }

    hashes
}
