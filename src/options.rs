//! Option parsing and management.
//!
//! Use the `Options::parse()` function to get the program's configuration,
//! as parsed from the commandline.
//!
//! # Examples
//!
//! ```skip
//! let opts = Options::parse();
//! println!("{:#?}", opts);
//! ```


use clap::{self, App, Arg, AppSettings};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use self::super::Algorithm;
use std::str::FromStr;
use num_cpus;
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
    /// Max recursion depth. Infinite if None. Default: `0`
    pub depth: Option<usize>,
    /// In-/Output filename. Default: `"./INFERRED_FROM_DIRECTORY.hash"`
    pub file: (String, PathBuf),
    /// Whether to recurse down symlinks. Default: `true`
    pub follow_symlinks: bool,
    /// Files/directories to ignore. Default: none
    pub ignored_files: BTreeSet<String>,
    /// # of threads used for hashing. Default: # of CPU threads
    pub jobs: u32,
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
                                     Supported algorithms: SHA{1,2-{256,512},3-{256,512}}, BLAKE{,2}, CRC{64,32,16,8}, MD{5,6-{128,256,512}}, XOR8'")
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
                    Arg::from_usage("--force 'Override output file'"),
                    Arg::from_usage("--follow-symlinks 'Recurse down symlinks. Default: yes'").overrides_with("no-follow-symlinks"),
                    Arg::from_usage("--no-follow-symlinks 'Don\'t recurse down symlinks'").overrides_with("follow-symlinks"),
                    Arg::from_usage("-i --ignore [file]... 'Ignore specified file(s)'"),
                    Arg::from_usage("-j --jobs=[jobs] '# of threads used for hashing. No/empty value: # of CPU threads. -1: Infinite'")
                        .empty_values(true)
                        .validator(Options::jobs_validator)])
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
                None
            } else {
                let i = matches.value_of("depth").map(|s| s.parse::<isize>().unwrap()).unwrap_or(0);
                if i < 0 {
                    None
                } else {
                    Some(i as usize)
                }
            },
            file: file,
            follow_symlinks: !matches.is_present("no-follow-symlinks"),
            ignored_files: matches.values_of("ignore").map(|v| v.map(String::from).collect()).unwrap_or(BTreeSet::new()),
            jobs: match matches.value_of("jobs") {
                None | Some("") => num_cpus::get() as u32,
                Some(s) => {
                    match i32::from_str(s).unwrap() {
                        -1 => u32::max_value(),
                        i => i as u32,
                    }
                }
            },
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
        s.parse::<isize>().map(|_| ()).map_err(|e| e.to_string())
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

    fn jobs_validator(s: String) -> Result<(), String> {
        if s.is_empty() {
            Ok(())
        } else {
            i32::from_str(&s).map_err(|e| format!("jobs: {}", e)).and_then(|i| {
                if i == 0 {
                    Err("cannot execute 0 jobs".to_string())
                } else if i < -1 {
                    Err("cannot execute a negative amount of jobs".to_string())
                } else {
                    Ok(())
                }
            })
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
    fn root_fname(dir: &Path) -> String {
        let dir = dir.as_os_str().to_str().unwrap().to_string();
        dir[dir.len() - 3..dir.len() - 2].to_string()
    }

    #[cfg(not(windows))]
    fn root_fname(_: &Path) -> String {
        "root".to_string()
    }
}
