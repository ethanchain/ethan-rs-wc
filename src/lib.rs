/*!
The crate library of ethan-rs-wc(erwc).
*/
use parse::{CliConfig, ParseMode};
use rayon::prelude::*;
use stat::{Stat, StatData};
use std::{env, ffi::OsString, process};

mod parse;
mod stat;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Mode {}

impl Mode {
    /// Parse parameters, statistical data
    ///
    /// Parse command line configuration information based on command line parameters.
    /// It may or may not read a file, multiple files, or it may read standard input text from the command line.
    /// Use this to collect statistics based on the command line parameter configuration.
    /// Using Rayon's artifact iterator for bulk file processing.
    pub fn run(args: env::ArgsOs) {
        run(args);
    }
}

/// Refactor code for simulated testing.
fn run<I, T>(args: I)
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let args: Vec<OsString> = args.into_iter().map(|s| s.into()).collect();
    let cli_cfg = CliConfig::build(args).unwrap_or_else(|err| {
        eprintln!("Parse CLI config error: {err}");
        process::exit(1);
    });
    let parse_mode = cli_cfg.parse_mode;
    match parse_mode {
        ParseMode::Cli => {
            StatData::read_std_in_contents(cli_cfg.cli_simp_cfg);
            process::exit(0);
        }
        ParseMode::Normal => (),
    }
    let filepaths = cli_cfg.filepaths;
    // Using Rayon's artifact iterator for bulk file processing
    let results: Vec<_> = filepaths
        .par_iter()
        .map(|file_path| {
            // Process each file
            StatData::read_file(file_path, cli_cfg.cli_simp_cfg).unwrap_or_else(
                |err| {
                    eprintln!("Read file [{}] error: {}", file_path, err);
                    process::exit(1);
                },
            )
        })
        .collect();
    if results.len() > 1 {
        Stat::print_total(results);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_with_mock_os_args() {
        // ArgsOs { inner: ["target/debug/erwc", "test.txt"] }
        let new_args = vec![
            OsString::from("target/debug/erwc"),
            OsString::from("tests/data/test.txt"),
        ];

        run(new_args);
        assert_eq!(1 + 2, 3);
    }
}
