use clap::{App, Arg, AppSettings};
use self::super::Algorithm;
use std::path::PathBuf;
use std::str::FromStr;
use std::fs;


/// Representation of the application's of all configurable values
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Options {
    /// Directory to hach/verify. Default: `"."`
    pub dir: PathBuf,
    /// Hashing algorithm to use. Default: `"SHA1"`
    pub algorithm: Algorithm,
    /// Whether to verify or create checksums. Default: yes
    pub verify: bool,
    /// Max recursion depth. `-1` for infinite. Default: 0
    pub depth: i32,
}

impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        let matches = App::new("checksums")
            .setting(AppSettings::AllowLeadingHyphen)
            .setting(AppSettings::ColoredHelp)
            .version(crate_version!())
            .author(crate_authors!())
            .about("Tool for making/verifying checksums of directory trees")
            .args(&[Arg::from_usage("[DIRECTORY] 'Directory to hash/verify'").default_value(".").validator(Options::directory_validator),
                    Arg::from_usage("--algorithm=[algorithm] -a 'Hashing algorithm to use. \
                                     Supported algorithms: SHA{1,2,3}, BLAKE{,2}, CRC{64,32,16,8}, MD5'")
                        .default_value("SHA1")
                        .validator(Options::algorithm_validator),
                    Arg::from_usage("--create -c 'Make checksums'").overrides_with("verify"),
                    Arg::from_usage("--verify -v 'Verify checksums (default)'").overrides_with("create"),
                    Arg::from_usage("--depth=[depth] -d 'Max recursion depth. `-1` for infinite.'")
                        .default_value("0")
                        .validator(Options::i32_validator)
                        .overrides_with("create")])
            .get_matches();

        Options {
            dir: fs::canonicalize(matches.value_of("DIRECTORY").unwrap()).unwrap(),
            algorithm: Algorithm::from_str(matches.value_of("algorithm").unwrap()).unwrap(),
            verify: !matches.is_present("create"),
            depth: i32::from_str(matches.value_of("depth").unwrap()).unwrap(),
        }
    }

    fn algorithm_validator(s: String) -> Result<(), String> {
        Algorithm::from_str(&s).map(|_| ())
    }

    fn directory_validator(s: String) -> Result<(), String> {
        fs::canonicalize(s).map_err(|e| e.to_string()).and_then(|p| {
            if p.is_file() {
                Err("DIRECTORY cannot be a file.".to_string())
            } else {
                Ok(())
            }
        })
    }

    fn i32_validator(s: String) -> Result<(), String> {
        i32::from_str(&s).map(|_| ()).map_err(|e| e.to_string())
    }
}
