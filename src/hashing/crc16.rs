use crc16::{State, ARC};
use crate::Algorithm;


hash_func!(State::new(),
           |state: &mut State<ARC>, buffer: &[u8]| state.update(buffer),
           |state: State<ARC>| format!("{:01$X}", state.get(), Algorithm::CRC16.hexlen()));
