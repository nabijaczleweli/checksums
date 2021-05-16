use self::super::hash_string;

hash_func!(
	blake3::Hasher::new(),
	|blake: &mut blake3::Hasher, buffer: &[u8]| {
		blake.update(buffer);
	},
	|blake: blake3::Hasher| hash_string(blake.finalize().as_bytes())
);
