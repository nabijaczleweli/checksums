use std::collections::BTreeMap;
use std::iter;


/// Compare two provided hashes.
pub fn compare_hashes(out_file: &String, mut current_hashes: BTreeMap<String, String>, mut loaded_hashes: BTreeMap<String, String>) {
    // TODO: Proper result handling form this function
    let current_hashes_value_len = current_hashes.iter().next().unwrap().1.len();
    let loaded_hashes_value_len = loaded_hashes.iter().next().unwrap().1.len();
    if current_hashes_value_len != loaded_hashes_value_len {
        panic!("Hash lengths do not match; selected: {}, loaded: {}",
               current_hashes_value_len,
               loaded_hashes_value_len);
    }
    let placeholder_value = iter::repeat("-").take(current_hashes_value_len).collect::<String>();

    current_hashes.remove(out_file);
    loaded_hashes.remove(out_file);


    process_ignores(|key, _, other| !other.contains_key(key),
                    "added",
                    "removed",
                    &mut current_hashes,
                    &mut loaded_hashes);
    process_ignores(|_, value, _| *value == placeholder_value,
                    "ignored, skipping",
                    "ignored, skipping",
                    &mut current_hashes,
                    &mut loaded_hashes);

    // By this point both hashes have the same keysets
    assert_eq!(current_hashes.len(), loaded_hashes.len());

    if current_hashes.is_empty() {
        println!("No files left to compare");
    } else {
        for (key, loaded_value) in loaded_hashes {
            let ref current_value = current_hashes[&key];
            if *current_value == loaded_value {
                println!("File \"{}\" matches", key);
            } else {
                println!("File \"{}\" doesn't match.", key);
                println!("Was: {}", loaded_value);
                println!("Is: {}", current_value);
                println!("");
            }
        }
    }
}


fn process_ignores<F>(f: F, csubmsg: &str, lsubmsg: &str, ch: &mut BTreeMap<String, String>, lh: &mut BTreeMap<String, String>)
    where F: Fn(&String, &String, &BTreeMap<String, String>) -> bool
{
    let mut keys_to_remove = Vec::new();

    process_ignores_iter(&f, csubmsg, ch, lh, &mut keys_to_remove);
    process_ignores_iter(&f, lsubmsg, lh, ch, &mut keys_to_remove);

    if !keys_to_remove.is_empty() {
        println!("");
    }

    for key in keys_to_remove {
        ch.remove(&key);
        lh.remove(&key);
    }
}

fn process_ignores_iter<F>(f: &F, submsg: &str, curr: &BTreeMap<String, String>, other: &BTreeMap<String, String>, keys_to_remove: &mut Vec<String>)
    where F: Fn(&String, &String, &BTreeMap<String, String>) -> bool
{
    for (key, value) in curr {
        if f(key, value, other) {
            println!("File was {}: \"{}\"", submsg, key);
            keys_to_remove.push(key.clone());
        }
    }
}
