# zulu
Replaces epoch times in text with human-readable strings, defaults to UTC/Zulu time. If you have a file or other stream of text data that contains epoch times:
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

This program makes some assumptions about the range of years people would be interested in to limit false matches by the regex, effectively restricting to the years 1973-2128. For example `5574739532` would convert to `Sunday, August 28, 2146 10:45:32 AM`. It starts with a `5` which means it would be later than `2128` and **probably** not an intentional timestamp.

## Installation
TODO

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
- Write to stderr for errors
- Benchmarks
- Field aware, allow operations only on specific columns of data as output by something like `awk`
- `brew` installation support
