use std::str::FromStr;


/// A hashing algorithm.
///
/// # Examples
///
/// ```
/// assert_eq!(Algorithm::from_str("SHA1"), Algorithm::SHA1);
/// assert_eq!(Algorithm::from_str("SHA-1"), Algorithm::SHA1);
///
/// assert_eq!(Algorithm::from_str("SHA2"), Algorithm::SHA2);
/// assert_eq!(Algorithm::from_str("SHA-2"), Algorithm::SHA2);
///
/// assert_eq!(Algorithm::from_str("BLAKE"), Algorithm::BLAKE);
/// assert_eq!(Algorithm::from_str("BLAKE2"), Algorithm::BLAKE2);
///
/// assert_eq!(Algorithm::from_str("MD5"), Algorithm::MD5);
/// ```
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Algorithm {
    SHA1,
    SHA2,
    SHA3,
    BLAKE,
    BLAKE2,
    CRC64,
    CRC32,
    CRC16,
    CRC8,
    MD5,
}

impl FromStr for Algorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SHA1" | "SHA-1" | "sha1" => Ok(Algorithm::SHA1),
            "SHA2" | "SHA-2" | "sha2" => Ok(Algorithm::SHA2),
            "SHA3" | "SHA-3" | "sha3" => Ok(Algorithm::SHA3),
            "BLAKE" | "blake" => Ok(Algorithm::BLAKE),
            "BLAKE2" | "blake2" => Ok(Algorithm::BLAKE2),
            "CRC64" | "crc64" => Ok(Algorithm::CRC64),
            "CRC32" | "crc32" => Ok(Algorithm::CRC32),
            "CRC16" | "crc16" => Ok(Algorithm::CRC16),
            "CRC8" | "crc8" => Ok(Algorithm::CRC8),
            "MD5" | "md5" => Ok(Algorithm::MD5),
            _ => Err(format!("{:?} is not a recognised compression algorithm", s)),
        }
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use self::super::Algorithm;


    #[test]
    fn from_str() {
        for p in &[("sha1", Algorithm::SHA1),
                   ("sha2", Algorithm::SHA2),
                   ("sha3", Algorithm::SHA3),
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
