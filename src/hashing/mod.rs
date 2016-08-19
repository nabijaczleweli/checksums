use std::iter::FromIterator;
use self::super::Algorithm;
use std::path::PathBuf;

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
/// Probably very inefficient. Better way = ?
///
/// # Examples
///
/// ```
/// assert_eq!(checksums::hash_string(&[0x99, 0xAA, 0xBB, 0xCC]), "99AABBCC".to_string());
/// assert_eq!(checksums::hash_string(&[0x09, 0x0A]), "090A".to_string());
/// ```
pub fn hash_string(bytes: &[u8]) -> String {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    static LOOKUP_TABLE: [&'static str; 256] = [
        "00", "01", "02", "03", "04", "05", "06", "07", "08", "09", "0A", "0B", "0C", "0D", "0E", "0F",
        "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "1A", "1B", "1C", "1D", "1E", "1F",
        "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "2A", "2B", "2C", "2D", "2E", "2F",
        "30", "31", "32", "33", "34", "35", "36", "37", "38", "39", "3A", "3B", "3C", "3D", "3E", "3F",
        "40", "41", "42", "43", "44", "45", "46", "47", "48", "49", "4A", "4B", "4C", "4D", "4E", "4F",
        "50", "51", "52", "53", "54", "55", "56", "57", "58", "59", "5A", "5B", "5C", "5D", "5E", "5F",
        "60", "61", "62", "63", "64", "65", "66", "67", "68", "69", "6A", "6B", "6C", "6D", "6E", "6F",
        "70", "71", "72", "73", "74", "75", "76", "77", "78", "79", "7A", "7B", "7C", "7D", "7E", "7F",
        "80", "81", "82", "83", "84", "85", "86", "87", "88", "89", "8A", "8B", "8C", "8D", "8E", "8F",
        "90", "91", "92", "93", "94", "95", "96", "97", "98", "99", "9A", "9B", "9C", "9D", "9E", "9F",
        "A0", "A1", "A2", "A3", "A4", "A5", "A6", "A7", "A8", "A9", "AA", "AB", "AC", "AD", "AE", "AF",
        "B0", "B1", "B2", "B3", "B4", "B5", "B6", "B7", "B8", "B9", "BA", "BB", "BC", "BD", "BE", "BF",
        "C0", "C1", "C2", "C3", "C4", "C5", "C6", "C7", "C8", "C9", "CA", "CB", "CC", "CD", "CE", "CF",
        "D0", "D1", "D2", "D3", "D4", "D5", "D6", "D7", "D8", "D9", "DA", "DB", "DC", "DD", "DE", "DF",
        "E0", "E1", "E2", "E3", "E4", "E5", "E6", "E7", "E8", "E9", "EA", "EB", "EC", "ED", "EE", "EF",
        "F0", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "FA", "FB", "FC", "FD", "FE", "FF",
    ];

    String::from_iter(bytes.iter().map(|&i| LOOKUP_TABLE[i as usize]))
}
