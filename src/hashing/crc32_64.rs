macro_rules! make_crc_mod {
    ($modname:ident, $algo:expr, $digest:ty, $digest_new:expr, $hasher_write:expr, $hasher_sum:expr, $poly:expr) => {
        pub mod $modname {
            use ::Algorithm;
            use crc;


            hash_func!($digest_new($poly),
                       |digest: &mut $digest, buffer: &[u8], read: usize| $hasher_write(digest, &buffer[0..read]),
                       |digest: $digest| format!("{:01$X}", $hasher_sum(&digest), $algo.hexlen()));
        }
    }
}


make_crc_mod!(crc32,
              Algorithm::CRC32,
              crc::crc32::Digest,
              crc::crc32::Digest::new,
              crc::crc32::Hasher32::write,
              crc::crc32::Hasher32::sum32,
              crc::crc32::IEEE);
make_crc_mod!(crc64,
              Algorithm::CRC64,
              crc::crc64::Digest,
              crc::crc64::Digest::new,
              crc::crc64::Hasher64::write,
              crc::crc64::Hasher64::sum64,
              crc::crc64::ISO);
