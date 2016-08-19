macro_rules! make_md_mod {
    ($modname:ident, $bytesize:expr) => {
        pub mod $modname {
            use self::super::super::hash_string;
            use std::path::PathBuf;
            use std::fs::File;
            use std::io::Read;
            use md6::Md6;


            hash_func!(Md6::new($bytesize * 8).unwrap(),
                       |md6: &mut Md6, buffer: &[u8], read: usize| md6.update(&buffer[0..read]),
                       |md6: Md6| {
                           let mut result = [0; $bytesize];
                           md6.finalise(&mut result);
                           hash_string(&result)
                       });
        }
    }
}


make_md_mod!(md6128, 16);
make_md_mod!(md6256, 32);
make_md_mod!(md6512, 64);
