[![Tests](https://github.com/ethancws/ethan-rs-wc/actions/workflows/tests.yml/badge.svg)](https://github.com/ethancws/ethan-rs-wc/actions/workflows/tests.yml)
[![Crates.io](https://img.shields.io/crates/v/ethan-rs-wc.svg)](https://crates.io/crates/ethan-rs-wc)

## ethan-rs-wc (erwc)

ethan-rs-ws(erwc)命令能够统计文件与标准输入的单词数，行数，字符数和字节数。
erwc 命令像 wc 命令但是不仅仅是 wc 命令，相比它统计数据更精确和快速。

[English](README.md)

## 开始

### 手动编译

#### 必要准备

- [git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
  - 当你运行 `git --version` 然后看见命令行输出 `git version x.x.x` 这说明你已正确配置。
- [rust](https://www.rust-lang.org/tools/install)
  - 请按照 Rust 的官方文档配置 Rust 的运行环境，当你打开终端并运行 `rustc --version` 看见像这样的输出 `rustc x.y.z (abcabcabc yyyy-mm-dd)` ，以及在终端运行 `cargo --version` 看见像这样的输出 `cargo x.y.z (abcabc yyyy-mm-dd)` ，这些信息说明你已经把 Rust 的运行环境配置好了。

```bash
$ git clone https://github.com/ethancws/ethan-rs-wc.git
$ cd ethan-rs-wc
$ cargo build --release
```

编译后的可执行二进制文件的路径是 `target/release/erwc`，你可以拷贝或者移动到相应目录中，或者直接运行命令 `cargo run --release -- <args>`。

> 当你直接运行可执行二进制文件，你可以选择按照相对路径或者绝对路径运行 erwc 然后再跟相应参数。也可以为 erwc 配置系统环境 PATH。

### 发行包

你可以下载项目发布的可执行[发行包](https://github.com/ethancws/ethan-rs-wc/releases/)直接运行命令。

## 怎么使用呢

```bash
$ erwc tests/data/test.txt
```

执行以上命令将看到类似输出 `9       5       38      42      tests/data/test.txt`。按照顺序依次是单词数，行数，字符数，字节数以及文件路径。

```bash
$ erwc -l tests/data/test.txt
```

执行以上命令将看到类似输出 `5       tests/data/test.txt`。5 代表行数，`tests/data/test.txt`代表文件路径。这个命令和 `erwc --lines tests/data/test.txt` 命令等同。

```bash
$ erwc -lwcbL tests/data/test.txt
```

执行以上命令将看到类似输出 `9       5       38      42      tests/data/test.txt     30@5`。按照顺序依次是单词数，行数，字符数，字节数，文件路径，最多字节那一行的字节数@对应的行号

> 你可以运行 `erwc -h` 或 `erwc --help` 命令获取帮助及更多的相关信息。

## 快速示例比较

这些统计示例使用的文件从几十字节到 1.2G 字节。同时也统计了一次性读取多个文件的情况。测试机的配置为 Intel(R) Core(TM) i7-8559U CPU @ 2.70GHz 4 核 16G。

为了尽可能的让这些基准测试准确与完备，特别比较了在多种情况下 erwc 和 wc 命令间的执行效率与准确率，特别额外使用了其它命令来从侧面验证统计这些文件的准确性。

为了验证在大文件下的运行情况，使用了以下命令创建了两个超过 1G 字节的文件：

```bash
$ echo `base64 -i /dev/urandom | head -c 1000000000`｜ fold -w $((RANDOM % 50 + 50))  > tests/data/splitfile.txt # tests/data/splitfile.txt

$ awk '{
  output = "";
  while (length($0) > 0) {
    len = int(rand() * 7) + 1;
    part = substr($0, 1, len);
    output = output (output ? " " : "") part;
    $0 = substr($0, len + 1);
  }
  print output;
}' tests/data/splitfile.txt > tests/data/largefile.txt # tests/data/largefile.txt
```

### 单个文件的数据统计

| 命令                            | 文件                     | 耗时                                         | 字节大小   |
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

### 多个文件的数据统计

| 命令                                                                                                 | 耗时                                         |
| ---------------------------------------------------------------------------------------------------- | -------------------------------------------- |
| `erwc tests/data/largefile.txt tests/data/splitfile.txt tests/data/sherlock.txt tests/data/test.txt` | 9.04s user 0.60s system 156% cpu 6.166 total |
| `wc tests/data/largefile.txt tests/data/splitfile.txt tests/data/sherlock.txt tests/data/test.txt`   | 9.34s user 0.33s system 99% cpu 9.694 total  |

### 其它工具统计文件行数

#### largefile.txt

| 工具 | 命令                                                           | 行数     |
| ---- | -------------------------------------------------------------- | -------- |
| awk  | `awk 'END {print NR}' tests/data/largefile.txt`                | 10869566 |
| sed  | `sed -n '$=' tests/data/largefile.txt`                         | 10869566 |
| grep | `grep -c '' tests/data/largefile.txt`                          | 10869566 |
| cat  | `cat -n tests/data/largefile.txt                 \| tail -n 1` | 10869566 |

#### splitfile.txt

| 工具 | 命令                                                           | 行数     |
| ---- | -------------------------------------------------------------- | -------- |
| awk  | `awk 'END {print NR}' tests/data/splitfile.txt`                | 10869566 |
| sed  | `sed -n '$=' tests/data/splitfile.txt`                         | 10869566 |
| grep | `grep -c '' tests/data/splitfile.txt`                          | 10869566 |
| cat  | `cat -n tests/data/splitfile.txt                 \| tail -n 1` | 10869566 |

#### sherlock.txt

| 工具 | 命令                                                          | 行数 |
| ---- | ------------------------------------------------------------- | ---- |
| awk  | `awk 'END {print NR}' tests/data/sherlock.txt`                | 2133 |
| sed  | `sed -n '$=' tests/data/sherlock.txt`                         | 2133 |
| grep | `grep -c '' tests/data/sherlock.txt`                          | 2133 |
| cat  | `cat -n tests/data/sherlock.txt                 \| tail -n 1` | 2133 |

#### test.txt

| 工具 | 命令                                                      | 行数 |
| ---- | --------------------------------------------------------- | ---- |
| awk  | `awk 'END {print NR}' tests/data/test.txt`                | 5    |
| sed  | `sed -n '$=' tests/data/test.txt`                         | 5    |
| grep | `grep -c '' tests/data/test.txt`                          | 5    |
| cat  | `cat -n tests/data/test.txt                 \| tail -n 1` | 5    |

### 其它工具统计文件的总字节数

#### largefile.txt

| 工具 | 命令                                  | 总字节数   |
| ---- | ------------------------------------- | ---------- |
| stat | `stat -f %z tests/data/largefile.txt` | 1255440864 |
| du   | `du -k tests/data/largefile.txt`      | 1229796k   |
| ls   | `ls -l tests/data/largefile.txt`      | 1255440864 |

#### splitfile.txt

| 工具 | 命令                                  | 总字节数   |
| ---- | ------------------------------------- | ---------- |
| stat | `stat -f %z tests/data/splitfile.txt` | 1010869565 |
| du   | `du -k tests/data/splitfile.txt`      | 999712k    |
| ls   | `ls -l tests/data/splitfile.txt`      | 1010869565 |

#### sherlock.txt

| 工具 | 命令                                 | 总字节数 |
| ---- | ------------------------------------ | -------- |
| stat | `stat -f %z tests/data/sherlock.txt` | 90314    |
| du   | `du -k tests/data/sherlock.txt`      | 92k      |
| ls   | `ls -l tests/data/sherlock.txt`      | 90314    |

#### test.txt

| 工具 | 命令                             | 总字节数 |
| ---- | -------------------------------- | -------- |
| stat | `stat -f %z tests/data/test.txt` | 46       |
| du   | `du -k tests/data/test.txt`      | 4?       |
| ls   | `ls -l tests/data/test.txt`      | 46       |

## 运行测试

```bash
$ cargo test
```

## 感谢

如果你在此项目中有所收获，想感激我可以在我的 GitHub 主页上 Follow 或者捐赠。

- Solana 地址： 3gArMnKUHkZ1eEry4dD8zdMpJH385HKrUdnG9ig6S5Zy
- 数字人民币账号：0071130516031100
- 支付宝二维码：
<br/>
<p align="center">
<img src="./alipay-ec.jpg" width="225">
</a>
</p>
<br/>

## 资源

- [Rust](https://www.rust-lang.org/)
- [Crate](https://crates.io/)
