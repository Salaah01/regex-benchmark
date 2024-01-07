//! Contains all the enums used in the crate.

use core::fmt;

/// Enum used to represent the time unit used in the graph.
#[derive(Debug)]
pub enum TimeUnit {
    Nanoseconds,
    Microseconds,
    Milliseconds,
}

impl fmt::Display for TimeUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeUnit::Nanoseconds => write!(f, "ns"),
            TimeUnit::Microseconds => write!(f, "μs"),
            TimeUnit::Milliseconds => write!(f, "ms"),
        }
    }
}

/// Enum used to represent the method used to test the regex expression.
#[derive(Copy, Clone, Debug)]
pub enum RegexMethod {
    Match,
    Find,
    FindIter,
}

impl RegexMethod {
    pub fn options_as_str() -> &'static str {
        "match, find, find_iter"
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_can_get_display_for_nano_seconds() {
        assert_eq!(format!("{}", TimeUnit::Nanoseconds), "ns");
    }

    #[test]
    fn test_can_get_display_for_micro_seconds() {
        assert_eq!(format!("{}", TimeUnit::Microseconds), "μs");
    }

    #[test]
    fn test_can_get_display_for_milli_seconds() {
        assert_eq!(format!("{}", TimeUnit::Milliseconds), "ms");
    }
}
