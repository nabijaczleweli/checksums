use crc32c::crc32c_append;


hash_func!(0u32,
           |state_crc: &mut u32, buffer: &[u8]| *state_crc = crc32c_append(*state_crc, buffer),
           |state_crc: u32| format!("{:08X}", state_crc));
