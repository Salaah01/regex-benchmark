//! Contains CLI related code.

use regex::Regex;
use std::env;

pub struct Args {
    pub regex: Regex,
    pub min_length: usize,
    pub step_size: usize,
    pub max_length: usize,
    pub num_tests: usize,
    pub required_str: Option<String>,
}

impl Args {
    pub fn new(
        regex: Regex,
        min_length: usize,
        step_size: usize,
        max_length: usize,
        num_tests: usize,
        required_str: Option<String>,
    ) -> Args {
        Args {
            regex,
            min_length,
            step_size,
            max_length,
            num_tests,
            required_str,
        }
    }
}

/// This function is used to retrieve the command line arguments passed to the
/// program.
///
/// # Returns
///
/// * `Args` - The command line arguments passed to the program stored in a
/// struct.
///
/// # Panics
///
/// * If the number of command line arguments is not 4.
/// * If the upper limit of the length of the random strings is less than 1.
/// * If the number of tests to be carried out for a single length of random
/// string is less than 1.
/// * If the upper limit of the length of the random strings is not a number.
pub fn parse_args() -> Args {
    let mut args = env::args().skip(1);
    let regex = Regex::new(&args.next().unwrap()).unwrap();
    let min_length = args.next().unwrap().parse::<usize>().unwrap();
    let max_length = args.next().unwrap().parse::<usize>().unwrap();
    let step_size = args.next().unwrap().parse::<usize>().unwrap();
    let num_tests = args.next().unwrap().parse::<usize>().unwrap();

    if max_length < 1 {
        panic!("max_length must be greater than 0");
    }
    if num_tests < 1 {
        panic!("num_tests must be greater than 0");
    }

    let mut required_str = None;
    if let Some(string) = args.next() {
        required_str = Some(string);
    }

    Args::new(
        regex,
        min_length,
        step_size,
        max_length,
        num_tests,
        required_str,
    )
}
