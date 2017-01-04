use self::super::hash_string;
use md5::Context;


hash_func!(Context::new(),
           |ctx: &mut Context, buffer: &[u8]| ctx.consume(buffer),
           |ctx: Context| hash_string(&*ctx.compute()));
