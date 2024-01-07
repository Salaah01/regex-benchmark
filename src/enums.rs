//! Contains all the enums used in the crate.

/// Enum used to represent the time unit used in the graph.
#[derive(Debug)]
pub enum TimeUnit {
    Nanoseconds,
    Microseconds,
    Milliseconds,
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
