macro_rules! hash_func {
    ($ctx:expr, $update:expr, $convert:expr) => {
        pub fn hash<R: ::std::io::Read>(reader: &mut R) -> String {
            let mut buffer = vec![0; 4096];

            let mut ctx = $ctx;
            loop {
                let read = reader.read(&mut buffer[..]).unwrap();

                if read == 0 {
                    break;
                }

                $update(&mut ctx, &buffer, read);
            }

            $convert(ctx)
        }
    }
}


use std::path::Path;
use super::Algorithm;
use std::fmt::Write;
use std::fs::File;

mod md5;
mod xor8;
mod crc8;
mod crc16;
mod blake;
mod blake2;
mod crc32_64;
mod sha3256_3512;
mod sha1_2256_2512;
mod md6128_256_512;

/// Hash the specified file using the specified hashing algorithm.
pub fn hash_file(path: &Path, algo: Algorithm) -> String {
    let mut file = &mut File::open(path).unwrap();
    match algo {
        Algorithm::SHA1 => sha1_2256_2512::sha1::hash(file),
        Algorithm::SHA2256 => sha1_2256_2512::sha2256::hash(file),
        Algorithm::SHA2512 => sha1_2256_2512::sha2512::hash(file),
        Algorithm::SHA3256 => sha3256_3512::sha3256::hash(file),
        Algorithm::SHA3512 => sha3256_3512::sha3512::hash(file),
        Algorithm::BLAKE => blake::hash(file),
        Algorithm::BLAKE2 => blake2::hash(file),
        Algorithm::CRC64 => crc32_64::crc64::hash(file),
        Algorithm::CRC32 => crc32_64::crc32::hash(file),
        Algorithm::CRC16 => crc16::hash(file),
        Algorithm::CRC8 => crc8::hash(file),
        Algorithm::MD5 => md5::hash(file),
        Algorithm::MD6128 => md6128_256_512::md6128::hash(file),
        Algorithm::MD6256 => md6128_256_512::md6256::hash(file),
        Algorithm::MD6512 => md6128_256_512::md6512::hash(file),
        Algorithm::XOR8 => xor8::hash(file),
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
