use std::str::FromStr;


/// A hashing algorithm.
///
/// # Examples
///
/// ```
/// # use std::str::FromStr;
/// assert_eq!(checksums::Algorithm::from_str("SHA1"), Ok(checksums::Algorithm::SHA1));
/// assert_eq!(checksums::Algorithm::from_str("SHA-1"), Ok(checksums::Algorithm::SHA1));
///
/// assert_eq!(checksums::Algorithm::from_str("SHA2"), Ok(checksums::Algorithm::SHA2512));
/// assert_eq!(checksums::Algorithm::from_str("SHA-2"), Ok(checksums::Algorithm::SHA2512));
///
/// assert_eq!(checksums::Algorithm::from_str("BLAKE"), Ok(checksums::Algorithm::BLAKE));
/// assert_eq!(checksums::Algorithm::from_str("BLAKE2"), Ok(checksums::Algorithm::BLAKE2));
/// assert_eq!(checksums::Algorithm::from_str("BLAKE2"), Ok(checksums::Algorithm::BLAKE2B));
/// assert_eq!(checksums::Algorithm::from_str("BLAKE2B"), Ok(checksums::Algorithm::BLAKE2B));
///
/// assert_eq!(checksums::Algorithm::from_str("MD5"), Ok(checksums::Algorithm::MD5));
/// ```
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Algorithm {
    SHA1,
    /// SHA2-224
    SHA2224,
    /// SHA2-256
    SHA2256,
    /// SHA2-384
    SHA2384,
    /// SHA2-512
    SHA2512,
    /// SHA3-256
    SHA3256,
    /// SHA3-512
    SHA3512,
    BLAKE,
    BLAKE2B,
    BLAKE2S,
    BLAKE3,
    CRC64,
    CRC32,
    /// CRC-32-Castagnoli
    CRC32C,
    CRC16,
    CRC8,
    MD5,
    /// MD6-128
    MD6128,
    /// MD6-256
    MD6256,
    /// MD6-512
    MD6512,
    XOR8,
}

impl Algorithm {
    /// Compatibility alias.
    pub const BLAKE2: Algorithm = Algorithm::BLAKE2B;
}

impl Algorithm {
    /// Length, in bytes, of the algorithm's output hex string
    pub fn hexlen(&self) -> usize {
        match *self {
            Algorithm::XOR8 | Algorithm::CRC8 => 2,
            Algorithm::CRC16 => 4,
            Algorithm::CRC32C |
            Algorithm::CRC32 => 8,
            Algorithm::CRC64 => 16,
            Algorithm::SHA2224 => 28,
            Algorithm::MD5 |
            Algorithm::MD6128 => 32,
            Algorithm::SHA1 => 40,
            Algorithm::SHA2384 => 48,
            Algorithm::SHA2256 |
            Algorithm::SHA3256 |
            Algorithm::BLAKE2S |
            Algorithm::BLAKE3 |
            Algorithm::MD6256 => 64,
            Algorithm::SHA2512 |
            Algorithm::SHA3512 |
            Algorithm::BLAKE |
            Algorithm::BLAKE2B |
            Algorithm::MD6512 => 128,
        }
    }
}

impl FromStr for Algorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.replace("_", "-").to_lowercase()[..] {
            "sha-1" | "sha1" => Ok(Algorithm::SHA1),
            "sha2256" | "sha2-256" | "sha-2-256" => Ok(Algorithm::SHA2256),
            "sha2224" | "sha2-224" | "sha-2-224" => Ok(Algorithm::SHA2224),
            "sha2384" | "sha2-384" | "sha-2-384" => Ok(Algorithm::SHA2384),
            "sha2" | "sha-2" | "sha2512" | "sha2-512" | "sha-2-512" => Ok(Algorithm::SHA2512),
            "sha3256" | "sha3-256" | "sha-3-256" => Ok(Algorithm::SHA3256),
            "sha3" | "sha-3" | "sha3512" | "sha3-512" | "sha-3-512" => Ok(Algorithm::SHA3512),
            "blake" => Ok(Algorithm::BLAKE),
            "blake2" | "blake2b" => Ok(Algorithm::BLAKE2B),
            "blake2s" => Ok(Algorithm::BLAKE2S),
            "blake3" => Ok(Algorithm::BLAKE3),
            "crc64" => Ok(Algorithm::CRC64),
            "crc32c" |
            "crc32-c" |
            "crc32castagnoli" |
            "crc32-castagnoli" => Ok(Algorithm::CRC32C),
            "crc32" => Ok(Algorithm::CRC32),
            "crc16" => Ok(Algorithm::CRC16),
            "crc8" => Ok(Algorithm::CRC8),
            "md5" => Ok(Algorithm::MD5),
            "md6128" | "md6-128" => Ok(Algorithm::MD6128),
            "md6256" | "md6-256" => Ok(Algorithm::MD6256),
            "md6512" | "md6-512" => Ok(Algorithm::MD6512),
            "xor8" => Ok(Algorithm::XOR8),
            _ => Err(format!("\"{}\" is not a recognised hashing algorithm", s)),
        }
    }
}
