#![allow(unused_imports)]
use aoc_2023::*;
use aoc_driver::*;
use itertools::Itertools;
use zachs18_stdx::*;

struct Data {
    galaxy_locations: Vec<(usize, usize)>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

fn parse(input: &str) -> Data {
    let data = input
        .lines()
        .map(str::trim)
        .map(str::as_bytes)
        .collect_vec();
    let h = data.len();
    let w = data[0].len();
    let empty_rows = (0..h)
        .filter(|&row| (0..w).all(|col| data[row][col] == b'.'))
        .collect_vec();
    let empty_cols = (0..w)
        .filter(|&col| (0..h).all(|row| data[row][col] == b'.'))
        .collect_vec();
    let galaxy_locations = (0..h)
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
    Data {
        galaxy_locations,
        empty_rows,
        empty_cols,
    }
}

fn distance(
    (y1, x1): (usize, usize),
    (y2, x2): (usize, usize),
    empty_rows: &[usize],
    empty_cols: &[usize],
    empty_space_multiplier: usize,
) -> usize {
    let (y1, y2) = (Ord::min(y1, y2), Ord::max(y1, y2));
    let (x1, x2) = (Ord::min(x1, x2), Ord::max(x1, x2));
    (y2 - y1)
        + (x2 - x1)
        + empty_rows.iter().filter(|y| (y1..y2).contains(&y)).count() * (empty_space_multiplier - 1)
        + empty_cols.iter().filter(|x| (x1..x2).contains(&x)).count() * (empty_space_multiplier - 1)
}

fn part_1(input: &Data) -> usize {
    let Data {
        galaxy_locations,
        empty_rows,
        empty_cols,
    } = input;
    let mut acc = 0;
    for i in 0..galaxy_locations.len() {
        for j in 0..i {
            acc += distance(
                galaxy_locations[i],
                galaxy_locations[j],
                &empty_rows,
                &empty_cols,
                2,
            );
        }
    }
    acc
}

fn part_2(input: &Data) -> usize {
    let Data {
        galaxy_locations,
        empty_rows,
        empty_cols,
    } = input;
    let mut acc = 0;
    for i in 0..galaxy_locations.len() {
        for j in 0..i {
            acc += distance(
                galaxy_locations[i],
                galaxy_locations[j],
                &empty_rows,
                &empty_cols,
                1_000_000,
            );
        }
    }
    acc
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    let mut both = PreParsed::new(parse, part_1, part_2);
    let part_2 = both.part_2();
    if let Err(error) = aoc_magic!(session, 2023:11:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    let part_1 = both.part_1();
    if let Err(error) = aoc_magic!(session, 2023:11:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}

#[test]
fn example() {
    let input = "#.#";
    assert_eq!(part_1(&parse(input)), 3);
    let input = "#.#.#";
    assert_eq!(part_1(&parse(input)), 12);
    let input = "\
#.
.#";
    assert_eq!(part_1(&parse(input)), 2);
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
    assert_eq!(part_1(&parse(input)), 374);
}
