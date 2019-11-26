# zulu
Replaces epoch times in text with human-readable UTC strings. If you have a file or other stream of text data that contains epoch times:
```json
{"text": "I am some JSON", "ts": 1574736728 }
```
You can pipe the stdout from a process like `cat`, `kafakcat`, `jq`, etc. to `zulu` to reformat these timestamps with minimal effort:
```json
{"text": "I am some JSON", "ts": "2019-11-26T02:52:08Z" }
```

This program makes some assumptions about the range of years people would be interested in to limit false matches by the regex, effectively restricting to the years 1973-2128.

## Installation
TODO

## Usage
Use `zulu` on the command line as part of a series of pipes, ie:
```
cat my_file | grep 'something to limit' | zulu
```
Optional parameters:
- `-s` Stringify output by wrapping with double quotes, useful for working with JSON data
- `-l` Use local timezone for output
- `-f <format string>` Specify output format, see [Chrono's strftime module](https://docs.rs/chrono/0.4.0/chrono/format/strftime/index.html#specifiers) for more details

## TODO
- Benchmarks
- Field aware, allow operations only on specific columns of data as output by something like `awk`
- `brew` installation support
