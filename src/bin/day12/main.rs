#![allow(unused_imports)]
use std::collections::HashMap;

use aoc_2023::*;
use aoc_driver::*;
use itertools::Itertools;
use regex::Regex;
use zachs18_stdx::*;

fn solve(
    known: &[Option<bool>],
    expected: &[usize],
    known_start_idx: usize,
    expected_start_idx: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(&result) = cache.get(&(known_start_idx, expected_start_idx)) {
        return result;
    } else if expected_start_idx == expected.len() {
        // No more sections to place, just check if all remaining cells are empty.
        let valid = known[known_start_idx..]
            .iter()
            .all(|cell| cell.is_none_or(|cell| !cell));
        cache.insert((known_start_idx, expected_start_idx), valid as usize);
        return valid as usize;
    } else if known_start_idx == known.len() {
        // Trying to fit expected sections into no space
        return 0;
    } else if expected_start_idx == expected.len() - 1 {
        let next_expected = expected[expected_start_idx];
        let mut acc = 0;
        let mut section = &known[known_start_idx..];
        while section.len() >= next_expected {
            if section[..next_expected]
                .iter()
                .all(|cell| cell.is_none_or(|cell| cell))
                && section[next_expected..]
                    .iter()
                    .all(|cell| cell.is_none_or(|cell| !cell))
            {
                acc += 1;
            }
            if section[0] == Some(true) {
                break;
            }
            section = &section[1..];
        }
        cache.insert((known_start_idx, expected_start_idx), acc);
        return acc;
    }

    let next_expected = expected[expected_start_idx];
    let next_expected_start_idx = expected_start_idx + 1;

    let mut acc = 0;
    let mut this_known_start_idx = known_start_idx;
    while known.len() - this_known_start_idx > next_expected {
        let section = &known[this_known_start_idx..];
        if section.len() == next_expected {}
        if section[..next_expected]
            .iter()
            .all(|cell| cell.is_none_or(|cell| cell))
            && section[next_expected].is_none_or(|cell| !cell)
        {
            acc += solve(
                known,
                expected,
                this_known_start_idx + next_expected + 1,
                next_expected_start_idx,
                cache,
            );
        }
        if section[0] == Some(true) {
            break;
        }
        this_known_start_idx += 1;
    }
    cache.insert((known_start_idx, expected_start_idx), acc);
    acc
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(str::trim)
        .map(|line| {
            let (known, expected) = line.split_once(' ').unwrap();
            let known: Vec<Option<bool>> = known
                .chars()
                .map(|c| match c {
                    '.' => Some(false),
                    '?' => None,
                    '#' => Some(true),
                    _ => unreachable!(),
                })
                .collect();
            let expected = expected
                .split(',')
                .map(|field| field.parse::<usize>().unwrap())
                .collect_vec();
            solve(&known, &expected, 0, 0, &mut HashMap::new())
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(str::trim)
        .map(|line| {
            let (known, expected) = line.split_once(' ').unwrap();
            let known1: Vec<Option<bool>> = known
                .chars()
                .map(|c| match c {
                    '.' => Some(false),
                    '?' => None,
                    '#' => Some(true),
                    _ => unreachable!(),
                })
                .collect();
            let mut known = std::iter::repeat(known1.iter().copied().chain([None]))
                .take(5)
                .flatten()
                .collect_vec();
            let _ = known.pop(); // extra separator
            let expected1 = expected
                .split(',')
                .map(|field| field.parse::<usize>().unwrap())
                .collect_vec();
            let expected = std::iter::repeat(expected1).take(5).flatten().collect_vec();
            solve(&known, &expected, 0, 0, &mut HashMap::new())
        })
        .sum()
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    if let Err(error) = aoc_magic!(session, 2023:12:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    if let Err(error) = aoc_magic!(session, 2023:12:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}

#[test]
fn example() {
    let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    assert_eq!(part_1(input), 21);
    assert_eq!(part_2(input), 525152);
}
