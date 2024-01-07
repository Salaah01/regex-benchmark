//! Module helps build random strings of a given length for testing purposes.

use rand::{distributions::Alphanumeric, Rng};

/// This function is used to build a random string of a given length.
/// # Arguments
/// * `length` - The length of the random string to be generated.
///
/// # Returns
/// * `String` - The random string of the given length.
///
/// # Panics
/// * `length` - If the length is less than 1.
pub fn build_rand_str(length: usize) -> String {
    if length < 1 {
        panic!("length must be greater than 0, length: {}", length);
    }
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

/// Retrieves a certain number of random strings of a given length for testing
/// purposes.
///
/// # Arguments
/// * `length` - The length of the random string to be generated.
/// * `num` - The number of random strings to be generated.
///
/// # Returns
/// * `Vec<String>` - The vector of random strings of the given length.
pub fn build_rand_strs(length: usize, num: usize) -> Vec<String> {
    let mut rand_strs = Vec::new();
    for _ in 0..num {
        rand_strs.push(build_rand_str(length));
    }
    rand_strs
}

/// Randomly inserts a substring into a string.
/// # Arguments
/// * `string` - The string to be inserted into.
/// * `substring` - The substring to be inserted.
///
/// # Returns
/// * `String` - The string with the substring inserted.
pub fn insert_substring(string: &str, substring: &str) -> String {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..string.to_string().len());
    let mut result = String::new();
    result.push_str(&string[..index]);
    result.push_str(substring);
    result.push_str(&string[index..]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_build_rand_str_zero_length() {
        build_rand_str(0);
    }

    #[test]
    fn test_build_rand_str() {
        let rand_str = build_rand_str(10);
        assert_eq!(rand_str.len(), 10);
    }

    #[test]
    fn test_build_rand_strs() {
        let rand_strs = build_rand_strs(10, 3);
        assert_eq!(rand_strs.len(), 3);
        assert_eq!(rand_strs[0].len(), 10);
        assert_eq!(rand_strs[1].len(), 10);
        assert_eq!(rand_strs[2].len(), 10);
    }
}
