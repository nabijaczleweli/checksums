macro_rules! make_sha_mod {
    ($modname:ident, $len:expr, $keccak_new:expr) => {
        pub mod $modname {
            use self::super::super::hash_string;
            use std::path::PathBuf;
            use std::fs::File;
            use std::io::Read;
            use tiny_keccak;


            pub fn hash(path: &PathBuf) -> String {
                let mut file = File::open(path).unwrap();
                let mut buffer = vec![0; 1024];

                let mut keccak = $keccak_new();
                loop {
                    let read = file.read(&mut buffer[..]).unwrap();

                    if read == 0 {
                        break;
                    }

                    keccak.update(&buffer[0..read]);
                }

                let mut output = [0u8; $len];
                keccak.finalize(&mut output);
                hash_string(&output)
            }
        }
    }
}


make_sha_mod!(sha3256, 32, tiny_keccak::Keccak::new_sha3_256);
make_sha_mod!(sha3512, 64, tiny_keccak::Keccak::new_sha3_512);
