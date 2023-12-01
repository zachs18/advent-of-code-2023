use std::io::Write;

use regex::Regex;

const YEAR: u32 = 2023;

const DEFAULT_TEMPLATE: &str = r###"
#![allow(unused_imports)]
use aoc_2023::*;
use aoc_driver::*;
use itertools::Itertools;

fn part_1(#[allow(unused)] input: &str) -> u64 {
    todo!()
}

fn part_2(#[allow(unused)] input: &str) -> u64 {
    todo!()
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    if let Err(error) = aoc_magic!(session.trim(), %%YEAR%%:%%DAY%%:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    if let Err(error) = aoc_magic!(session.trim(), %%YEAR%%:%%DAY%%:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}
"###;

const PREPARSED_TEMPLATE: &str = r###"
#![allow(unused_imports)]
use aoc_2023::*;
use aoc_driver::*;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
pub struct Thing {
}

fn parse(input: &str) -> Vec<Thing> {
    input
        .lines()
        .flat_map(|line| match line.trim() {
            _ => todo!(),
        })
        .collect_vec()
}

fn part_1(#[allow(unused)] input: &Vec<Thing>) -> i64 {
    todo!()
}
fn part_2(#[allow(unused)] input: &Vec<Thing>) -> String {
    todo!()
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    let mut both = PreParsed::new(parse, part_1, part_2);
    let part_2 = both.part_2();
    if let Err(error) = aoc_magic!(session, %%YEAR%%:%%DAY%%:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    let part_1 = both.part_1();
    if let Err(error) = aoc_magic!(session, %%YEAR%%:%%DAY%%:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}

#[test]
fn example() {
    let input = "example input";
    let mut both = PreParsed::new(parse, part_1, part_2);
    let part_1 = both.part_1();
    assert_eq!(part_1(input), &42);
    (both.part_2())(&input);
}

"###;

fn usage() -> ! {
    eprintln!("Usage: \n\tcargo newday\n\tcargo newday 6\n\tcargo newday --preparsed\n\tcargo newday 6 --preparsed\n\tcargo newday --preparsed 6\n\t");
    std::process::exit(-1)
}

fn find_highest_day() -> i32 {
    let dir = std::fs::read_dir("./src").unwrap();
    let mut highest_day = 0;
    let day_regex = Regex::new("^day([0-9])+$").unwrap();
    for a in dir.map(Result::unwrap) {
        if !a.file_type().unwrap().is_dir() {
            continue;
        }
        let filename = a.file_name();
        let Some(filename) = filename.to_str() else {
            continue;
        };
        let Some(captures) = day_regex.captures(filename) else {
            continue;
        };
        let Some(day_num) = captures.get(1) else {
            continue;
        };
        let day_num = day_num.as_str().parse().unwrap();
        highest_day = highest_day.max(day_num);
    }
    highest_day
}

fn main() {
    let mut args = std::env::args().skip(1);
    let arg1 = args.next();
    let arg2 = args.next();
    let arg3 = args.next();
    let (new_day, use_preparsed) = match (arg1.as_deref(), arg2.as_deref(), arg3.as_deref()) {
        (None, _, _) => (find_highest_day() + 1, false),
        (Some("--preparsed"), None, _) => (find_highest_day() + 1, true),
        (Some(day), None, _) => (day.parse().unwrap(), true),
        (Some("--preparsed"), Some(day), None) | (Some(day), Some("--preparsed"), None) => {
            (day.parse().unwrap_or_else(|_| usage()), true)
        }
        _ => usage(),
    };

    std::fs::DirBuilder::new()
        .recursive(true)
        .create(&format!("./src/bin/day{new_day}"))
        .expect("failed to create new day folder");
    let real_main = std::fs::File::options()
        .write(true)
        .create_new(true)
        .open(&format!("./src/bin/day{new_day}/main.rs"));
    let default_main = std::fs::File::options()
        .write(true)
        .create_new(true)
        .open(&format!("./src/bin/day{new_day}/default_main.rs"));
    let preparsed_main = std::fs::File::options()
        .write(true)
        .create_new(true)
        .open(&format!("./src/bin/day{new_day}/preparsed_main.rs"));

    let default_formatted = DEFAULT_TEMPLATE
        .replace("%%YEAR%%", &format!("{}", YEAR))
        .replace("%%DAY%%", &format!("{}", new_day));
    let preparsed_formatted = PREPARSED_TEMPLATE
        .replace("%%YEAR%%", &format!("{}", YEAR))
        .replace("%%DAY%%", &format!("{}", new_day));

    match real_main {
        Ok(mut real_main) => {
            let data = if use_preparsed {
                preparsed_formatted.as_bytes()
            } else {
                default_formatted.as_bytes()
            };
            real_main
                .write_all(data)
                .expect("Failed to write to main.rs");
        }
        Err(_) => {
            eprintln!("Warning: main.rs already exists or failed to be opened. Not modifying it.")
        }
    }

    match default_main {
        Ok(mut default_main) => {
            default_main
                .write_all(default_formatted.as_bytes())
                .expect("Failed to write to default_main.rs");
        }
        Err(_) => {
            eprintln!(
                "Warning: default_main.rs already exists or failed to be opened. Not modifying it."
            )
        }
    }

    match preparsed_main {
        Ok(mut preparsed_main) => {
            preparsed_main
                .write_all(preparsed_formatted.as_bytes())
                .expect("Failed to write to preparsed_main.rs");
        }
        Err(_) => {
            eprintln!("Warning: preparsed_main.rs already exists or failed to be opened. Not modifying it.")
        }
    }
}
