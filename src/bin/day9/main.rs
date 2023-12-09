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

fn parse(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|field| field.parse::<isize>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn part_1(seqs: &Vec<Vec<isize>>) -> isize {
    seqs.iter().map(|seq| predict_next(seq)).sum()
}

fn part_2(seqs: &Vec<Vec<isize>>) -> isize {
    seqs.iter().map(|seq| predict_prev(seq)).sum()
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    let mut both = PreParsed::new(parse, part_1, part_2);
    let part_2 = both.part_2();
    if let Err(error) = aoc_magic!(session, 2023:9:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    let part_1 = both.part_1();
    if let Err(error) = aoc_magic!(session, 2023:9:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}

#[test]
fn example() {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    let mut both = PreParsed::new(parse, part_1, part_2);
    let part_1 = both.part_1();
    assert_eq!(part_1(input), &114);
    let part_2 = both.part_2();
    assert_eq!(part_2(input), &2);
}
