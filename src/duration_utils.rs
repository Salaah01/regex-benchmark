//! Contains utility structs and functions related to the test durations.

use std::time::Duration;

use crate::enums::TimeUnit;

/// Contains the minimum and maximum duration.
pub struct MinMaxDuration {
    pub min: Duration,
    pub max: Duration,
}

impl MinMaxDuration {
    /// Creates a new instance of the struct with the minimum and maximum
    /// durations set to 0.
    pub fn new() -> MinMaxDuration {
        MinMaxDuration {
            min: Duration::new(0, 0),
            max: Duration::new(0, 0),
        }
    }

    /// Attempts to register the given duration as the new minimum.
    /// If the given duration is less than the current minimum, it is set as
    /// the new minimum. Otherwise, nothing happens.
    ///
    /// Arguments:
    /// * `duration` - The duration to be registered.
    ///
    /// Returns:
    /// * `()` - Nothing.
    ///
    /// Example:
    /// ```
    /// use std::time::Duration;
    /// use regex_benchmark::duration_utils::MinMaxDuration;
    ///
    /// let mut min_max_duration = MinMaxDuration::new();
    ///
    /// min_max_duration.register_min(Duration::new(0, 100));
    /// assert_eq!(min_max_duration.min.as_nanos(), 100);
    ///
    /// min_max_duration.register_min(Duration::new(0, 50));
    /// assert_eq!(min_max_duration.min.as_nanos(), 50);
    ///
    /// min_max_duration.register_min(Duration::new(0, 200));
    /// assert_eq!(min_max_duration.min.as_nanos(), 50);
    /// ```
    pub fn register_min(&mut self, duration: Duration) {
        if self.min.as_nanos() == 0 || duration.as_nanos() < self.min.as_nanos() {
            self.min = duration;
        }
    }

    /// Attempts to register the given duration as the new maximum.
    /// If the given duration is greater than the current maximum, it is set as
    /// the new maximum. Otherwise, nothing happens.
    ///
    /// Arguments:
    /// * `duration` - The duration to be registered.
    ///
    /// Returns:
    /// * `()` - Nothing.
    ///
    /// Example:
    /// ```
    /// use std::time::Duration;
    /// use regex_benchmark::duration_utils::MinMaxDuration;
    ///
    /// let mut min_max_duration = MinMaxDuration::new();
    ///
    /// min_max_duration.register_max(Duration::new(0, 100));
    /// assert_eq!(min_max_duration.max.as_nanos(), 100);
    ///
    /// min_max_duration.register_max(Duration::new(0, 50));
    /// assert_eq!(min_max_duration.max.as_nanos(), 100);
    ///
    /// min_max_duration.register_max(Duration::new(1, 0));
    /// assert_eq!(min_max_duration.max.as_nanos(), 1_000_000_000);
    /// ```
    pub fn register_max(&mut self, duration: Duration) {
        if duration.as_nanos() > self.max.as_nanos() {
            self.max = duration;
        }
    }

    /// Returns the minimum and maximum durations in the given time unit.
    /// The minimum and maximum durations are converted to the given time unit
    /// before being returned.
    ///
    /// Arguments:
    /// * `units` - The time unit to convert the durations to.
    ///
    /// Returns:
    /// * `(u128, u128)` - A tuple containing the minimum and maximum durations
    /// in the given time unit.
    pub fn get_as_units(&self, units: &TimeUnit) -> (u128, u128) {
        (
            duration_repr(self.min, units),
            duration_repr(self.max, units),
        )
    }
}

/// Determines the units to be used in the graph base on what the minimum and
/// maximum durations are.
/// 
/// Arguments:
/// * `max_duration` - The maximum duration.
/// 
/// Returns:
/// * `TimeUnit` - The time unit to be used in the graph.
pub fn get_display_units(max_duration: Duration) -> TimeUnit {
    return if max_duration.as_nanos() < 1_000 {
        TimeUnit::Nanoseconds
    } else if max_duration.as_micros() < 1_000 {
        TimeUnit::Microseconds
    } else {
        TimeUnit::Milliseconds
    };
}

/// Function to convert duration to a particular time unit.
///
/// Arguments:
/// * `duration` - The duration to be converted.
/// * `units` - The time unit to convert the duration to.
///
/// Returns:
/// * `u128` - The duration in the given time unit.
pub fn duration_repr(duration: Duration, units: &TimeUnit) -> u128 {
    match units {
        TimeUnit::Nanoseconds => duration.as_nanos(),
        TimeUnit::Microseconds => duration.as_micros(),
        TimeUnit::Milliseconds => duration.as_millis(),
    }
}
