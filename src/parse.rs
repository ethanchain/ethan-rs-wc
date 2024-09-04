/*!
The parse command line parameters.
*/
use clap::{arg, value_parser, Arg, ArgAction, Command};
// use std::env;
use std::ffi::OsString;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ParseMode {
    Normal,
    Cli,
}

impl Default for ParseMode {
    fn default() -> Self {
        ParseMode::Normal
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum MutliMode {
    Single,
    Mutilple,
}

impl Default for MutliMode {
    fn default() -> Self {
        MutliMode::Single
    }
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
pub(crate) struct CliSimpCfg {
    pub(crate) characters_flag: bool,
    pub(crate) lines_flag: bool,
    pub(crate) words_flag: bool,
    pub(crate) longest_line_flag: bool,
    pub(crate) bytes_flag: bool,
}

#[derive(Debug, Default, Eq, PartialEq)]
pub(crate) struct CliConfig {
    pub(crate) cli_simp_cfg: CliSimpCfg,
    pub(crate) filepaths: Vec<String>,
    pub(crate) parse_mode: ParseMode,
    pub(crate) multi_mode: MutliMode,
}

impl CliConfig {
    pub(crate) fn build<I, T>(args: I) -> Result<CliConfig, &'static str>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let matches: clap::ArgMatches = Command::new("erwc")
            .about("ethan's rust of word, line, character, and byte count")
            .version("0.1.0")
            .arg(
                arg!(characters: -c)
                    .long("characters")
                    .action(ArgAction::SetTrue)
                    .help("The number of characters."),
            )
            .arg(
                arg!(lines: -l)
                    .long("lines")
                    .action(ArgAction::SetTrue)
                    .help("The number of lines."),
            )
            .arg(
                arg!(words: -w)
                    .long("words")
                    .action(ArgAction::SetTrue)
                    .help("The number of words."),
            )
            .arg(
                arg!(bytes: -b)
                    .long("bytes")
                    .action(ArgAction::SetTrue)
                    .help("The number of bytes."),
            )
            .arg(
                Arg::new("longest-line")
                    .short('L')
                    .long("longest-line")
                    .action(ArgAction::SetTrue)
                    .help("The longest line of provide text content, (The number of bytes)@(The line number of the longest line)."),
            )
            .arg(
                arg!(filepath: [FILEPATH])
                    .num_args(1..)
                    .value_parser(value_parser!(String))
                    .required(false)
                    .help("The file paths, option. None one or many. With no files provide, will accept standard input util receing EOF, or [^D] in most."),
            )
            .get_matches_from(args);
        let filepaths = matches
            .get_many::<String>("filepath")
            .map(|vals| vals.cloned().collect::<Vec<_>>())
            .unwrap_or_default();
        let file_path_len = filepaths.len();
        let mut multi_mode = MutliMode::default();
        let parse_mode = if file_path_len <= 0 {
            ParseMode::Cli
        } else {
            if file_path_len > 1 {
                multi_mode = MutliMode::Mutilple;
            }
            ParseMode::Normal
        };
        Ok(CliConfig {
            cli_simp_cfg: CliSimpCfg {
                characters_flag: matches.get_flag("characters"),
                lines_flag: matches.get_flag("lines"),
                words_flag: matches.get_flag("words"),
                longest_line_flag: matches.get_flag("longest-line"),
                bytes_flag: matches.get_flag("bytes"),
            },
            filepaths,
            parse_mode,
            multi_mode,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_characters_flag_true() {
        let args = vec![
            OsString::from("target/debug/erwc"),
            OsString::from("-c"),
            OsString::from("test.txt"),
        ];
        let cli_cfg = CliConfig::build(args).unwrap();
        assert_eq!(cli_cfg.cli_simp_cfg.characters_flag, true);
        assert_eq!(cli_cfg.cli_simp_cfg.lines_flag, false);
        assert_eq!(cli_cfg.cli_simp_cfg.words_flag, false);
        assert_eq!(cli_cfg.cli_simp_cfg.longest_line_flag, false);
        assert_eq!(cli_cfg.cli_simp_cfg.bytes_flag, false);
    }

    #[test]
    fn check_all_flag_true() {
        let args = vec![
            OsString::from("target/debug/erwc"),
            OsString::from("-clwbL"),
            OsString::from("test.txt"),
        ];
        let cli_cfg = CliConfig::build(args).unwrap();
        assert_eq!(cli_cfg.cli_simp_cfg.characters_flag, true);
        assert_eq!(cli_cfg.cli_simp_cfg.lines_flag, true);
        assert_eq!(cli_cfg.cli_simp_cfg.words_flag, true);
        assert_eq!(cli_cfg.cli_simp_cfg.longest_line_flag, true);
        assert_eq!(cli_cfg.cli_simp_cfg.bytes_flag, true);
    }

    #[test]
    fn check_all_flag_false() {
        let args = vec![
            OsString::from("target/debug/erwc"),
            OsString::from("test.txt"),
        ];
        let cli_cfg = CliConfig::build(args).unwrap();
        assert_ne!(cli_cfg.cli_simp_cfg.characters_flag, true);
        assert_ne!(cli_cfg.cli_simp_cfg.lines_flag, true);
        assert_ne!(cli_cfg.cli_simp_cfg.words_flag, true);
        assert_ne!(cli_cfg.cli_simp_cfg.longest_line_flag, true);
        assert_ne!(cli_cfg.cli_simp_cfg.bytes_flag, true);
    }
}
