//! Contains utility structs and functions related to the test durations.

use std::time::Duration;

use crate::enums::TimeUnit;

/// Contains the minimum and maximum duration.
pub struct MinMaxDuration {
    pub min: Duration,
    pub max: Duration,
}

impl MinMaxDuration {
    pub fn new() -> MinMaxDuration {
        MinMaxDuration {
            min: Duration::new(0, 0),
            max: Duration::new(0, 0),
        }
    }

    pub fn register_min(&mut self, duration: Duration) {
        if self.min.as_nanos() == 0 || duration.as_nanos() < self.min.as_nanos() {
            self.min = duration;
        }
    }

    pub fn register_max(&mut self, duration: Duration) {
        if duration.as_nanos() > self.max.as_nanos() {
            self.max = duration;
        }
    }

    pub fn get_as_units(&self, units: &TimeUnit) -> (u128, u128) {
        return match units {
            TimeUnit::Nanoseconds => (self.min.as_nanos(), self.max.as_nanos()),
            TimeUnit::Microseconds => (self.min.as_micros(), self.max.as_micros()),
            TimeUnit::Milliseconds => (self.min.as_millis(), self.max.as_millis()),
        };
    }
}

/// Determines the units to be used in the graph base on what the minimum and
/// maximum durations are.
pub fn get_display_units(max_duration: Duration) -> TimeUnit {
    return if max_duration.as_nanos() < 1_000 {
        TimeUnit::Nanoseconds
    } else if max_duration.as_micros() < 1_000 {
        TimeUnit::Microseconds
    } else {
        TimeUnit::Milliseconds
    };
}
