/*!
The main entry point into ethan-rs-wc(erwc).
*/
#![allow(clippy::arithmetic_side_effects)]

use std::env;

use ethan_rs_wc::Mode;
/// Program entrance
///
/// Passing command line arguments to the crate library.
///
/// Command erwc are some differences from the POSIX standard and GNU implementation.
/// Such as no -c print number of characters, -b number of bytes. Has no -m command parameter.
/// -L will print like '32@5', 32 is number of bytes, 5 is the maximum number of bytes for line number.
/// Also -L will print the file or standard input contents the number of bytes counted.
///
/// # Usage
/// erwc -[wlcbL] [FILEPATH] ...
///
/// # Examples
/// ```bash
/// $ erwc test.txt
/// ```
///             words   lines   characters  bytes   filename/filepath
/// Will print: 9       5       38      42      test.txt
/// By default, it will be printed in sequence the number of words, lines, characters, bytes.
///
/// ```bash
/// $ erwc -L test.txt
/// ```
///             words   lines   characters  bytes   filename/filepath    number of bytes in one line@the maximum number of bytes for line number
/// Will print: 9       5       38      42      test.txt        30@5
///
/// ```bash
/// $ erwc test.txt README.md
/// ```
/// Will print:
///             209     45      1106    1280    README.md
///             9       5       38      42      test.txt
///             218     50      1144    1322    total
///
/// ```bash
/// $ erwc -c test.txt
/// ```
/// Will print: 38      test.txt
/// Just the number of characters and filename.
///
/// ```bash
/// $ erwc -lcb test.txt
/// ```
///             lines   characters  bytes   filename/filepath
/// Will print: 5       38      42      test.txt
fn main() {
    Mode::run(env::args_os());
}
