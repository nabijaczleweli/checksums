use blake2_rfc::blake2b::Blake2b;
use self::super::hash_string;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;


pub fn hash(path: &PathBuf) -> String {
    let mut file = File::open(path).unwrap();
    let mut buffer = vec![0; 1024];

    let mut blake = Blake2b::new(64);
    loop {
        let read = file.read(&mut buffer[..]).unwrap();

        if read == 0 {
            break;
        }

        blake.update(&buffer[0..read]);
    }

    hash_string(blake.finalize().as_bytes())
}
