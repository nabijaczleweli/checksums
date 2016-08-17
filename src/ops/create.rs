use self::super::super::{Algorithm, hash_file};
use self::super::super::options::DepthSetting;
use self::super::super::util::btreemap_append;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::fs;


/// Create subpath->hash mappings for a given path using a given algorithm up to a given depth.
pub fn create_hashes(path: &PathBuf, algo: Algorithm, remaining_depth: DepthSetting) -> BTreeMap<String, String> {
    create_hashes_p(path, String::new(), algo, remaining_depth)
}

fn create_hashes_p(path: &PathBuf, prefix: String, algo: Algorithm, remaining_depth: DepthSetting) -> BTreeMap<String, String> {
    let mut hashes = BTreeMap::new();

    for file in fs::read_dir(&path).unwrap() {
        let file = file.unwrap();
        let file_type = file.file_type().unwrap();
        // TODO: unnecessary? clone(), forced by compiler, "use of moved value" otherwise
        let file_name_s = prefix.clone() + file.file_name().to_str().unwrap();

        let mut subpath = path.clone();
        subpath.push(file.path());

        if file_type.is_file() {
            hashes.insert(file_name_s, hash_file(&subpath, algo));
        } else if remaining_depth.can_recurse() {
            // TODO: replace with stock `append()` call once `btree_append` is stabilised.
            //       Tracked by https://github.com/nabijaczleweli/checksums/issues/7
            btreemap_append(&mut hashes,
                            create_hashes_p(&subpath, file_name_s + "/", algo, remaining_depth.next_level().unwrap()));
        }
    }

    hashes
}
