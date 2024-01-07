extern crate regex_speed;
use regex_speed::calc_regex_speed::{calc_duration_for_text, SpeedTestResult};
use regex_speed::cli::{parse_args, Args};
use regex_speed::duration_utils::{get_display_units, MinMaxDuration};
use regex_speed::graph;
use regex_speed::rand_str_builder::{build_rand_strs, insert_substring};

/// Calculates the speed of the regex for the given CLI arguments.
///
/// Arguments:
/// * `cli_args` - The CLI arguments.
///
/// Returns:
/// * `Vec<SpeedTestResult>` - The results of the speed tests.
/// * `MinMaxDuration` - The minimum and maximum durations.
fn calc_results(cli_args: &Args) -> (Vec<SpeedTestResult>, MinMaxDuration) {
    let mut speed_tests: Vec<SpeedTestResult> = Vec::new();
    let mut min_max_duration = MinMaxDuration::new();

    for length in (cli_args.min_length..=cli_args.max_length).step_by(cli_args.step_size) {
        let rand_strs = build_rand_strs(length, cli_args.num_tests);
        for rand_str in rand_strs {
            let test_str = if let Some(required_str) = &cli_args.required_str {
                insert_substring(&rand_str, &required_str)
            } else {
                rand_str
            };
            // The first time the regex is used, it takes longer to execute.
            // So, call it once before the speed test.
            cli_args.regex.captures("z");
            let speed_test_result =
                calc_duration_for_text(&cli_args.regex, cli_args.method, &test_str);
            min_max_duration.register_min(speed_test_result.duration);
            min_max_duration.register_max(speed_test_result.duration);

            if cli_args.verbose {
                println!(
                    "Length: {}\tDuration: {:?}",
                    speed_test_result.length, speed_test_result.duration
                );
            }
            speed_tests.push(speed_test_result);
        }
    }

    (speed_tests, min_max_duration)
}

/// Builds the graph for the given results.
///
/// Arguments:
/// * `cli_args` - The CLI arguments.
/// * `speed_tests` - The results of the speed tests.
/// * `min_max_duration` - The minimum and maximum durations.
fn build_graph(
    cli_args: &Args,
    speed_tests: Vec<SpeedTestResult>,
    min_max_duration: MinMaxDuration,
) {
    let units = get_display_units(min_max_duration.max);
    let (min_y, max_y) = min_max_duration.get_as_units(&units);

    graph::create(
        speed_tests,
        cli_args.max_length as i32,
        min_y as i32,
        max_y as i32,
        &units,
    )
    .unwrap();
}

fn main() {
    let cli_args = parse_args();
    let (speed_tests, min_max_duration) = calc_results(&cli_args);
    build_graph(&cli_args, speed_tests, min_max_duration);
    println!("Done! Opening graph.png...");
    open::that("graph.png").unwrap();
}
