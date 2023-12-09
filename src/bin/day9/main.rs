#![allow(unused_imports)]
use aoc_2023::*;
use aoc_driver::*;
use itertools::Itertools;
use zachs18_stdx::*;

fn predict_next(sequence: &[isize]) -> isize {
    if (1..sequence.len()).all(|idx| sequence[idx] == sequence[0]) {
        return sequence[0];
    }
    let last = *sequence.last().unwrap();
    let sequence = sequence.windows(2).map(|win| win[1] - win[0]).collect_vec();
    predict_next(&sequence) + last
}

fn predict_prev(sequence: &[isize]) -> isize {
    if (1..sequence.len()).all(|idx| sequence[idx] == sequence[0]) {
        return sequence[0];
    }
    let first = *sequence.first().unwrap();
    let sequence = sequence.windows(2).map(|win| win[1] - win[0]).collect_vec();
    first - predict_prev(&sequence)
}

fn part_1(input: &str) -> isize {
    let seqs = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|field| field.parse::<isize>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    seqs.iter().map(|seq| predict_next(seq)).sum()
}

fn part_2(input: &str) -> isize {
    let seqs = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|field| field.parse::<isize>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    seqs.iter().map(|seq| predict_prev(seq)).sum()
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    if let Err(error) = aoc_magic!(session, 2023:9:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    if let Err(error) = aoc_magic!(session, 2023:9:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}

#[test]
fn example() {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    assert_eq!(part_1(input), 114);
    assert_eq!(part_2(input), 2);
}
