use std::str::FromStr;


/// A hashing algorithm.
///
/// # Examples
///
/// ```
/// assert_eq!(Algorithm::from_str("SHA1"), Ok(Algorithm::SHA1));
/// assert_eq!(Algorithm::from_str("SHA-1"), Ok(Algorithm::SHA1));
///
/// assert_eq!(Algorithm::from_str("SHA2"), Ok(Algorithm::SHA2512));
/// assert_eq!(Algorithm::from_str("SHA-2"), Ok(Algorithm::SHA2512));
///
/// assert_eq!(Algorithm::from_str("BLAKE"), Ok(Algorithm::BLAKE));
/// assert_eq!(Algorithm::from_str("BLAKE2"), Ok(Algorithm::BLAKE2));
///
/// assert_eq!(Algorithm::from_str("MD5"), Ok(Algorithm::MD5));
/// ```
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Algorithm {
    SHA1,
    /// SHA2-256
    SHA2256,
    /// SHA2-512
    SHA2512,
    /// SHA3-256
    SHA3256,
    /// SHA3-512
    SHA3512,
    /// SHA3-1024
    SHA31024,
    /// SHA3-2048
    SHA32048,
    BLAKE,
    BLAKE2,
    CRC64,
    CRC32,
    CRC16,
    CRC8,
    MD5,
    XOR8,
}

impl Algorithm {
    /// Length, in bytes, of the algorithm's output
    pub fn size(&self) -> usize {
        match self {
            &Algorithm::XOR8 | &Algorithm::CRC8 => 2,
            &Algorithm::CRC16 => 4,
            &Algorithm::CRC32 => 8,
            &Algorithm::CRC64 => 16,
            &Algorithm::MD5 => 32,
            &Algorithm::SHA1 => 40,
            &Algorithm::SHA2256 |
            &Algorithm::SHA3256 => 64,
            &Algorithm::SHA2512 |
            &Algorithm::SHA3512 |
            &Algorithm::BLAKE |
            &Algorithm::BLAKE2 => 128,
            &Algorithm::SHA31024 => 256,
            &Algorithm::SHA32048 => 512,
        }
    }
}

impl FromStr for Algorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SHA1" | "SHA-1" | "sha1" => Ok(Algorithm::SHA1),
            "SHA2256" | "SHA2-256" | "SHA-2-256" | "SHA2_256" | "SHA-2_256" | "SHA_2-256" | "SHA_2_256" | "sha2256" | "sha2-256" | "sha2_256" => {
                Ok(Algorithm::SHA2256)
            }
            "SHA2" | "SHA-2" | "sha2" | "SHA2512" | "SHA2-512" | "SHA-2-512" | "SHA2_512" | "SHA-2_512" | "SHA_2-512" | "SHA_2_512" | "sha2512" |
            "sha2-512" | "sha2_512" => Ok(Algorithm::SHA2512),
            "SHA3256" | "SHA3-256" | "SHA-3-256" | "SHA3_256" | "SHA-3_256" | "SHA_3-256" | "SHA_3_256" | "sha3256" | "sha3-256" | "sha3_256" => {
                Ok(Algorithm::SHA3256)
            }
            "SHA3" | "SHA-3" | "sha3" | "SHA3512" | "SHA3-512" | "SHA-3-512" | "SHA3_512" | "SHA-3_512" | "SHA_3-512" | "SHA_3_512" | "sha3512" |
            "sha3-512" | "sha3_512" => Ok(Algorithm::SHA3512),
            "SHA31024" | "SHA3-1024" | "SHA-3-1024" | "SHA3_1024" | "SHA-3_1024" | "SHA_3-1024" | "SHA_3_1024" | "sha31024" | "sha3-1024" | "sha3_1024" => {
                Ok(Algorithm::SHA31024)
            }
            "SHA32048" | "SHA3-2048" | "SHA-3-2048" | "SHA3_2048" | "SHA-3_2048" | "SHA_3-2048" | "SHA_3_2048" | "sha32048" | "sha3-2048" | "sha3_2048" => {
                Ok(Algorithm::SHA32048)
            }
            "BLAKE" | "blake" => Ok(Algorithm::BLAKE),
            "BLAKE2" | "blake2" => Ok(Algorithm::BLAKE2),
            "CRC64" | "crc64" => Ok(Algorithm::CRC64),
            "CRC32" | "crc32" => Ok(Algorithm::CRC32),
            "CRC16" | "crc16" => Ok(Algorithm::CRC16),
            "CRC8" | "crc8" => Ok(Algorithm::CRC8),
            "MD5" | "md5" => Ok(Algorithm::MD5),
            "XOR8" | "xor8" => Ok(Algorithm::XOR8),
            _ => Err(format!("\"{}\" is not a recognised compression algorithm", s)),
        }
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use self::super::Algorithm;


    #[test]
    fn doctest() {
        assert_eq!(Algorithm::from_str("SHA1"), Ok(Algorithm::SHA1));
        assert_eq!(Algorithm::from_str("SHA-1"), Ok(Algorithm::SHA1));

        assert_eq!(Algorithm::from_str("SHA2"), Ok(Algorithm::SHA2512));
        assert_eq!(Algorithm::from_str("SHA-2"), Ok(Algorithm::SHA2512));

        assert_eq!(Algorithm::from_str("BLAKE"), Ok(Algorithm::BLAKE));
        assert_eq!(Algorithm::from_str("BLAKE2"), Ok(Algorithm::BLAKE2));

        assert_eq!(Algorithm::from_str("MD5"), Ok(Algorithm::MD5));
    }

    #[test]
    fn from_str() {
        for p in &[("sha1", Algorithm::SHA1),
                   ("sha2", Algorithm::SHA2512),
                   ("sha2-256", Algorithm::SHA2256),
                   ("sha2-512", Algorithm::SHA2512),
                   ("sha2256", Algorithm::SHA2256),
                   ("sha2512", Algorithm::SHA2512),
                   ("sha3", Algorithm::SHA3512),
                   ("sha3-256", Algorithm::SHA3256),
                   ("sha3-512", Algorithm::SHA3512),
                   ("sha3-1024", Algorithm::SHA31024),
                   ("sha3-2048", Algorithm::SHA32048),
                   ("sha3256", Algorithm::SHA3256),
                   ("sha3512", Algorithm::SHA3512),
                   ("sha31024", Algorithm::SHA31024),
                   ("sha32048", Algorithm::SHA32048),
                   ("blake", Algorithm::BLAKE),
                   ("blake2", Algorithm::BLAKE2),
                   ("crc64", Algorithm::CRC64),
                   ("crc32", Algorithm::CRC32),
                   ("crc16", Algorithm::CRC16),
                   ("crc8", Algorithm::CRC8),
                   ("md5", Algorithm::MD5)] {
            assert_eq!(Algorithm::from_str(p.0).unwrap(), p.1);
        }
    }

    #[test]
    fn from_str_bad() {
        for s in &["asdf2", "sha123", "bla", "crc", "31234"] {
            Algorithm::from_str(s).unwrap_err();
        }
    }
}
