#![allow(unused_imports)]
use aoc_2023::*;
use aoc_driver::*;
use itertools::Itertools;
use zachs18_stdx::*;

fn tilt_column_north(column: &mut [u8]) {
    let mut read_head = 0;
    let mut write_head = 0;
    while read_head < column.len() {
        match column[read_head] {
            b'O' => {
                if read_head != write_head {
                    column[write_head] = b'O';
                    column[read_head] = b'.';
                }
                read_head += 1;
                write_head += 1;
            }
            b'.' => {
                read_head += 1;
            }
            b'#' => {
                read_head += 1;
                write_head = read_head;
            }
            _ => unreachable!(),
        }
    }
}

fn calculate_column_load(column: &[u8]) -> usize {
    let n = column.len();
    column
        .iter()
        .enumerate()
        .map(|(y, &rock)| if rock == b'O' { n - y } else { 0 })
        .sum()
}

fn part_1(input: &str) -> usize {
    let data = input
        .lines()
        .map(str::trim)
        .map(str::as_bytes)
        .collect_vec();
    let h = data.len();
    let w = data[0].len();
    let mut columns = (0..w)
        .map(|x| (0..h).map(|y| data[y][x]).collect_vec())
        .collect_vec();
    for column in &mut columns {
        tilt_column_north(column);
    }
    columns
        .into_iter()
        .map(|column| calculate_column_load(&column))
        .sum()
}

fn part_2(input: &str) -> u64 {
    todo!()
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    if let Err(error) = aoc_magic!(session, 2023:14:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    if let Err(error) = aoc_magic!(session, 2023:14:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}

#[test]
fn example() {
    let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    assert_eq!(part_1(input), 136);
    // assert_eq!(part_2(input), 42);
}
