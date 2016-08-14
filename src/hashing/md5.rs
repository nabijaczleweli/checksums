use self::super::hash_string;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use md5::Context;


pub fn hash(path: &PathBuf) -> String {
    let mut file = File::open(path).unwrap();
    let mut buffer = vec![0; 1024];

    let mut ctx = Context::new();
    loop {
        let read = file.read(&mut buffer[..]).unwrap();

        if read == 0 {
            break;
        }

        ctx.consume(&buffer[0..read]);
    }

    hash_string(&ctx.compute())
}
