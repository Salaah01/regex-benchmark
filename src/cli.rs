//! Contains CLI related code.

use crate::enums;

use clap::{ArgAction, Parser};
use enums::RegexMethod;
use regex_lite::Regex;

impl std::str::FromStr for RegexMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "match" => Ok(RegexMethod::Match),
            "find" => Ok(RegexMethod::Find),
            "find_iter" => Ok(RegexMethod::FindIter),
            _ => Err(format!("{} is not a valid regex method", s)),
        }
    }
}

/// The command line arguments passed to the program.
/// Run `regex-speed --help` to see the help message and more information about
/// the arguments.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, help = "The regex to test the random strings against")]
    pub regex: Regex,

    #[clap(
        short,
        long,
        help = format!("The method to be used to test the regex expression. [options: {}]", RegexMethod::options_as_str()),
        default_value = "match"
    )]
    pub method: RegexMethod,

    #[clap(
        short = 'a',
        long,
        help = "The minimum length of the random strings",
        default_value = "1"
    )]
    pub min_length: usize,

    #[clap(short = 'b', long, help = "The maximum length of the random strings")]
    pub max_length: usize,

    #[clap(
        short,
        long,
        help = "The step size between the lengths of the random strings",
        default_value = "1"
    )]
    pub step_size: usize,

    #[clap(
        short,
        long,
        help = "The number of tests to be carried out for a single length of random string"
    )]
    pub num_tests: usize,

    #[clap(
        short = 'R',
        long,
        help = "An optional string that must appear in the random strings"
    )]
    pub required_str: Option<String>,

    #[clap(short, long, help = "Verbose output", action=ArgAction::SetTrue)]
    pub verbose: bool,
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
