use checksums;

use self::checksums::Algorithm;
use std::str::FromStr;


#[test]
fn from_str() {
    for p in &[("sha1", Algorithm::SHA1),
               ("sha2", Algorithm::SHA2512),
               ("sha2-224", Algorithm::SHA2224),
               ("sha2-256", Algorithm::SHA2256),
               ("sha2-384", Algorithm::SHA2384),
               ("sha2-512", Algorithm::SHA2512),
               ("sha2256", Algorithm::SHA2256),
               ("sha2512", Algorithm::SHA2512),
               ("sha3", Algorithm::SHA3512),
               ("sha3-256", Algorithm::SHA3256),
               ("sha3-512", Algorithm::SHA3512),
               ("sha3256", Algorithm::SHA3256),
               ("sha3512", Algorithm::SHA3512),
               ("blake", Algorithm::BLAKE),
               ("blake2", Algorithm::BLAKE2),
               ("crc64", Algorithm::CRC64),
               ("crc32c", Algorithm::CRC32C),
               ("crc32", Algorithm::CRC32),
               ("crc16", Algorithm::CRC16),
               ("crc8", Algorithm::CRC8),
               ("md5", Algorithm::MD5),
               ("md6-128", Algorithm::MD6128),
               ("md6-256", Algorithm::MD6256),
               ("md6-512", Algorithm::MD6512)] {
        assert_eq!(Algorithm::from_str(p.0).unwrap(), p.1);
    }
}

#[test]
fn from_str_bad() {
    for s in &["asdf2", "sha123", "bla", "crc", "31234"] {
        Algorithm::from_str(s).unwrap_err();
    }
}
