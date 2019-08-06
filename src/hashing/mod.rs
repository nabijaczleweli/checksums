macro_rules! hash_func {
    ($ctx:expr, $update:expr, $convert:expr) => {
        use std::io::Read;


        pub fn hash<R: Read>(reader: &mut R) -> String {
            let mut buffer = vec![0; 4096];

            let mut ctx = $ctx;
            loop {
                let read = reader.read(&mut buffer[..]).unwrap();

                if read == 0 {
                    break;
                }

                $update(&mut ctx, &buffer[..read]);
            }

            $convert(ctx)
        }
    }
}

macro_rules! hash_func_write {
    ($ctx:expr, $convert:expr) => {
        use std::io::{self, Read};


        pub fn hash<R: Read>(reader: &mut R) -> String {
            let mut ctx = $ctx;
            io::copy(reader, &mut ctx).unwrap();
            $convert(ctx)
        }
    }
}


use super::Algorithm;
use std::path::Path;
use std::fmt::Write;
use std::fs::File;

mod md5;
mod xor8;
mod crc8;
mod crc16;
mod blake;
mod blake2;
mod crc32c;
mod crc32_64;
mod sha3256_3512;
mod md6128_256_512;
mod sha1_2256_2224_2384_2512;


/// Hash the specified file using the specified hashing algorithm.
pub fn hash_file(path: &Path, algo: Algorithm) -> String {
    let mut file = File::open(path).unwrap();
    match algo {
        Algorithm::SHA1 => sha1_2256_2224_2384_2512::sha1::hash(&mut file),
        Algorithm::SHA2224 => sha1_2256_2224_2384_2512::sha2224::hash(&mut file),
        Algorithm::SHA2256 => sha1_2256_2224_2384_2512::sha2256::hash(&mut file),
        Algorithm::SHA2384 => sha1_2256_2224_2384_2512::sha2384::hash(&mut file),
        Algorithm::SHA2512 => sha1_2256_2224_2384_2512::sha2512::hash(&mut file),
        Algorithm::SHA3256 => sha3256_3512::sha3256::hash(&mut file),
        Algorithm::SHA3512 => sha3256_3512::sha3512::hash(&mut file),
        Algorithm::BLAKE => blake::hash(&mut file),
        Algorithm::BLAKE2 => blake2::hash(&mut file),
        Algorithm::CRC64 => crc32_64::crc64::hash(&mut file),
        Algorithm::CRC32 => crc32_64::crc32::hash(&mut file),
        Algorithm::CRC32C => crc32c::hash(&mut file),
        Algorithm::CRC16 => crc16::hash(&mut file),
        Algorithm::CRC8 => crc8::hash(&mut file),
        Algorithm::MD5 => md5::hash(&mut file),
        Algorithm::MD6128 => md6128_256_512::md6128::hash(&mut file),
        Algorithm::MD6256 => md6128_256_512::md6256::hash(&mut file),
        Algorithm::MD6512 => md6128_256_512::md6512::hash(&mut file),
        Algorithm::XOR8 => xor8::hash(&mut file),
    }
}

/// Create a hash string out of its raw bytes.
///
/// # Examples
///
/// ```
/// assert_eq!(checksums::hash_string(&[0x99, 0xAA, 0xBB, 0xCC]), "99AABBCC".to_string());
/// assert_eq!(checksums::hash_string(&[0x09, 0x0A]), "090A".to_string());
/// ```
pub fn hash_string(bytes: &[u8]) -> String {
    let mut result = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        write!(result, "{:02X}", b).unwrap();
    }
    result
}
