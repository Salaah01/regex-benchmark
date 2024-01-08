//! This module is used to calculate the speed of a particular regex
//! expression.

use regex_lite::Regex;
use std::time::{Duration, Instant};

use crate::enums::RegexMethod;

/// This struct is used to store the result of a speed test.
pub struct SpeedTestResult {
    pub length: usize,
    pub duration: Duration,
}

impl SpeedTestResult {
    /// This function is used to create a new `SpeedTestResult`.
    /// # Arguments
    /// * `length` - The length of the random string tested.
    /// * `duration` - The time taken to execute the regex expression.
    ///
    /// # Returns
    /// * `SpeedTestResult` - The new `SpeedTestResult`.
    pub fn new(length: usize, duration: Duration) -> SpeedTestResult {
        SpeedTestResult { length, duration }
    }
}

/// This function is to return a function to be called by the test itself. This
/// allows us to test the actual speed of the regex expression without having
/// to worry about the overhead of determining which method to use.
///
/// # Arguments
/// * `regex` - The regex expression to be tested.
/// * `search_method` - The method to be used to test the regex expression.
/// * `text` - The text to be tested against the regex expression.
///
/// # Returns
/// * A function to be called by the test itself.
fn method_factory<'a>(
    regex: &'a Regex,
    search_method: RegexMethod,
    text: &'a str,
) -> impl Fn() -> bool + 'a {
    move || match search_method {
        RegexMethod::Match => regex.is_match(text),
        RegexMethod::Find => regex.find(text).is_some(),
        RegexMethod::FindIter => {
            // Iterate over all the matches and just return true as we're not
            // actually interested in the matches.
            regex.find_iter(text).for_each(|_| {});
            true
        }
    }
}

/// This function is used to calculate the speed of a particular regex
/// expression.
/// # Arguments
/// * `regex` - The regex expression to be tested.
/// * `search_method` - The method to be used to test the regex expression.
/// * `text` - The text to be tested against the regex expression.
///
/// # Returns
/// SpeedTestResult - The result of the speed test.
pub fn calc_duration_for_text(
    regex: &Regex,
    search_method: RegexMethod,
    text: &str,
) -> SpeedTestResult {
    let method = method_factory(regex, search_method, text);

    let start = Instant::now();
    method();
    let duration = start.elapsed();
    SpeedTestResult::new(text.len(), duration)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_create_speed_test_result() {
        let length = 10;
        let duration = Duration::from_secs(1);
        let result = SpeedTestResult::new(length, duration);
        assert_eq!(result.length, length);
        assert_eq!(result.duration, duration);
    }

    #[test]
    fn test_calc_duration_for_text_match_method() {
        let regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let text = "2012-03-14, 2013-01-01 and 2014-07-05";
        let result = calc_duration_for_text(&regex, RegexMethod::Match, text);
        assert!(result.duration > Duration::from_secs(0));
    }

    #[test]
    fn test_calc_duration_for_text_find_method() {
        let regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let text = "2012-03-14, 2013-01-01 and 2014-07-05";
        let result = calc_duration_for_text(&regex, RegexMethod::Find, text);
        assert!(result.duration > Duration::from_secs(0));
    }

    #[test]
    fn test_calc_duration_for_text_find_iter_method() {
        let regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let text = "2012-03-14, 2013-01-01 and 2014-07-05";
        let result = calc_duration_for_text(&regex, RegexMethod::FindIter, text);
        assert!(result.duration > Duration::from_secs(0));
    }

    #[test]
    fn test_method_factory_returns_a_fn_for_match_method() {
        let regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let text = "2012-03-14, 2013-01-01 and 2014-07-05";
        let method = method_factory(&regex, RegexMethod::Match, text);
        assert!(method());
    }

    #[test]
    fn test_method_factory_returns_a_fn_for_find_method() {
        let regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let text = "2012-03-14, 2013-01-01 and 2014-07-05";
        let method = method_factory(&regex, RegexMethod::Find, text);
        assert!(method());
    }

    #[test]
    fn test_method_factory_returns_a_fn_for_find_iter_method() {
        let regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let text = "2012-03-14, 2013-01-01 and 2014-07-05";
        let method = method_factory(&regex, RegexMethod::FindIter, text);
        assert!(method());
    }
}
