use std::collections::HashMap;

use aoc_driver::*;
use itertools::Itertools;
use num_integer::Integer;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum Direction {
    Left = 0,
    Right = 1,
}

type Graph<'a> = HashMap<&'a str, [&'a str; 2]>;

fn parse(input: &str) -> (Vec<Direction>, Graph<'_>) {
    let mut lines = input.lines();
    let directions = lines.next().unwrap();
    let directions = directions
        .trim()
        .bytes()
        .map(|b| match b {
            b'L' => Direction::Left,
            b'R' => Direction::Right,
            _ => unreachable!(),
        })
        .collect_vec();
    let _ = lines.next();
    let map = lines
        .map(|line| {
            let src = &line[..3];
            let dst_left = &line[7..10];
            let dst_right = &line[12..15];
            (src, [dst_left, dst_right])
        })
        .collect();
    (directions, map)
}

fn part_1(input: &str) -> usize {
    let (directions, map) = parse(input);
    let mut node = "AAA";
    for (idx, direction) in directions.iter().copied().cycle().enumerate() {
        if node == "ZZZ" {
            return idx;
        }
        node = map[node][direction as usize]
    }
    unreachable!()
}

fn part_2(input: &str) -> usize {
    let (directions, map) = parse(input);
    let starting_nodes = map
        .iter()
        .flat_map(|(&src, &[dst1, dst2])| [src, dst1, dst2])
        .filter(|node| node.ends_with('A'))
        .collect_vec();
    let lengths = starting_nodes
        .iter()
        .copied()
        .map(|mut node| {
            for (idx, direction) in directions.iter().copied().cycle().enumerate() {
                if node.ends_with('Z') {
                    return idx;
                }
                node = map[node][direction as usize]
            }
            unreachable!()
        })
        .collect_vec();
    lengths.into_iter().reduce(|a, b| a.lcm(&b)).unwrap()
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    if let Err(error) = aoc_magic!(session, 2023:8:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    if let Err(error) = aoc_magic!(session, 2023:8:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}

#[test]
fn example() {
    let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    assert_eq!(part_1(input), 2);

    let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    assert_eq!(part_1(input), 6);

    let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    assert_eq!(part_2(input), 6);
}
