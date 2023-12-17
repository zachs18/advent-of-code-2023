use aoc_driver::*;
use regex::Regex;

fn part_1(input: &str) -> u64 {
    input
        .lines()
        .map(str::trim)
        .map(|line| {
            let mut bytes = line.bytes().filter(|c| c.is_ascii_digit());
            let tens = bytes.next().unwrap();
            let ones = bytes.next_back().unwrap_or(tens);
            u64::from((tens - b'0') * 10 + ones - b'0')
        })
        .sum()
}

fn digit_value(digit: &str) -> u64 {
    match digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => digit.parse().expect("found invalid digit"),
    }
}

#[allow(unused)]
fn part_2_regex(input: &str) -> u64 {
    let input = input.lines().map(str::trim);
    let digit_regex = Regex::new("one|two|three|four|five|six|seven|eight|nine|[0-9]").unwrap();
    input
        .map(|line| {
            // let digits = digit_regex.find_iter(line).collect_vec();
            let mut digits = (0..line.len()).filter_map(|idx| digit_regex.find_at(line, idx));
            let tens = digits.next().unwrap();
            let ones = digits.next_back();
            let tens = digit_value(tens.as_str());
            let ones = ones.map(|ones| digit_value(ones.as_str())).unwrap_or(tens);
            tens * 10 + ones
        })
        .sum()
}

fn part_2(input: &str) -> u64 {
    fn find_first_digit(input: &str) -> u8 {
        let slice_first_or_empty = |n: usize| input.get(..n).unwrap_or("");
        if let b @ (b'1'..=b'9') = input.as_bytes()[0] {
            b - b'0'
        } else if let ("one", b, _, _) | ("two", _, b, _) | ("six", _, _, b) =
            (slice_first_or_empty(3), 1, 2, 6)
        {
            b
        } else if let ("four", b, _, _) | ("five", _, b, _) | ("nine", _, _, b) =
            (slice_first_or_empty(4), 4, 5, 9)
        {
            b
        } else if let ("three", b, _, _) | ("seven", _, b, _) | ("eight", _, _, b) =
            (slice_first_or_empty(5), 3, 7, 8)
        {
            b
        } else {
            find_first_digit(&input[1..])
        }
    }
    fn find_last_digit(input: &str) -> u8 {
        let slice_last_or_empty = |n: usize| {
            input
                .len()
                .checked_sub(n)
                .and_then(|start| input.get(start..))
                .unwrap_or("")
        };
        if let b @ (b'1'..=b'9') = input.as_bytes().last().unwrap() {
            b - b'0'
        } else if let ("one", b, _, _) | ("two", _, b, _) | ("six", _, _, b) =
            (slice_last_or_empty(3), 1, 2, 6)
        {
            b
        } else if let ("four", b, _, _) | ("five", _, b, _) | ("nine", _, _, b) =
            (slice_last_or_empty(4), 4, 5, 9)
        {
            b
        } else if let ("three", b, _, _) | ("seven", _, b, _) | ("eight", _, _, b) =
            (slice_last_or_empty(5), 3, 7, 8)
        {
            b
        } else {
            find_last_digit(&input[..input.len() - 1])
        }
    }

    let input = input.lines().map(str::trim);
    input
        .map(|line| find_first_digit(line) as u64 * 10 + find_last_digit(line) as u64)
        .sum()
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    if let Err(error) = aoc_magic!(session, 2023:1:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    if let Err(error) = aoc_magic!(session, 2023:1:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}

#[test]
fn example() {
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    assert_eq!(part_1(input), 142);
    let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    assert_eq!(part_2(input), 281);
}

#[test]
fn regex() {
    use itertools::Itertools;
    let digit_regex = Regex::new("one|two|three|four|five|six|seven|eight|nine|[0-9]").unwrap();
    let s = "one43twothreefourfivesixseveneightnine023451223639";
    let matches = digit_regex.find_iter(s).collect_vec();
    assert_eq!(digit_value(matches[0].as_str()), 1);
    assert_eq!(digit_value(matches[1].as_str()), 4);
    assert_eq!(digit_value(matches[2].as_str()), 3);
    assert_eq!(digit_value(matches[3].as_str()), 2);
}
