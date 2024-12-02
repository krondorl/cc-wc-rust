# cc-wc-rust

Rust version of wc (word counter) program.

## Task Description

This is the Rust version of [Coding Challenges](https://codingchallenges.fyi/challenges/intro/): [Build Your Own wc Tool](https://codingchallenges.fyi/challenges/challenge-wc/)

## Dependencies

Only the [clap crate](https://crates.io/crates/clap).

## Installation of Programming Language Tools

First, the [Rust tools must be installed.](https://www.rust-lang.org/tools/install)

## Usage and Output

Inside repo folder run this in a terminal:
`cargo run -- .\test.txt`

```
> cargo run -- .\test.txt
   Compiling ccwc v0.1.0 (F:\Dev\cc-wc-rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.72s
     Running `target\debug\ccwc.exe .\test.txt`
Lines: 7145
Words: 58164
Bytes: 327900
Characters: 325002
```

You can run help as well:
`cargo run -- --help`

```
> cargo run -- --help
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target\debug\ccwc.exe --help`
ccwc 0.1.0
Counts words and various attributes of files

USAGE:
    ccwc.exe [OPTIONS] <file>

ARGS:
    <file>    File to process

OPTIONS:
    -c, --bytes      Count bytes
    -h, --help       Print help information
    -l, --lines      Count lines
    -m, --chars      Count characters
    -V, --version    Print version information
    -w, --words      Count words
```

## Known Bugs

The `bytes` and `characters` are not matching the spec yet.

## License

Read the [LICENSE file](LICENSE).

## History

I started the project on 2nd of December, 2024.