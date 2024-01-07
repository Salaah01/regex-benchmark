extern crate regex_speed;
use regex_speed::calc_regex_speed::{calc_duration_for_text, SpeedTestResult};
use regex_speed::cli::parse_args;
use regex_speed::enums::TimeUnit;
use regex_speed::graph;
use regex_speed::rand_str_builder::insert_substring;

fn main() {
    let cli_args = parse_args();
    let mut speed_tests: Vec<SpeedTestResult> = Vec::new();
    let mut min_duration_nanosecs = 0;
    let mut max_duration_nanosecs = 0;
    let mut min_duration_microsecs = 0;
    let mut max_duration_microsecs = 0;
    let mut min_duration_millisecs = 0;
    let mut max_duration_millisecs = 0;

    for length in (cli_args.min_length..=cli_args.max_length).step_by(cli_args.step_size) {
        let rand_strs = regex_speed::rand_str_builder::build_rand_strs(length, cli_args.num_tests);
        for rand_str in rand_strs {
            let test_str = if let Some(required_str) = &cli_args.required_str {
                insert_substring(&rand_str, &required_str)
            } else {
                rand_str
            };
            // The first time the regex is used, it takes longer to execute.
            // So, call it once before the speed test.
            cli_args.regex.find("z");
            let speed_test_result = calc_duration_for_text(&cli_args.regex, &test_str);
            if speed_test_result.duration.as_nanos() > max_duration_nanosecs {
                max_duration_nanosecs = speed_test_result.duration.as_nanos();
                max_duration_microsecs = speed_test_result.duration.as_micros();
                max_duration_millisecs = speed_test_result.duration.as_millis();
            }
            if min_duration_nanosecs == 0
                || speed_test_result.duration.as_nanos() < min_duration_nanosecs
            {
                min_duration_nanosecs = speed_test_result.duration.as_nanos();
                min_duration_microsecs = speed_test_result.duration.as_micros();
                min_duration_millisecs = speed_test_result.duration.as_millis();
            }
            if cli_args.verbose {
                println!(
                    "Length: {}\tDuration: {:?}",
                    speed_test_result.length, speed_test_result.duration
                );
            }
            speed_tests.push(speed_test_result);
        }
    }

    let units: TimeUnit;
    let max_y: i32;
    let min_y: i32;

    if max_duration_nanosecs < 1_000 {
        units = TimeUnit::Nanoseconds;
        max_y = max_duration_nanosecs as i32;
        min_y = min_duration_nanosecs as i32;
    } else if max_duration_microsecs < 1_000 {
        units = TimeUnit::Microseconds;
        max_y = max_duration_microsecs as i32;
        min_y = min_duration_microsecs as i32;
    } else {
        units = TimeUnit::Milliseconds;
        max_y = max_duration_millisecs as i32;
        min_y = min_duration_millisecs as i32;
    }

    graph::create(
        speed_tests,
        cli_args.max_length as i32,
        min_y,
        max_y,
        &units,
    )
    .unwrap();
}
