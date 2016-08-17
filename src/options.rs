//! Option parsing and management.
//!
//! Use the `Options::parse()` function to get the program's configuration,
//! as parsed from the commandline.
//!
//! # Examples
//!
//! ```skip
//! let opts = Options::parse();
//! println!("{:?}", opts);
//! ```


use clap::{self, App, Arg, AppSettings};
use self::super::Algorithm;
use std::path::PathBuf;
use std::str::FromStr;
use std::fs;


/// Representation of the application's all configurable values.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Options {
    /// Directory to hach/verify. Default: `"."`
    pub dir: PathBuf,
    /// Hashing algorithm to use. Default: `"SHA1"`
    pub algorithm: Algorithm,
    /// Whether to verify or create checksums. Default: yes
    pub verify: bool,
    /// Max recursion depth. Default: `LastLevel`
    pub depth: DepthSetting,
    /// In-/Output filename. Default: `"./INFERRED_FROM_DIRECTORY.hash"`
    pub file: (String, PathBuf),
}

/// Representation of how deep recursion should be.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum DepthSetting {
    /// Infinite allowed recursion, `-1` in argument
    Infinite,
    /// Last recursion level, go no further. `0` in argument. The default
    LastLevel,
    /// Another `N` recursion levels remaining.
    ///
    /// Entering a value `<= 0` here yields unspecified behaviour.
    NRemaining(u32),
}


impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        let matches = App::new("checksums")
            .setting(AppSettings::ColoredHelp)
            .version(crate_version!())
            .author(crate_authors!())
            .about("Tool for making/verifying checksums of directory trees")
            .args(&[Arg::from_usage("[DIRECTORY] 'Directory to hash/verify'").default_value(".").validator(Options::directory_validator),
                    Arg::from_usage("--algorithm=[algorithm] -a 'Hashing algorithm to use. {n}\
                                     Supported algorithms: SHA{1,2-{256,512},3-{256,512}}, BLAKE{,2}, CRC{64,32,16,8}, MD5, XOR8'")
                        .next_line_help(true)
                        .default_value("SHA1")
                        .validator(Options::algorithm_validator),
                    Arg::from_usage("--create -c 'Make checksums'").overrides_with("verify"),
                    Arg::from_usage("--verify -v 'Verify checksums (default)'").overrides_with("create"),
                    Arg::from_usage("--depth=[depth] -d 'Max recursion depth. `-1` for infinite.'. Default: don't recurse")
                        .validator(Options::depth_validator)
                        .overrides_with("recursive"),
                    Arg::from_usage("--recursive -r 'Infinite recursion depth.'").overrides_with("depth"),
                    Arg::from_usage("--file=[file] -f 'File with hashes to be read/created'").validator(Options::file_validator),
                    Arg::from_usage("--force 'Override output file'")])
            .get_matches();

        let dir = fs::canonicalize(matches.value_of("DIRECTORY").unwrap()).unwrap();
        let verify = !matches.is_present("create");
        let file = Options::file_process(matches.value_of("file"), &dir);

        if file.1.exists() && !verify && !matches.is_present("force") {
            clap::Error {
                    message: "The output file exists and was not overridden to prevent data loss.\n\
                              Pass the --force option to suppress this error."
                        .to_string(),
                    kind: clap::ErrorKind::MissingRequiredArgument,
                    info: None,
                }
                .exit();
        }

        Options {
            dir: dir,
            algorithm: Algorithm::from_str(matches.value_of("algorithm").unwrap()).unwrap(),
            verify: verify,
            depth: if matches.is_present("recursive") {
                DepthSetting::Infinite
            } else {
                DepthSetting::from_str(matches.value_of("depth").unwrap_or("0")).unwrap()
            },
            file: file,
        }
    }

    fn algorithm_validator(s: String) -> Result<(), String> {
        Algorithm::from_str(&s).map(|_| ())
    }

    fn directory_validator(s: String) -> Result<(), String> {
        fs::canonicalize(s).map_err(|e| format!("directory: {}", e.to_string())).and_then(|p| {
            if p.is_file() {
                Err("DIRECTORY cannot be a file.".to_string())
            } else {
                Ok(())
            }
        })
    }

    fn depth_validator(s: String) -> Result<(), String> {
        DepthSetting::from_str(&s).map(|_| ())
    }

    fn file_validator(s: String) -> Result<(), String> {
        let mut buf = PathBuf::from(s);
        if buf.exists() && buf.is_dir() {
            Err("file exists and is a directory".to_string())
        } else {
            buf.pop();

            // Handle pathless filename
            if buf.as_os_str().is_empty() {
                Ok(())
            } else {
                buf.canonicalize().map(|_| ()).map_err(|e| format!("file: {}", e.to_string()))
            }
        }
    }

    fn file_process(file: Option<&str>, dir: &PathBuf) -> (String, PathBuf) {
        match file {
            Some(file) => {
                let mut file = PathBuf::from(file);
                let file_name = file.file_name().unwrap().to_os_string();

                file.pop();
                // Handle pathless filename
                if file.as_os_str().is_empty() {
                    file.push(".");
                }

                (file_name.to_str().unwrap().to_string(),
                 file.canonicalize()
                    .map(|mut p| {
                        p.push(file_name);
                        p
                    })
                    .unwrap())
            }
            None => {
                let mut file = dir.clone();
                match dir.file_name() {
                    Some(fname) => file.push(fname),
                    None => file.push(Options::root_fname(dir)),
                }
                file.set_extension("hash");

                (file.file_name().unwrap().to_str().unwrap().to_string(), file)
            }
        }
    }

    #[cfg(windows)]
    fn root_fname(dir: &PathBuf) -> String {
        let mut dir = dir.as_os_str().to_str().unwrap().to_string();

        dir[dir.len() - 3..dir.len() - 2].to_string()
    }

    #[cfg(not(windows))]
    fn root_fname(_: &PathBuf) -> String {
        "root".to_string()
    }
}


impl DepthSetting {
    /// Check if this depth can go one level deeper.
    ///
    /// If so, `next_level()` will return `Some`, otherwise, `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// assert!(DepthSetting::Infinite.can_recurse());
    ///
    /// assert_false!(DepthSetting::LastLevel.can_recurse());
    ///
    /// assert!(DepthSetting::NRemaining(1).can_recurse());
    /// assert!(DepthSetting::NRemaining(100).can_recurse());
    /// ```
    pub fn can_recurse(&self) -> bool {
        match self {
            &DepthSetting::Infinite |
            &DepthSetting::NRemaining(_) => true,
            &DepthSetting::LastLevel => false,
        }
    }

    /// Get the next recursion level, if one exists.
    ///
    /// The next recursion level does *not* exist only for `LastLevel`.
    ///
    /// # Examples
    ///
    /// ```
    /// // Normally you'd acquire from elsewhere.
    /// let depth = DepthSetting::NRemaining(1);
    ///
    /// if let Some(next) = depth.next_level() {
    ///     // Recurse into the next level...
    ///     assert_eq!(next, DepthSetting::LastLevel);
    /// }
    /// ```
    pub fn next_level(&self) -> Option<Self> {
        match self {
            &DepthSetting::Infinite => Some(DepthSetting::Infinite),
            &DepthSetting::NRemaining(n) => Some(Self::from(n as i32 - 1)),
            &DepthSetting::LastLevel => None,
        }
    }
}

impl FromStr for DepthSetting {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        i32::from_str(&s).map(DepthSetting::from).map_err(|e| e.to_string())
    }
}

impl From<i32> for DepthSetting {
    fn from(n: i32) -> Self {
        if n < 0 {
            DepthSetting::Infinite
        } else if n > 0 {
            DepthSetting::NRemaining(n as u32)
        } else {
            DepthSetting::LastLevel
        }
    }
}


#[cfg(test)]
mod tests {
    mod depth {
        use self::super::super::DepthSetting;
        use std::str::FromStr;


        mod can_recurse {
            use self::super::super::super::DepthSetting;


            #[test]
            fn doctest() {
                assert!(DepthSetting::Infinite.can_recurse());

                assert!(!DepthSetting::LastLevel.can_recurse());

                assert!(DepthSetting::NRemaining(1).can_recurse());
                assert!(DepthSetting::NRemaining(100).can_recurse());
            }
        }

        mod next_level {
            use self::super::super::super::DepthSetting;


            #[test]
            fn doctest() {
                // Normally you'd acquire from elsewhere.
                let depth = DepthSetting::NRemaining(1);

                if let Some(next) = depth.next_level() {
                    // Recurse into the next level...
                    assert_eq!(next, DepthSetting::LastLevel);
                }
            }

            #[test]
            fn infinite() {
                assert_eq!(DepthSetting::Infinite.next_level(), Some(DepthSetting::Infinite));
            }

            #[test]
            fn last_level() {
                assert_eq!(DepthSetting::LastLevel.next_level(), None);
            }

            #[test]
            fn nremaining() {
                assert_eq!(DepthSetting::NRemaining(1).next_level(), Some(DepthSetting::LastLevel));

                assert_eq!(DepthSetting::NRemaining(2).next_level(), Some(DepthSetting::NRemaining(1)));
                assert_eq!(DepthSetting::NRemaining(100).next_level(), Some(DepthSetting::NRemaining(99)));
            }
        }

        #[test]
        fn from_str() {

            for p in &[("-1", DepthSetting::Infinite),
                       ("-100", DepthSetting::Infinite),
                       ("0", DepthSetting::LastLevel),
                       ("1", DepthSetting::NRemaining(1)),
                       ("2", DepthSetting::NRemaining(2)),
                       ("100", DepthSetting::NRemaining(100))] {
                assert_eq!(DepthSetting::from_str(p.0).unwrap(), p.1);
            }
        }

        #[test]
        fn from_str_bad() {
            for s in &["a234", "1231d"] {
                DepthSetting::from_str(s).unwrap_err();
            }
        }
    }
}
