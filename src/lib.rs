//! Tool for making/verifying checksums of directory trees.
//!
//! # Library doc
//!
//! This library is used by `checksums` itself for all its function and is therefore contains all necessary functions.
//!
//! ## Data flow
//!
//! Hash verification
//!
//! ```plaintext
//! Options
//! |> create_hashes()
//! |> compare_hashes()
//! |> write_hash_comparison_results()
//! ```
//!
//! Hash creation
//!
//! ```plaintext
//! Options
//! |> create_hashes()
//! |> write_hashes()
//! ```
//!
//! # Executable manpage
//!
//! ## SYNOPSIS
//!
//! [`checksums`](https://github.com/nabijaczleweli/checksums) [OPTIONS] [DIRECTORY]
//!
//! ## DESCRIPTION
//!
//! Tool for making/verifying checksums of directory trees.
//!
//! Use the generated checksums to automatically verify file/directory tree
//! correctness.
//!
//! All output is wrapped to 80 columns.
//!
//! ## OPTIONS
//!
//! -a --algorithm &lt;algorithm&gt;
//!
//! ```text
//! Set the hashing algorithm to use.
//!
//! Supported algorithms: SHA1, SHA2-256, SHA2-512, SHA3-256, SHA3-512, BLAKE,
//!                       BLAKE2, CRC8, CRC16, CRC32, CRC64, MD5, MD6-128,
//!                       MD6-256, MD6-512, XOR8
//! ```
//!
//! -c --create
//!
//! ```text
//! Create directory hashes, rather than verifying them.
//!
//! Directory hashes are output to the output file, which, if not specified, will
//! be "`DIRECTORY`.hash".
//!
//! Will fail if the output file already exists and `--force` is not specified.
//!
//! Exclusive with `--verify`. Overrides `--verify`.
//! ```
//!
//! -v --verify
//!
//! ```text
//! Verify directory hashes. Default.
//!
//! Exclusive with `--create`. Overrides `--create`.
//! ```
//!
//! -d --depth &lt;depth&gt;
//!
//! ```text
//! Set max recursion depth to `depth`. Default: 0.
//!
//! Exclusive with `--recursive`. Overrides `--recursive`.
//! ```
//!
//! -r --recursive
//!
//! ```text
//! Set max recursion depth to infinity.
//!
//! Exclusive with `--depth`. Overrides `--depth`.
//! ```
//!
//! --follow-symlinks
//!
//! ```text
//! Recurse down symlinks. Default.
//! ```
//!
//! --no-follow-symlinks
//!
//! ```text
//! Don't recurse down symlinks.
//! ```
//!
//! -i --ignore &lt;filename[,filename2][,filename3][,filenameN]...&gt;...
//!
//! ```text
//! Add filename(s) to ignored files list. Default: none.
//!
//! Ignored files are marked as such.
//!
//! Accepted multiple times.
//! ```
//!
//! --force
//!
//! ```text
//! Override output file in `--create` mode. No meaning in `--verify` mode.
//! ```
//!
//! -j --jobs [jobs]
//!
//! ```text
//! Amount of threads used for hashing. Default: # of CPU threads
//!
//! One thread can hash one file at a time, potentially speeding up hashing
//! up to `jobs` times.
//!
//! No/empty value: # of CPU threads. -1: Infinite
//! ```
//!
//! [DIRECTORY]
//!
//! ```text
//! Directory to create/verify hash for. Default: current workdir.
//! ```
//!
//! ## EXAMPLES
//!
//! `checksums` [`-v`] [`-f` *infile*]
//!
//! ```text
//! Verify the current directory tree against the saved hashes.
//!
//! `-v` is not necessary as it's the default.
//!
//! *infile* defaults to "`DIRECTORY`.hash"
//!
//! Example output:
//!   File added: "file_that_was_not_here_before"
//!   File removed: "file_that_was_here_before_but_not_now"
//!   File ignored: "file_specified_with_ignore_now_or_during_creation"
//!
//!   File "file_that_did_not_change" matches
//!   File "changed_file" doesn't match
//!     Was: 8313958F86F7B15D4775D12886D479C1CFAAA111
//!     Is : FCFC1548B30B5ACB25A7421D068E12F07DF74DCC
//! ```
//!
//! `examples` `-c` [`-f` *outfile*] [`--force`]
//!
//! ```text
//! Create hashes of the current directory tree for later verification.
//!
//! *outfile* defaults to "`DIRECTORY`.hash".
//!
//! Use `--force` to override *outfile*.
//!
//! Example output:
//!   FILENAME 722 / 722 [===============================================] 100.00 %
//!
//! *outfile* contents:
//!   a_file.txt      8313958F86F7B15D4775D12886D479C1CFAAA111
//!   *outfile*.hash  ----------------------------------------
//!   different_file  8D742C1F2D39434771039E98AD854C72F91FCCA5
//! ```
//!
//! `examples` [`-d` *depth*] [`-r`] [`OTHER OPTIONS`]
//!
//! ```text
//! Recurse *depth* or infinity directories down.
//!
//! Example output for *depth*=2:
//!   File "dir1/dir2/file" matches
//!   File "dir1/file" matches
//!   File "file" matches
//! ```


extern crate md5;
extern crate md6;
extern crate pbr;
extern crate crc;
#[macro_use]
extern crate clap;
extern crate crc8;
extern crate crc16;
extern crate blake;
extern crate regex;
extern crate shaman;
extern crate futures;
extern crate num_cpus;
extern crate tabwriter;
extern crate blake2_rfc;
#[macro_use]
extern crate lazy_static;
extern crate tiny_keccak;
extern crate futures_cpupool;

mod hashing;
mod algorithms;

pub mod ops;
pub mod util;
pub mod options;

pub use hashing::*;
pub use algorithms::Algorithm;
