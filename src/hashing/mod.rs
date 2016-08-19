use std::path::PathBuf;
use super::Algorithm;
use std::fmt::Write;

macro_rules! hash_func {
    ($ctx:expr, $update:expr, $convert:expr) => {
        pub fn hash(path: &PathBuf) -> String {
            let mut file = File::open(path).unwrap();
            let mut buffer = vec![0; 4096];

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
pub fn hash_file(path: &PathBuf, algo: Algorithm) -> String {
    match algo {
        Algorithm::SHA1 => sha1_2256_2512::sha1::hash(path),
        Algorithm::SHA2256 => sha1_2256_2512::sha2256::hash(path),
        Algorithm::SHA2512 => sha1_2256_2512::sha2512::hash(path),
        Algorithm::SHA3256 => sha3256_3512::sha3256::hash(path),
        Algorithm::SHA3512 => sha3256_3512::sha3512::hash(path),
        Algorithm::BLAKE => blake::hash(path),
        Algorithm::BLAKE2 => blake2::hash(path),
        Algorithm::CRC64 => crc32_64::crc64::hash(path),
        Algorithm::CRC32 => crc32_64::crc32::hash(path),
        Algorithm::CRC16 => crc16::hash(path),
        Algorithm::CRC8 => crc8::hash(path),
        Algorithm::MD5 => md5::hash(path),
        Algorithm::MD6128 => md6128_256_512::md6128::hash(path),
        Algorithm::MD6256 => md6128_256_512::md6256::hash(path),
        Algorithm::MD6512 => md6128_256_512::md6512::hash(path),
        Algorithm::XOR8 => xor8::hash(path),
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
