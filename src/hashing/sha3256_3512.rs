macro_rules! make_sha_mod {
    ($modname:ident, $len:expr, $keccak_new:expr) => {
        pub mod $modname {
            use self::super::super::hash_string;
            use tiny_keccak::Keccak;


            hash_func!($keccak_new(),
                       |keccak: &mut Keccak, buffer: &[u8]| keccak.update(buffer),
                       |keccak: Keccak| {
                           let mut output = [0u8; $len];
                           keccak.finalize(&mut output);
                           hash_string(&output)
                       });
        }
    }
}


make_sha_mod!(sha3256, 32, Keccak::new_sha3_256);
make_sha_mod!(sha3512, 64, Keccak::new_sha3_512);
