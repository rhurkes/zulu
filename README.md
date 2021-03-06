# zulu
Reads stdin and writes to stdout while replacing epoch times in text with human-readable strings, defaults to UTC/Zulu time. If you have a file or other stream of text data that contains epoch times:
```json
{ "text": "I am some JSON", "ts": 1574736728 }
```
You can pipe the stdout from a process like `cat`, `kafakcat`, `jq`, etc. to `zulu` to reformat these timestamps with minimal effort:
```json
{ "text": "I am some JSON", "ts": "2019-11-26T02:52:08Z" }
```

What values are converted?
- 9-10 digit values are treated as **seconds** since epoch
- 12-13 digit values are treated as **milliseconds** since epoch
- 15-16 digit values are treated as **microseconds** since epoch

This program makes some assumptions about the range of years to limit false matches by the regex, effectively restricting replacements to the years **1973-2128**. For example `5574739532` would convert to `Sunday, August 28, 2146 10:45:32 AM`. It starts with a `5` which means it would be later than `2128` and probably not an intentional timestamp - so it is **not** converted. It could be considered "greedy" in what it tries to convert, with these restrictions in place. UPCs are a good example of a numeric string that can be unintentionally converted - **it is handy for triaging things and poking around, but not recommended to use zulu for data conversion because of this.**

## Installation
- You can download a binary from the *Releases* section for supported platforms
- If you'd like to build from source:
    - have Rust 2018 installed (developed with 1.38, although slightly older versions should work) 
    - clone this repository on your local filesystem
    - run `cargo build --release` and copy the `target/release/zulu` binary to somewhere on your path, ie. `/usr/local/bin/`

## Usage
Use `zulu` on the command line as part of a series of pipes, ie:
```
cat my_file | grep 'something to limit' | zulu > new_file
```
Optional parameters:
- `-f <format string>` Specify output format, see [Chrono's strftime module](https://docs.rs/chrono/0.4.0/chrono/format/strftime/index.html#specifiers) for more details
- `-l` Use local timezone for output
- `-s` Stringify output by wrapping with double quotes, useful for working with JSON data

## TODO
- Benchmarks
- Field aware, allow operations only on specific columns of data as output by something like `awk`
- `brew` installation support
