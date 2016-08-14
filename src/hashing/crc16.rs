use crc16::{State, ARC};
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use ::Algorithm;


pub fn hash(path: &PathBuf) -> String {
    let mut file = File::open(path).unwrap();
    let mut buffer = vec![0; 1024];

    let mut state: State<ARC> = State::new();
    loop {
        let read = file.read(&mut buffer[..]).unwrap();

        if read == 0 {
            break;
        }

        state.update(&buffer[0..read]);
    }

    format!("{:01$x}", state.get(), Algorithm::CRC16.size())
}
