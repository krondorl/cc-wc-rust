# cc-wc-rust

Rust version of wc (word counter) program.

## Task Description

This is the Rust version of [Coding Challenges](https://codingchallenges.fyi/challenges/intro/): [Build Your Own wc Tool](https://codingchallenges.fyi/challenges/challenge-wc/)

## Dependencies

I use only the [clap crate](https://crates.io/crates/clap).

## Installation of the Rust Language

[Rust must be installed.](https://www.rust-lang.org/tools/install)

## Usage and Output

Inside repo folder run this in a terminal:
`cargo run -- .\test.txt`

```
> cargo run -- .\test.txt
   Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
   Running `target\debug\ccwc.exe .\test.txt`
7145 58164 342190 339292 .\test.txt
```

You can run help as well:
`cargo run -- --help`

```
> cargo run -- --help
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target\debug\ccwc.exe --help`
ccwc 0.2.0
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

## License

Read the [LICENSE file](LICENSE).

## History

### 0.2.0 (6th December, 2024)

I refactored the algorithm so the `bytes` and `characters` are matching the spec.

### 0.1.0 (2nd of December, 2024)

I started the project.