use self::super::hash_string;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use crc8::Crc8;


include!("hash_func.rs");


hash_func!((Crc8::create_lsb(0x9b), 0u8),
           |state_crc: &mut (Crc8, u8), buffer: &[u8], read: usize| state_crc.1 = state_crc.0.calc(buffer, read as i32, state_crc.1),
           |state_crc: (Crc8, u8)| hash_string(&[state_crc.1]));
