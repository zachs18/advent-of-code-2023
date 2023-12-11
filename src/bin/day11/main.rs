#![allow(unused_imports)]
use aoc_2023::*;
use aoc_driver::*;
use itertools::Itertools;
use zachs18_stdx::*;

fn distance(
    p1: (usize, usize),
    p2: (usize, usize),
    empty_rows: &[usize],
    empty_cols: &[usize],
    empty_space_multiplier: usize,
) -> usize {
    let (y1, x1) = p1;
    let (y2, x2) = p2;
    let (y1, y2) = (Ord::min(y1, y2), Ord::max(y1, y2));
    let (x1, x2) = (Ord::min(x1, x2), Ord::max(x1, x2));
    (y2 - y1)
        + (x2 - x1)
        + empty_rows.iter().filter(|y| (y1..y2).contains(&y)).count() * empty_space_multiplier
        + empty_cols.iter().filter(|x| (x1..x2).contains(&x)).count() * empty_space_multiplier
}

fn part_1(input: &str) -> usize {
    let data = input
        .lines()
        .map(str::trim)
        .map(|line| line.as_bytes())
        .collect_vec();
    let h = data.len();
    let w = data[0].len();
    let empty_rows = (0..h)
        .filter(|&row| data[row].iter().all(|&b| b == b'.'))
        .collect_vec();
    let empty_cols = (0..w)
        .filter(|&col| (0..h).all(|row| data[row][col] == b'.'))
        .collect_vec();
    let locations = (0..h)
        .flat_map(|row| {
            let data = &data;
            (0..w).filter_map(move |col| {
                if data[row][col] == b'#' {
                    Some((row, col))
                } else {
                    None
                }
            })
        })
        .collect_vec();
    let mut acc = 0;
    for i in 0..locations.len() {
        for j in 0..i {
            acc += distance(locations[i], locations[j], &empty_rows, &empty_cols, 1);
        }
    }
    acc
}

fn part_2(input: &str) -> usize {
    let data = input
        .lines()
        .map(str::trim)
        .map(|line| line.as_bytes())
        .collect_vec();
    let h = data.len();
    let w = data[0].len();
    let empty_rows = (0..h)
        .filter(|&row| data[row].iter().all(|&b| b == b'.'))
        .collect_vec();
    let empty_cols = (0..w)
        .filter(|&col| (0..h).all(|row| data[row][col] == b'.'))
        .collect_vec();
    let locations = (0..h)
        .flat_map(|row| {
            let data = &data;
            (0..w).filter_map(move |col| {
                if data[row][col] == b'#' {
                    Some((row, col))
                } else {
                    None
                }
            })
        })
        .collect_vec();
    let mut acc = 0;
    for i in 0..locations.len() {
        for j in 0..i {
            acc += distance(
                locations[i],
                locations[j],
                &empty_rows,
                &empty_cols,
                1_000_000 - 1,
            );
        }
    }
    acc
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    if let Err(error) = aoc_magic!(session, 2023:11:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    if let Err(error) = aoc_magic!(session, 2023:11:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}

#[test]
fn example() {
    let input = "#.#";
    assert_eq!(part_1(input), 3);
    let input = "#.#.#";
    assert_eq!(part_1(input), 12);
    let input = "\
#.
.#";
    assert_eq!(part_1(input), 2);
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    assert_eq!(part_1(input), 374);
}
