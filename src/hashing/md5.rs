use self::super::hash_string;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use md5::Context;


include!("hash_func.rs");


hash_func!(Context::new(),
           |ctx: &mut Context, buffer: &[u8], read: usize| ctx.consume(&buffer[0..read]),
           |ctx: Context| hash_string(&ctx.compute()));
