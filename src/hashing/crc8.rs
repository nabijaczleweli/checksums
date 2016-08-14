use self::super::hash_string;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use crc8::Crc8;


pub fn hash(path: &PathBuf) -> String {
    let mut file = File::open(path).unwrap();
    let mut buffer = vec![0; 1024];

    let mut state = Crc8::create_lsb(0x9b);
    let mut crc = 0u8;
    loop {
        let read = file.read(&mut buffer[..]).unwrap();

        if read == 0 {
            break;
        }

        crc = state.calc(&buffer[..], read as i32, crc);
    }

    hash_string(&[crc])
}
