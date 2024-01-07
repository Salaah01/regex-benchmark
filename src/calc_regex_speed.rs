//! This module is used to calculate the speed of a particular regex
//! expression.

use regex::Regex;
use std::time::{Duration, Instant};

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

/// This function is used to calculate the speed of a particular regex
/// expression.
/// # Arguments
/// * `regex` - The regex expression to be tested.
/// * `text` - The text to be tested against the regex expression.
///
/// # Returns
/// SpeedTestResult - The result of the speed test.
pub fn calc_duration_for_text(regex: &Regex, text: &str) -> SpeedTestResult {
    let start = Instant::now();
    regex.find(text);
    SpeedTestResult::new(text.len(), start.elapsed())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_duration_for_text() {
        let regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let text = "2012-03-14, 2013-01-01 and 2014-07-05";
        let result = calc_duration_for_text(&regex, text);
        assert!(result.duration > Duration::from_secs(0));
    }
}
