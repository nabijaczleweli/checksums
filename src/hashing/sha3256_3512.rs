macro_rules! make_sha_mod {
    ($modname:ident, $len:expr, $keccak_new:expr) => {
        pub mod $modname {
            use self::super::super::hash_string;
            use tiny_keccak::{Hasher, Sha3};


            hash_func!($keccak_new(),
                       |keccak: &mut Sha3, buffer: &[u8]| keccak.update(buffer),
                       |keccak: Sha3| {
                           let mut output = [0u8; $len];
                           keccak.finalize(&mut output);
                           hash_string(&output)
                       });
        }
    }
}


make_sha_mod!(sha3256, 32, Sha3::v256);
make_sha_mod!(sha3512, 64, Sha3::v512);
