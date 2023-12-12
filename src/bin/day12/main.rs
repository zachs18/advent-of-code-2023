#![allow(unused_imports)]
use aoc_2023::*;
use aoc_driver::*;
use itertools::Itertools;
use regex::Regex;
use zachs18_stdx::*;

fn all(expected: &[usize], length: usize) -> Vec<Vec<bool>> {
    if expected.len() == 0 {
        return vec![vec![false; length]];
    } else if expected.len() == 1 {
        if expected[0] <= length {
            return (0..=length - expected[0])
                .map(|a| {
                    let mut v = vec![false; length];
                    v[a..][..expected[0]].fill(true);
                    v
                })
                .collect();
        } else {
            return vec![];
        }
    } else if 2 * expected.len() - 1 > length || expected[0] + 1 > length {
        return vec![];
    }
    let mut results = vec![];
    // place first one first, then continue
    {
        let mut prefix = vec![true; expected[0] + 1];
        prefix[expected[0]] = false;
        let length = length - expected[0] - 1;
        let expected = &expected[1..];
        let res = all(expected, length);
        for mut res in res {
            res.splice(..0, prefix.iter().copied());
            results.push(res);
        }
    }
    // remove first cell, then continue
    {
        let mut prefix: Vec<bool> = vec![false];
        let length = length - 1;
        let res = all(expected, length);
        for mut res in res {
            res.splice(..0, prefix.iter().copied());
            results.push(res);
        }
    }

    results
}

fn solve(known: &[Option<bool>], expected: &[usize]) -> usize {
    let possible = all(expected, known.len());
    possible
        .iter()
        .filter(|possiblity| {
            possiblity
                .iter()
                .zip(known)
                .all(|(&lhs, &rhs)| rhs.is_none_or(|rhs| rhs == lhs))
        })
        .count()
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
            dbg!(solve(&known, &expected))
        })
        .sum()
}

fn part_2(input: &str) -> u64 {
    todo!()
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
