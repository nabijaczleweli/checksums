macro_rules! hash_func {
    ($ctx:expr, $update:expr, $convert:expr) => {
        pub fn hash(path: &PathBuf) -> String {
            let mut file = File::open(path).unwrap();
            let mut buffer = vec![0; 1024];

            let mut ctx = $ctx;
            loop {
                let read = file.read(&mut buffer[..]).unwrap();

                if read == 0 {
                    break;
                }

                $update(&mut ctx, &buffer, read);
            }

            $convert(ctx)
        }
    }
}
