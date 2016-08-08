//! Module containing various utility functions


/// Merges two `Vec`s.
///
/// # Examples
///
/// ```
/// let vec1 = vec![0];
/// let vec2 = vec![1];
///
/// assert_eq!(vec_merge(vec1, vec2), vec![0, 1]);
/// ```
pub fn vec_merge<T>(mut lhs: Vec<T>, rhs: Vec<T>) -> Vec<T> {
    lhs.extend(rhs);
    lhs
}


#[cfg(test)]
mod tests {
    mod vec_merge {
        use self::super::super::vec_merge;


        #[test]
        fn doctest() {
            let vec1 = vec![0];
            let vec2 = vec![1];

            assert_eq!(vec_merge(vec1, vec2), vec![0, 1]);
        }
    }
}
