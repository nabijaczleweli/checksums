macro_rules! make_md_mod {
    ($modname:ident, $bytesize:expr) => {
        pub mod $modname {
            use self::super::super::hash_string;
            use md6::Md6;


            hash_func_write!(Md6::new($bytesize * 8).unwrap(),
                             |mut md6: Md6| {
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
