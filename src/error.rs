/// Enum representing each way the appication can fail.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Error {
    /// No errors occured, everything executed correctly.
    NoError,
    /// Parsing of command-line options failed.
    OptionParsingError,
    /// Selected and saved hash lengths differ.
    HashLengthDiffers,
    /// Parsing the hashes file failed.
    HashesFileParsingFailure,
    /// The specified amount of files do not match.
    NFilesDiffer(i32),
}

impl Error {
    /// Get the executable exit value from an `Error` instance.
    pub fn exit_value(&self) -> i32 {
        match *self {
            Error::NoError => 0,
            Error::OptionParsingError => 1,
            Error::HashLengthDiffers => 2,
            Error::HashesFileParsingFailure => 3,
            Error::NFilesDiffer(i) => i + 3,
        }
    }
}

impl From<i32> for Error {
    fn from(i: i32) -> Self {
        match i {
            0 => Error::NoError,
            1 => Error::OptionParsingError,
            2 => Error::HashLengthDiffers,
            3 => Error::HashesFileParsingFailure,
            i => Error::NFilesDiffer(i - 3),
        }
    }
}
