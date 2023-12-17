use std::collections::HashMap;

use aoc_2023::*;
use aoc_driver::*;
use itertools::Itertools;
use zachs18_stdx::*;

struct Row {
    known_cells: Vec<Option<bool>>,
    expected_chunks: Vec<usize>,
}

fn parse(input: &str) -> Vec<Row> {
    input
        .lines()
        .map(str::trim)
        .map(|line| {
            let (known_cells, expected_chunks) = line.split_once(' ').unwrap();
            let known_cells: Vec<Option<bool>> = known_cells
                .chars()
                .map(|c| match c {
                    '.' => Some(false),
                    '?' => None,
                    '#' => Some(true),
                    _ => unreachable!(),
                })
                .collect();
            let expected_chunks = expected_chunks
                .split(',')
                .map(|field| field.parse::<usize>().unwrap())
                .collect_vec();
            Row {
                known_cells,
                expected_chunks,
            }
        })
        .collect()
}

fn solve_row(
    known_cells: &[Option<bool>],
    expected_chunks: &[usize],
    known_start_idx: usize,
    expected_start_idx: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(&result) = cache.get(&(known_start_idx, expected_start_idx)) {
        return result;
    } else if expected_start_idx == expected_chunks.len() {
        // No more sections to place, just check if all remaining cells are empty.
        let valid = known_cells[known_start_idx..]
            .iter()
            .all(|cell| cell.is_none_or(|cell| !cell));
        cache.insert((known_start_idx, expected_start_idx), valid as usize);
        return valid as usize;
    } else if known_start_idx == known_cells.len() {
        // Trying to fit expected sections into no space
        return 0;
    } else if expected_start_idx == expected_chunks.len() - 1 {
        let next_expected = expected_chunks[expected_start_idx];
        let mut acc = 0;
        let mut section = &known_cells[known_start_idx..];
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

    let next_expected = expected_chunks[expected_start_idx];
    let next_expected_start_idx = expected_start_idx + 1;

    let mut acc = 0;
    let mut this_known_start_idx = known_start_idx;
    while known_cells.len() - this_known_start_idx > next_expected {
        let section = &known_cells[this_known_start_idx..];
        if section[..next_expected]
            .iter()
            .all(|cell| cell.is_none_or(|cell| cell))
            && section[next_expected].is_none_or(|cell| !cell)
        {
            acc += solve_row(
                known_cells,
                expected_chunks,
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

fn part_1(input: &Vec<Row>) -> usize {
    input
        .iter()
        .map(|row| {
            let Row {
                known_cells,
                expected_chunks,
            } = row;
            solve_row(known_cells, expected_chunks, 0, 0, &mut HashMap::new())
        })
        .sum()
}

fn part_2(input: &Vec<Row>) -> usize {
    input
        .iter()
        .map(|row| {
            let Row {
                known_cells,
                expected_chunks,
            } = row;
            let mut known_cells2 = Vec::with_capacity(known_cells.len() * 5 + 4);
            known_cells2.extend_from_slice(known_cells);
            known_cells2.push(None);
            known_cells2.extend_from_slice(known_cells);
            known_cells2.push(None);
            known_cells2.extend_from_slice(known_cells);
            known_cells2.push(None);
            known_cells2.extend_from_slice(known_cells);
            known_cells2.push(None);
            known_cells2.extend_from_slice(known_cells);
            let mut expected_chunks2 = Vec::with_capacity(expected_chunks.len() * 5);
            expected_chunks2.extend_from_slice(expected_chunks);
            expected_chunks2.extend_from_slice(expected_chunks);
            expected_chunks2.extend_from_slice(expected_chunks);
            expected_chunks2.extend_from_slice(expected_chunks);
            expected_chunks2.extend_from_slice(expected_chunks);
            solve_row(&known_cells2, &expected_chunks2, 0, 0, &mut HashMap::new())
        })
        .sum()
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    let mut both = PreParsed::new(parse, part_1, part_2);
    let part_2 = both.part_2();
    if let Err(error) = aoc_magic!(session, 2023:12:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    let part_1 = both.part_1();
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
    let mut both = PreParsed::new(parse, part_1, part_2);
    let part_1 = both.part_1();
    assert_eq!(part_1(input), &21);
    let part_2 = both.part_2();
    assert_eq!(part_2(input), &525152);
}
