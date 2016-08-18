include!("hash_func.rs");


macro_rules! make_sha_mod {
    ($modname:ident, $tpe:ty, $sha_new:expr) => {
        pub mod $modname {
            use shaman::digest::Digest;
            use shaman;
            use std::path::PathBuf;
            use std::fs::File;
            use std::io::Read;


            hash_func!($sha_new(),
                       |sha: &mut $tpe, buffer: &[u8], read: usize| sha.input(&buffer[0..read]),
                       |mut sha: $tpe| sha.result_str().to_uppercase());
        }
    }
}


make_sha_mod!(sha1, shaman::sha1::Sha1, shaman::sha1::Sha1::new);
make_sha_mod!(sha2256, shaman::sha2::Sha256, shaman::sha2::Sha256::new);
make_sha_mod!(sha2512, shaman::sha2::Sha512, shaman::sha2::Sha512::new);
