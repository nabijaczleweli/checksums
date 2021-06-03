use self::super::hash_string;
use whirlpool::{Whirlpool, Digest};

hash_func!(Whirlpool::new(),
           |whirlpool: &mut Whirlpool, buffer: &[u8]| whirlpool.update(buffer),
           |whirlpool: Whirlpool| hash_string(&whirlpool.finalize()));
