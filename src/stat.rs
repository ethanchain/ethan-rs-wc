/*!
The chart statistics and formatted printing.
*/
use crate::parse::CliSimpCfg;
use std::{
    fmt,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum StatData {
    // Variants that do not require specific implementation at this time.
}

/// Data dimensions
///
/// words       The number of worlds in the entire document or text.
/// lines       The number of lines in the entire document or text.
/// characters  The number of characters in the entire document or text.
/// bytes       The number of bytes in the entire document or text.
#[derive(Debug, Default, Eq, PartialEq)]
pub(crate) struct Stat {
    words: usize,
    lines: usize,
    characters: usize,
    bytes: usize,
}

// impl Default for Stat {
//     fn default() -> Self {
//         Stat { words: 0, lines: 0, characters: 0, bytes: 0 }
//     }
// }

impl StatData {
    /// Encapsulate the logic for processing each row of data.
    ///
    /// reader,  closure, following the read Buff stream to obtain text obtained from the file or standard input
    /// cli_simp_cfg, basic command line configuration, simple.
    /// source_name,    source name, file name or std-in(standard input).
    /// Returns Stat of Result.
    fn process_lines<F>(
        reader: F,
        cli_simp_cfg: CliSimpCfg,
        source_name: &str,
    ) -> Result<Stat, io::Error>
    where
        F: Fn() -> io::Result<Lines<Box<dyn BufRead>>>,
    {
        let mut stat = Stat::default();
        let mut longest_line_num: usize = 0;
        let mut line_num: usize = 0;
        let mut match_field_len: usize = 0;

        let default_flag = |cli_simp_cfg: &CliSimpCfg| -> bool {
            let words_flag = cli_simp_cfg.words_flag;
            let lines_flag = cli_simp_cfg.lines_flag;
            let characters_flag = cli_simp_cfg.characters_flag;
            !words_flag && !lines_flag && !characters_flag
        };

        // Read each row of data
        let lines = reader()?;
        for line in lines {
            let line = line?; // Process the Result and get the row content
            line_num += 1;
            let bytes_len = line.len();

            if default_flag(&cli_simp_cfg)
                || cli_simp_cfg.bytes_flag
                || cli_simp_cfg.longest_line_flag
            {
                stat.bytes += bytes_len;
            }

            if cli_simp_cfg.longest_line_flag && bytes_len > match_field_len {
                match_field_len = bytes_len;
                longest_line_num = line_num;
            }
            if default_flag(&cli_simp_cfg) || cli_simp_cfg.words_flag {
                stat.words += line.split_whitespace().count();
            }
            if default_flag(&cli_simp_cfg) || cli_simp_cfg.lines_flag {
                stat.lines += 1;
            }
            if default_flag(&cli_simp_cfg) || cli_simp_cfg.characters_flag {
                stat.characters += line.chars().count();
            }
        }

        // Print results according to configuration
        if cli_simp_cfg.longest_line_flag {
            stat.print_stat_long_rst(
                source_name,
                longest_line_num,
                match_field_len,
            );
        } else {
            stat.print_stat_rst(source_name);
        }

        Ok(stat)
    }

    /// Read file line by line
    ///
    /// filename,   file name, file path.
    /// cli_simp_cfg,   basic command line configuration, simple.
    /// Returns Stat of Result.
    pub(crate) fn read_file(
        filename: &str,
        cli_simp_cfg: CliSimpCfg,
    ) -> Result<Stat, io::Error> {
        // File
        Self::process_lines(
            || {
                let path = Path::new(filename);
                let file = File::open(path)?;
                Ok(Box::new(BufReader::new(file)) as Box<dyn BufRead>)
                    .map(|reader| reader.lines())
            },
            cli_simp_cfg,
            filename,
        )
    }

    /// Read from standard input content text.
    ///
    /// cli_simp_cfg,   basic command line configuration, simple.
    pub(crate) fn read_std_in_contents(cli_simp_cfg: CliSimpCfg) {
        // Standard input
        Self::process_lines(
            || {
                Ok(Box::new(io::BufReader::new(io::stdin()))
                    as Box<dyn BufRead>)
                .map(|reader| reader.lines())
            },
            cli_simp_cfg,
            "[std-in]",
        )
        .unwrap();
    }
}

impl fmt::Display for Stat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        // Selectively build an output string based on whether the field value is 0
        if self.words != 0 {
            output.push_str(&format!("{}\t", self.words));
        }
        if self.lines != 0 {
            output.push_str(&format!("{}\t", self.lines));
        }
        if self.characters != 0 {
            output.push_str(&format!("{}\t", self.characters));
        }
        if self.bytes != 0 {
            output.push_str(&format!("{}\t", self.bytes));
        }
        // Print the format string
        write!(f, "{}", output.trim_end())
    }
}

impl Stat {
    /// Print stat data of long
    fn print_stat_long_rst<'a>(
        &'a self,
        filename: &'a str,
        longest_line_num: usize,
        field_len: usize,
    ) {
        println!("{}\t{}\t{}@{}", self, filename, field_len, longest_line_num);
    }

    /// Print stat data
    fn print_stat_rst<'a>(&'a self, filename: &'a str) {
        println!("{}\t{}", self, filename);
    }

    /// Print total data. Combine all statistical results.
    ///
    /// Combine all statistical results.
    /// results,    total statistics vector.
    pub(crate) fn print_total(results: Vec<Stat>) {
        let total = "total";
        // Combine all statistical results
        let total_stat =
            results.into_iter().fold(Stat::default(), |mut acc, stat| {
                acc.words += stat.words;
                acc.lines += stat.lines;
                acc.characters += stat.characters;
                acc.bytes += stat.bytes;
                acc
            });
        println!("{}\t{}", total_stat, total);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn read_no_file_should_panic() {
        let filepath = "";
        let cli_simp_cfg = CliSimpCfg::default();
        let _ = StatData::read_file(filepath, cli_simp_cfg).unwrap();
    }

    #[test]
    fn read_file_should_pass() {
        let filepath = "tests/data/test.txt";
        // Default: CliSimpCfg { characters_flag: false, lines_flag: false, words_flag: false, longest_line_flag: false, bytes_flag: false }
        let cli_simp_cfg = CliSimpCfg::default();
        let _ = StatData::read_file(filepath, cli_simp_cfg).unwrap();
        assert_eq!(1 + 1, 2);
    }

    #[test]
    fn read_file_check_data() {
        let filepath = "tests/data/test.txt";
        let cli_simp_cfg = CliSimpCfg::default();
        let stat = StatData::read_file(filepath, cli_simp_cfg).unwrap();
        // words, lines, characters, bytes
        assert_eq!(stat.words, 9);
        assert_eq!(stat.lines, 5);
        assert_eq!(stat.characters, 38);
        assert_eq!(stat.bytes, 42);
    }
}
