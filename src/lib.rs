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
//! ## OPTIONS
//!
//! -a --algorithm &lt;algorithm&gt;
//!
//! ```text
//! Set the hashing algorithm to use.
//!
//! Supported algorithms: SHA1, SHA2-256, SHA2-512, SHA3-256, SHA3-512, BLAKE,
//!                       BLAKE2, CRC8, CRC16, CRC32, CRC64, MD5, XOR8
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
//! --force
//!
//! ```text
//! Override output file in `--create` mode. No meaning in `--verify` mode.
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
//! `examples` [`-v`] [`-f` *infile*]
//!
//! ```text
//! Verify the current directory tree against the saved hashes.
//!
//! `-v` is not necessary as it's the default.
//!
//! *infile* defaults to "`DIRECTORY`.hash"
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
//! ```
//!
//! `examples` [`-d` *depth*] [`-r`] [`OTHER OPTIONS`]
//!
//! ```text
//! Recurse *depth* or infinity directories down.
//! ```


extern crate md5;
extern crate crc;
#[macro_use]
extern crate clap;
extern crate crc8;
extern crate crc16;
extern crate blake;
extern crate regex;
extern crate shaman;
extern crate tabwriter;
extern crate blake2_rfc;
#[macro_use]
extern crate lazy_static;
extern crate tiny_keccak;

mod hashing;
mod algorithms;

pub mod ops;
pub mod util;
pub mod options;

pub use hashing::*;
pub use algorithms::Algorithm;
