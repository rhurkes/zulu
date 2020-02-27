extern crate chrono;

use chrono::{Local, SecondsFormat, TimeZone, Utc};
use regex::Regex;
use std::io::{self, Read, Write};
use structopt::StructOpt;

// For certain assumptions of seconds/millis/micros, leading digits of 1-4 will parse as being
// before the year ~2128. This bound is essentially what restricts the year range supported.
const INTERMEDIATE_TICKS_BOUND: char = '5';

const SECONDS_DIVISOR: i64 = 1;
const MILLIS_DIVISOR: i64 = 1_000;
const MICROS_DIVISOR: i64 = 1_000_000;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Format string
    #[structopt(short = "f", long)]
    format: Option<String>,

    /// Display timestamp in local time
    #[structopt(short = "l", long)]
    local: bool,

    /// Stringify the value, by wrapping with quotes
    #[structopt(short = "s", long)]
    stringify: bool,
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    let stdin = io::stdin();
    let stdout = io::stdout();
    let re = Regex::new(r"\d{9,16}").expect("Invalid regex");
    let mut buffer = String::new();
    let mut in_handle = stdin.lock();
    let mut out_handle = stdout.lock();

    in_handle.read_to_string(&mut buffer)?;
    let mut output = buffer.to_string();

    for caps in re.captures_iter(&buffer) {
        let matching = caps.get(0).expect("Invalid regex capture").as_str();
        let length = matching.len();
        let ticks = matching
            .parse::<i64>()
            .expect("Regex must only capture valid numbers");
        let first_char = matching.chars().nth(0).expect("Regex must capture numbers");
        let dt = get_dt(ticks, length, first_char, &opt);

        if let Some(dt) = dt {
            output = output.replacen(matching, &dt, 1)
        }
    }

    match out_handle.write_all(output.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", e);
            Ok(())
        }
    }
}

fn get_dt(ticks: i64, length: usize, first_char: char, opt: &Opt) -> Option<String> {
    match length {
        9 | 10 => {
            if length == 10 && first_char >= INTERMEDIATE_TICKS_BOUND {
                None
            } else {
                Some(parse_ticks(ticks, SECONDS_DIVISOR, &opt))
            }
        }
        12 | 13 => {
            if length == 13 && first_char >= INTERMEDIATE_TICKS_BOUND {
                None
            } else {
                Some(parse_ticks(ticks, MILLIS_DIVISOR, &opt))
            }
        }
        15 | 16 => {
            if length == 16 && first_char >= INTERMEDIATE_TICKS_BOUND {
                None
            } else {
                Some(parse_ticks(ticks, MICROS_DIVISOR, &opt))
            }
        }
        _ => None,
    }
}

fn parse_ticks(ticks: i64, divisor: i64, opt: &Opt) -> String {
    let seconds = ticks / divisor;
    let remainder = (ticks % divisor) as u32;
    let dt = Utc.timestamp(seconds, remainder);

    let output = if opt.local {
        let local_dt = Local.from_utc_datetime(&dt.naive_utc());
        match &opt.format {
            Some(format) => local_dt.format(format).to_string(),
            _ => local_dt.to_rfc3339_opts(SecondsFormat::AutoSi, true),
        }
    } else {
        match &opt.format {
            Some(format) => dt.format(&format).to_string(),
            _ => dt.to_rfc3339_opts(SecondsFormat::AutoSi, true),
        }
    };

    if opt.stringify {
        format!("\"{}\"", output)
    } else {
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_dt_should_stringify_replacements_if_asked_to() {
        let opt = Opt {
            format: None,
            local: false,
            stringify: true,
        };

        let result = get_dt(1574736728, 10, '1', &opt);
        assert_eq!(result, Some("\"2019-11-26T02:52:08Z\"".to_string()));
    }

    #[test]
    fn get_dt_should_handle_various_scenarios() {
        struct Test<'a> {
            input: &'a str,
            expected: Option<String>,
        }

        let opt = Opt {
            format: None,
            local: false,
            stringify: false,
        };

        let tests = vec![
            Test {
                input: "11111111",
                expected: None,
            },
            Test {
                input: "999999999",
                expected: Some("2001-09-09T01:46:39Z".to_string()),
            },
            Test {
                input: "4999999999",
                expected: Some("2128-06-11T08:53:19Z".to_string()),
            },
            Test {
                input: "5000000000",
                expected: None,
            },
            Test {
                input: "999999999999",
                expected: Some("2001-09-09T01:46:39.000000999Z".to_string()),
            },
            Test {
                input: "4999999999999",
                expected: Some("2128-06-11T08:53:19.000000999Z".to_string()),
            },
            Test {
                input: "5000000000000",
                expected: None,
            },
            Test {
                input: "999999999999999",
                expected: Some("2001-09-09T01:46:39.000999999Z".to_string()),
            },
            Test {
                input: "4999999999999999",
                expected: Some("2128-06-11T08:53:19.000999999Z".to_string()),
            },
            Test {
                input: "5000000000000000",
                expected: None,
            },
            Test {
                input: "11111111111111111",
                expected: None,
            },
        ];

        for test in tests {
            let ticks = test.input.to_string().parse::<i64>().unwrap();
            let length = test.input.len();
            let first_char = test.input.chars().nth(0).unwrap();
            let result = get_dt(ticks, length, first_char, &opt);
            assert_eq!(result, test.expected);
        }
    }

    #[test]
    fn parse_ticks_no_format_should_use_default_format() {
        let opt = Opt {
            format: None,
            local: false,
            stringify: false,
        };

        let result = parse_ticks(100_000_000, 1, &opt);
        assert_eq!(result, "1973-03-03T09:46:40Z".to_string());
    }

    #[test]
    fn parse_ticks_local_should_not_return_z_time() {
        let opt = Opt {
            format: None,
            local: true,
            stringify: false,
        };

        // This is brittle, since we can't easily mock system time
        let re = Regex::new(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}-\d{2}:\d{2}").unwrap();
        let result = parse_ticks(100_000_000, 1, &opt);
        assert!(re.is_match(&result));
    }

    #[test]
    fn parse_ticks_should_use_supplied_format() {
        let opt = Opt {
            format: Some("%Y %m".to_string()),
            local: false,
            stringify: false,
        };

        let result = parse_ticks(100_000_000, 1, &opt);
        assert_eq!(result, "1973 03".to_string());
    }

    #[test]
    fn parse_ticks_should_handle_various_scenarios() {
        struct Test {
            ticks: i64,
            divisor: i64,
            expected: String,
        }

        let opt = Opt {
            format: None,
            local: false,
            stringify: false,
        };

        let tests = vec![
            Test {
                ticks: 100_000_000,
                divisor: 1,
                expected: "1973-03-03T09:46:40Z".to_string(),
            },
            Test {
                ticks: 400_000_000,
                divisor: 1,
                expected: "1982-09-04T15:06:40Z".to_string(),
            },
            Test {
                ticks: 100_000_000_000,
                divisor: 1_000,
                expected: "1973-03-03T09:46:40Z".to_string(),
            },
            Test {
                ticks: 4_000_000_000_000,
                divisor: 1_000,
                expected: "2096-10-02T07:06:40Z".to_string(),
            },
        ];

        for test in tests {
            let result = parse_ticks(test.ticks, test.divisor, &opt);
            assert_eq!(result, test.expected);
        }
    }
}
