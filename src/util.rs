//! Module containing various utility functions


use std::path::Path;
use std::iter;


/// Merges two `Vec`s.
///
/// # Examples
///
/// ```
/// let vec1 = vec![0];
/// let vec2 = vec![1];
///
/// assert_eq!(checksums::util::vec_merge(vec1, vec2), vec![0, 1]);
/// ```
pub fn vec_merge<T>(mut lhs: Vec<T>, rhs: Vec<T>) -> Vec<T> {
    lhs.extend(rhs);
    lhs
}

/// Create a string consisting of `n` repetitions of `what`.
///
/// # Examples
///
/// ```
/// assert_eq!(checksums::util::mul_str("DIE! ", 3), "DIE! DIE! DIE! ".to_string());
/// ```
pub fn mul_str(what: &str, n: usize) -> String {
    iter::repeat(what).take(n).collect()
}

/// Create a user-usable path to `what` from `prefix`.
///
/// # Examples
///
/// ```
/// # use std::path::Path;
/// assert_eq!(checksums::util::relative_name(Path::new("/usr"), Path::new("/usr/bin/checksums")),
///            "bin/checksums".to_string());
/// ```
pub fn relative_name(prefix: &Path, what: &Path) -> String {
    what.strip_prefix(prefix).unwrap().to_str().unwrap().replace("\\", "/")
}
