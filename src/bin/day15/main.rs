use aoc_driver::*;
use indexmap::IndexMap;
use itertools::Itertools;

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

/// None is remove
fn parse_lens(s: &str) -> (&str, Option<usize>) {
    if let Some((label, length)) = s.split_once('=') {
        (label, Some(length.parse().unwrap()))
    } else {
        (&s[..s.len() - 1], None)
    }
}

fn part_2(input: &str) -> usize {
    let values = input.split(',').collect_vec();
    let mut boxes: Vec<IndexMap<&str, usize>> = vec![IndexMap::new(); 256];
    for &value in &values {
        let (label, focal_length) = parse_lens(value);
        let hash = hash(label);
        match focal_length {
            Some(length) => {
                boxes[hash].insert(label, length);
            }
            None => {
                boxes[hash].shift_remove(label);
            }
        }
    }
    boxes
        .iter()
        .enumerate()
        .map(|(bxidx, bx)| {
            bx.iter()
                .enumerate()
                .map(|(idx, (_, length))| (bxidx + 1) * (idx + 1) * length)
                .sum::<usize>()
        })
        .sum()
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
    assert_eq!(part_1(input), 1320);
    assert_eq!(part_2(input), 145);
}
