#![allow(unused_imports)]
use aoc_2023::*;
use aoc_driver::*;
use itertools::Itertools;
use zachs18_stdx::*;

fn hash(s: &str) -> usize {
    s.bytes().fold(0, |mut acc, byte| {
        acc += byte as usize;
        acc *= 17;
        acc %= 256;
        acc
    })
}

fn part_1(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

fn part_2(input: &str) -> u64 {
    todo!()
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    if let Err(error) = aoc_magic!(session, 2023:15:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    if let Err(error) = aoc_magic!(session, 2023:15:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}

#[test]
fn example() {
    let input = "HASH";
    assert_eq!(part_1(input), 52);
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    assert_eq!(part_1(input), 42);
    assert_eq!(part_2(input), 42);
}
