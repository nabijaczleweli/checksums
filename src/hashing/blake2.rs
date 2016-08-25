use blake2_rfc::blake2b::Blake2b;
use self::super::hash_string;


hash_func!(Blake2b::new(64),
           |blake: &mut Blake2b, buffer: &[u8]| blake.update(buffer),
           |blake: Blake2b| hash_string(blake.finalize().as_bytes()));
