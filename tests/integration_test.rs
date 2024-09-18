use ethan_rs_wc::Mode;
use std::ffi::OsString;

#[test]
fn erwc_test() {
    let args = vec![
        OsString::from("target/debug/erwc"),
        OsString::from("tests/data/sherlock.txt"),
    ];
    Mode::erwc(args);
    assert_eq!(2 + 2, 4);
}
