use crc16::{State, ARC};
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use ::Algorithm;


include!("hash_func.rs");


hash_func!(State::new(),
           |state: &mut State<ARC>, buffer: &[u8], read: usize| state.update(&buffer[0..read]),
           |state: State<ARC>| format!("{:01$X}", state.get(), Algorithm::CRC16.size()));
