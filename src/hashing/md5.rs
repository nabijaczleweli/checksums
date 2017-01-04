use self::super::hash_string;
use md5::Context;


hash_func_write!(Context::new(),
                 |ctx: Context| hash_string(&*ctx.compute()));
