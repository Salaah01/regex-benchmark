//! Contains CLI related code.

use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub regex: Regex,
    #[arg(short = 'a', long)]
    pub min_length: usize,
    #[arg(short = 'b', long)]
    pub max_length: usize,
    #[arg(short, long)]
    pub step_size: usize,
    #[arg(short, long)]
    pub num_tests: usize,
    #[arg(short = 'R', long)]
    pub required_str: Option<String>,
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
    let args = Args::parse();
    if args.max_length < 1 {
        panic!("max_length must be greater than 0");
    }
    if args.num_tests < 1 {
        panic!("num_tests must be greater than 0");
    }
    args
}
