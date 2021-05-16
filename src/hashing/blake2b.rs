use self::super::hash_string;
use blake2::{Blake2b, Digest};

hash_func!(Blake2b::new(), |blake: &mut Blake2b, buffer: &[u8]| blake.update(buffer), |blake: Blake2b| {
	hash_string(&blake.finalize())
});
