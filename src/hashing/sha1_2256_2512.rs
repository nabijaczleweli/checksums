macro_rules! make_sha_mod {
    ($modname:ident, $sha_new:expr) => {
        pub mod $modname {
            use shaman::digest::Digest;
            use shaman;
            use std::path::PathBuf;
            use std::fs::File;
            use std::io::Read;


            pub fn hash(path: &PathBuf) -> String {
                let mut file = File::open(path).unwrap();
                let mut buffer = vec![0; 1024];

                let mut sha = $sha_new();
                loop {
                    let read = file.read(&mut buffer[..]).unwrap();

                    if read == 0 {
                        break;
                    }

                    sha.input(&buffer[0..read]);
                }

                sha.result_str().to_uppercase()
            }
        }
    }
}


make_sha_mod!(sha1, shaman::sha1::Sha1::new);
make_sha_mod!(sha2256, shaman::sha2::Sha256::new);
make_sha_mod!(sha2512, shaman::sha2::Sha512::new);
