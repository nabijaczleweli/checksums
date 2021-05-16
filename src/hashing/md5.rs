use self::super::hash_string;
use md5::{Md5, Digest};

hash_func_write!(Md5::new(),
                 |ctx: Md5| hash_string(&*ctx.finalize()));
