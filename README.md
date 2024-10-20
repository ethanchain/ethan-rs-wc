[![Tests](https://github.com/ethancws/ethan-rs-wc/actions/workflows/tests.yml/badge.svg)](https://github.com/ethancws/ethan-rs-wc/actions/workflows/tests.yml)
[![Crates.io](https://img.shields.io/crates/v/ethan-rs-wc.svg)](https://crates.io/crates/ethan-rs-wc)

## ethan-rs-wc (erwc)

ethan-rs-ws(erwc) is word, line, character, and byte count.
Like wc command but not just wc command, more accurate and faster.
Text can also be read from standard input for statistics.

[中文](README.zh.md)

## Getting Started

### Manual Build

#### Requirements

- [git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
  - You'll know you did it right if you can run `git --version` and see a response like `git version x.x.x`
- [rust](https://www.rust-lang.org/tools/install)
  - Install Rust according to the official documentation, when you open the shell and run `rustc --version` and see a response
    like `rustc x.y.z (abcabcabc yyyy-mm-dd)`. And on the shell you run `cargo --version` and see a response like `cargo x.y.z (abcabc yyyy-mm-dd)`. This means you successfully installed the Rust compilation environment.

```bash
$ git clone https://github.com/ethanchain/ethan-rs-wc.git
$ cd ethan-rs-wc
$ cargo build --release
```

The path of command is target/release/erwc, you could copy/move it to other place or just `cargo run --release -- <args>`.

> When you run the binary command file erwc directly, you can specify the relative path or absolute path of erwc, and then add the corresponding parameters. Or configure PATH for erwc.

### Release

You could download the binary published in [Release](https://github.com/ethancws/ethan-rs-wc/releases/).

## How to use

```bash
$ erwc tests/data/test.txt
```

You'll see like `9       5       38      42      tests/data/test.txt`. In order, they are number of words, lines, characters, bytes and file path.

```bash
$ erwc -l tests/data/test.txt
```

You'll see like `5       tests/data/test.txt`. The are number of lines and file path. This is the same as running the `erwc --lines tests/data/test.txt` command.

```bash
$ erwc -lwcbL tests/data/test.txt
```

You'll see like `9       5       38      42      tests/data/test.txt     30@5`. In order, they are number of words, lines, characters, bytes and file path, the number of bytes with the most bytes, followed by the @ sign, then the line number with the most bytes.

> You can run `erwc -h` or `erwc --help` get more information.

## Quick examples comparison

These examples compare statistics for a given file, which varies in size from the smallest files to dozens of bytes to the largest 1.2G bytes. And statistics on multiple files at the same time. Timings were collected on a system with Intel(R) Core(TM) i7-8559U CPU @ 2.70GHz 4 core 16G.

In order to make the benchmark test as accurate and complete as possible, in addition to comparing the erwc command and the wc command, other commands are used to count the number of file lines and bytes, and compare the data statistics capabilities of the erwc and wc commands from the side.

In order to construct a large file of 1G bytes I used two commands to create `tests/data/splitfile.txt` and `tests/data/largefile.txt` file.

```bash
$ echo `base64 -i /dev/urandom | head -c 1000000000`｜ fold -w $((RANDOM % 50 + 50))  > tests/data/splitfile.txt

$ awk '{
  output = "";
  while (length($0) > 0) {
    len = int(rand() * 7) + 1;
    part = substr($0, 1, len);
    output = output (output ? " " : "") part;
    $0 = substr($0, len + 1);
  }
  print output;
}' tests/data/splitfile.txt > tests/data/largefile.txt
```

### Single file statistics

| Command                         | File                     | Time                                         | Size       |
| ------------------------------- | ------------------------ | -------------------------------------------- | ---------- |
| `erwc tests/data/test.txt`      | tests/data/test.txt      | 0.00s user 0.00s system 93% cpu 0.006 total  | 46         |
| `wc tests/data/test.txt`        | tests/data/test.txt      | 0.00s user 0.00s system 76% cpu 0.004 total  | 46         |
| `erwc tests/data/sherlock.txt`  | tests/data/sherlock.txt  | 0.00s user 0.00s system 106% cpu 0.007 total | 90314      |
| `wc tests/data/sherlock.txt`    | tests/data/sherlock.txt  | 0.00s user 0.00s system 88% cpu 0.006 total  | 90314      |
| `erwc /var/logs/keybagd.log.1`  | /var/logs/keybagd.log.1  | 0.01s user 0.00s system 76% cpu 0.016 total  | 1049061    |
| `wc /var/logs/keybagd.log.1`    | /var/logs/keybagd.log.1  | 0.01s user 0.00s system 89% cpu 0.012 total  | 1049061    |
| `erwc /var/log/install.log`     | /var/log/install.log     | 0.18s user 0.02s system 97% cpu 0.212 total  | 48244124   |
| `wc /var/log/install.log`       | /var/log/install.log     | 0.20s user 0.01s system 98% cpu 0.216 total  | 48244124   |
| `erwc tests/data/splitfile.txt` | tests/data/splitfile.txt | 3.60s user 0.91s system 86% cpu 5.210 total  | 1000000015 |
| `wc tests/data/splitfile.txt`   | tests/data/splitfile.txt | 5.86s user 0.18s system 99% cpu 6.060 total  | 1000000015 |
| `erwc tests/data/largefile.txt` | tests/data/largefile.txt | 5.53s user 0.35s system 99% cpu 5.901 total  | 1244571298 |
| `wc tests/data/largefile.txt`   | tests/data/largefile.txt | 5.67s user 0.19s system 99% cpu 5.872 total  | 1244571298 |

### Multi-file statistics

| Command                                                                                              | Time                                         |
| ---------------------------------------------------------------------------------------------------- | -------------------------------------------- |
| `erwc tests/data/largefile.txt tests/data/splitfile.txt tests/data/sherlock.txt tests/data/test.txt` | 9.04s user 0.60s system 156% cpu 6.166 total |
| `wc tests/data/largefile.txt tests/data/splitfile.txt tests/data/sherlock.txt tests/data/test.txt`   | 9.34s user 0.33s system 99% cpu 9.694 total  |

### Count file lines

#### largefile.txt

| Tool | Command                                                        | Number   |
| ---- | -------------------------------------------------------------- | -------- |
| awk  | `awk 'END {print NR}' tests/data/largefile.txt`                | 10869566 |
| sed  | `sed -n '$=' tests/data/largefile.txt`                         | 10869566 |
| grep | `grep -c '' tests/data/largefile.txt`                          | 10869566 |
| cat  | `cat -n tests/data/largefile.txt                 \| tail -n 1` | 10869566 |

#### splitfile.txt

| Tool | Command                                                        | Number   |
| ---- | -------------------------------------------------------------- | -------- |
| awk  | `awk 'END {print NR}' tests/data/splitfile.txt`                | 10869566 |
| sed  | `sed -n '$=' tests/data/splitfile.txt`                         | 10869566 |
| grep | `grep -c '' tests/data/splitfile.txt`                          | 10869566 |
| cat  | `cat -n tests/data/splitfile.txt                 \| tail -n 1` | 10869566 |

#### sherlock.txt

| Tool | Command                                                       | Number |
| ---- | ------------------------------------------------------------- | ------ |
| awk  | `awk 'END {print NR}' tests/data/sherlock.txt`                | 2133   |
| sed  | `sed -n '$=' tests/data/sherlock.txt`                         | 2133   |
| grep | `grep -c '' tests/data/sherlock.txt`                          | 2133   |
| cat  | `cat -n tests/data/sherlock.txt                 \| tail -n 1` | 2133   |

#### test.txt

| Tool | Command                                                   | Number |
| ---- | --------------------------------------------------------- | ------ |
| awk  | `awk 'END {print NR}' tests/data/test.txt`                | 5      |
| sed  | `sed -n '$=' tests/data/test.txt`                         | 5      |
| grep | `grep -c '' tests/data/test.txt`                          | 5      |
| cat  | `cat -n tests/data/test.txt                 \| tail -n 1` | 5      |

### Statistics of the total number of bytes in the file

#### largefile.txt

| Tool | Command                               | Size       |
| ---- | ------------------------------------- | ---------- |
| stat | `stat -f %z tests/data/largefile.txt` | 1255440864 |
| du   | `du -k tests/data/largefile.txt`      | 1229796k   |
| ls   | `ls -l tests/data/largefile.txt`      | 1255440864 |

#### splitfile.txt

| Tool | Command                               | Size       |
| ---- | ------------------------------------- | ---------- |
| stat | `stat -f %z tests/data/splitfile.txt` | 1010869565 |
| du   | `du -k tests/data/splitfile.txt`      | 999712k    |
| ls   | `ls -l tests/data/splitfile.txt`      | 1010869565 |

#### sherlock.txt

| Tool | Command                              | Size  |
| ---- | ------------------------------------ | ----- |
| stat | `stat -f %z tests/data/sherlock.txt` | 90314 |
| du   | `du -k tests/data/sherlock.txt`      | 92k   |
| ls   | `ls -l tests/data/sherlock.txt`      | 90314 |

#### test.txt

| Tool | Command                          | Size |
| ---- | -------------------------------- | ---- |
| stat | `stat -f %z tests/data/test.txt` | 46   |
| du   | `du -k tests/data/test.txt`      | 4?   |
| ls   | `ls -l tests/data/test.txt`      | 46   |

## Running tests

```bash
$ cargo test
```

## Thank You!

If you appreciated this, feel free to follow me or donate!

Solana Address: 3gArMnKUHkZ1eEry4dD8zdMpJH385HKrUdnG9ig6S5Zy

## Resources

- [Rust](https://www.rust-lang.org/)
- [Crate](https://crates.io/)
