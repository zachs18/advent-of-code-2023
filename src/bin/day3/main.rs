#![allow(unused_imports)]
use aoc_2023::*;
use aoc_driver::*;
use itertools::Itertools;
use regex::Regex;

fn is_symbol(b: u8) -> bool {
    !b.is_ascii_digit() && b != b'.'
}

fn part_1(input: &str) -> u64 {
    let board = input.lines().map(str::trim).collect_vec();
    let width = board[0].len();
    let height = board.len();
    let mut part_number_sum = 0;
    let num_regex = Regex::new("[0-9]+").unwrap();
    for (row_idx, row) in board.iter().copied().enumerate() {
        let row_bytes = row.as_bytes();
        'numbers: for number in num_regex.find_iter(row) {
            let start = number.start();
            let adjacent_start = start.saturating_sub(1);
            let end = number.end();
            let adjacent_end = width.min(end + 1);
            let number = number.as_str().parse::<u64>().unwrap();
            if (start != 0 && is_symbol(row_bytes[start - 1]))
                || (end != width && is_symbol(row_bytes[end]))
                || (row_idx != 0
                    && board[row_idx - 1][adjacent_start..adjacent_end]
                        .bytes()
                        .any(is_symbol))
                || (row_idx != height - 1
                    && board[row_idx + 1][adjacent_start..adjacent_end]
                        .bytes()
                        .any(is_symbol))
            {
                part_number_sum += number;
                continue 'numbers;
            }
        }
    }
    part_number_sum
}

fn part_2(input: &str) -> u64 {
    let board = input.lines().map(str::trim).collect_vec();
    let mut gear_ratio_sum = 0;
    let num_regex = Regex::new("[0-9]+").unwrap();
    for row in 0..board.len() {
        for gear_idx in board[row]
            .bytes()
            .enumerate()
            .filter_map(|(idx, b)| (b == b'*').then_some(idx))
        {
            let above = board
                .get(row.saturating_sub(1))
                .into_iter()
                .flat_map(|line| num_regex.find_iter(line));
            let this_line = num_regex.find_iter(&board[row]);
            let below = board
                .get(row + 1)
                .into_iter()
                .flat_map(|line| num_regex.find_iter(line));
            let mut gear_ratio = 1;
            let mut adjacent_count = 0;
            for number in above.chain(this_line).chain(below) {
                if (number.start().saturating_sub(1)..=number.end()).contains(&gear_idx) {
                    gear_ratio *= number.as_str().parse::<u64>().unwrap();
                    adjacent_count += 1;
                }
            }
            if adjacent_count == 2 {
                gear_ratio_sum += gear_ratio;
            }
        }
    }
    gear_ratio_sum
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    if let Err(error) = aoc_magic!(session, 2023:3:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    if let Err(error) = aoc_magic!(session, 2023:3:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}

#[test]
fn example() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(part_1(input), 4361);
    assert_eq!(part_2(input), 467835);
}
