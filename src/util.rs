//! Module containing various utility functions


use std::collections::BTreeMap;
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

// doc copied from BTreeMap::append()
/// Moves all elements from `what` into `to`.
///
/// # Examples
///
/// ```
/// use std::collections::BTreeMap;
///
/// let mut a = BTreeMap::new();
/// a.insert(1, "a");
/// a.insert(2, "b");
/// a.insert(3, "c");
///
/// let mut b = BTreeMap::new();
/// b.insert(3, "d");
/// b.insert(4, "e");
/// b.insert(5, "f");
///
/// checksums::util::btreemap_append(&mut a, b);
///
/// assert_eq!(a.len(), 5);
///
/// assert_eq!(a[&1], "a");
/// assert_eq!(a[&2], "b");
/// assert_eq!(a[&3], "d");
/// assert_eq!(a[&4], "e");
/// assert_eq!(a[&5], "f");
/// ```
pub fn btreemap_append<K: Ord, V>(to: &mut BTreeMap<K, V>, what: BTreeMap<K, V>) {
    for (k, v) in what {
        to.insert(k, v);
    }
}

/// Create a string consisting of `n` repetitions of `what`.
///
/// # Examples
///
/// ```
/// assert_eq!(checksums::util::mul_str("DIE! ", 3), "DIE! DIE! DIE! ".to_string());
/// ```
pub fn mul_str(what: &str, n: usize) -> String{
    iter::repeat(what).take(n).collect()
}
