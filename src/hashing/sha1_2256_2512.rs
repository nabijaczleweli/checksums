macro_rules! make_sha_mod {
    ($modname:ident, $tpe:ty, $sha_new:expr) => {
        pub mod $modname {
            use shaman::digest::Digest;
            use shaman;


            hash_func!($sha_new(),
                       |sha: &mut $tpe, buffer: &[u8]| sha.input(buffer),
                       |mut sha: $tpe| sha.result_str().to_uppercase());
        }
    }
}


make_sha_mod!(sha1, shaman::sha1::Sha1, shaman::sha1::Sha1::new);
make_sha_mod!(sha2256, shaman::sha2::Sha256, shaman::sha2::Sha256::new);
make_sha_mod!(sha2512, shaman::sha2::Sha512, shaman::sha2::Sha512::new);
