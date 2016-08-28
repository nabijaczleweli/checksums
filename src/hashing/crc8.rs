use self::super::hash_string;
use crc8::Crc8;


hash_func!((Crc8::create_lsb(0x9b), 0u8),
           |state_crc: &mut (Crc8, u8), buffer: &[u8]| state_crc.1 = state_crc.0.calc(buffer, buffer.len() as i32, state_crc.1),
           |state_crc: (Crc8, u8)| hash_string(&[state_crc.1]));
