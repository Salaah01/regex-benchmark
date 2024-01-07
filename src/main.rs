extern crate regex_speed;
use regex_speed::calc_regex_speed::{calc_duration_for_text, SpeedTestResult};
use regex_speed::cli::parse_args;
use regex_speed::rand_str_builder::insert_substring;

fn main() {
    let cli_args = parse_args();
    let mut speed_tests: Vec<SpeedTestResult> = Vec::new();

    for length in (cli_args.min_length..=cli_args.max_length).step_by(cli_args.step_size) {
        let rand_strs = regex_speed::rand_str_builder::build_rand_strs(length, cli_args.num_tests);
        for rand_str in rand_strs {
            let test_str = if let Some(required_str) = &cli_args.required_str {
                insert_substring(&rand_str, &required_str)
            } else {
                rand_str
            };
            let speed_test_result = calc_duration_for_text(&cli_args.regex, &test_str);
            println!(
                "{}\t{:?}",
                speed_test_result.length, speed_test_result.duration
            );
            speed_tests.push(speed_test_result);
        }
    }
}
