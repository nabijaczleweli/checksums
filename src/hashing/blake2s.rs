use self::super::hash_string;
use blake2::{Blake2s, Digest};

hash_func!(Blake2s::new(),
           |blake: &mut Blake2s, buffer: &[u8]| blake.update(buffer),
           |blake: Blake2s| hash_string(&blake.finalize()));
