#![allow(unused_imports)]
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, VecDeque},
    ops::RangeInclusive,
};

use aoc_2023::*;
use aoc_driver::*;
use itertools::Itertools;
use zachs18_stdx::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Cube {
    x: usize,
    y: usize,
    z: usize,
    brick_id: usize,
}

fn parse_xyz(s: &str) -> (usize, usize, usize) {
    let mut iter = s.split(',').map(|s| s.parse().unwrap());
    (
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    )
}

fn sorted_range_inclusive(a: usize, b: usize) -> RangeInclusive<usize> {
    if a < b {
        a..=b
    } else {
        b..=a
    }
}

fn part_1(input: &str) -> usize {
    let mut next_brick_id = {
        // Note: brick id usize::MAX is the ground
        let mut next_id = 0;
        move || {
            next_id += 1;
            next_id - 1
        }
    };

    // map from position to brick id
    let mut map: HashMap<(usize, usize, usize), usize> = HashMap::new();
    // map from brick id to positions
    let mut positions: HashMap<usize, Vec<(usize, usize, usize)>> = HashMap::new();

    for line in input.lines() {
        let (start, end) = line.trim().split_once('~').unwrap();
        let (x1, y1, z1) = parse_xyz(start);
        let (x2, y2, z2) = parse_xyz(end);

        let brick_id = next_brick_id();
        let positions = positions
            .entry(brick_id)
            .and_modify(|_| unreachable!())
            .or_default();
        for x in sorted_range_inclusive(x1, x2) {
            for y in sorted_range_inclusive(y1, y2) {
                for z in sorted_range_inclusive(z1, z2) {
                    map.insert((x, y, z), brick_id);
                    positions.push((x, y, z));
                }
            }
        }
    }
    let brick_count = positions.len();

    let mut rester_to_restees: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();

    while rester_to_restees.len() < brick_count {
        // find lowest non-resting cell and move its brick down until it rests on something.
        // Note that cells at z=0 rest on the floor. (z is vertical)

        let (floating_brick_with_lowest_z, the_z) = (0..brick_count)
            .filter(|brick| !rester_to_restees.contains_key(brick))
            .map(|brick| {
                // The positions in a brick are sorted by increasing coordinates (only one coordinate changes)
                // so the lowest Z occurs in the position at index 0.
                (brick, positions[&brick][0].2)
            })
            .min_by_key(|&(_brick, low_z)| low_z)
            .unwrap();
        if the_z == 0 {
            rester_to_restees.insert(floating_brick_with_lowest_z, BTreeSet::from([usize::MAX]));
            continue;
        }
        let mut resting_on_any = false;
        let mut is_resting_on = |brick_id: usize| {
            rester_to_restees
                .entry(floating_brick_with_lowest_z)
                .or_default()
                .insert(brick_id);
            resting_on_any = true;
        };
        for &(x, y, z) in &positions[&floating_brick_with_lowest_z] {
            if let Some(&resting_on_brick_id) = map.get(&(x, y, z - 1)) {
                // don't count vertical bricks as resting on themselves
                if resting_on_brick_id != floating_brick_with_lowest_z {
                    is_resting_on(resting_on_brick_id);
                }
            }
        }
        if !resting_on_any {
            // lower all z of this brick by one
            for &mut (x, y, ref mut z) in positions.get_mut(&floating_brick_with_lowest_z).unwrap()
            {
                map.remove(&(x, y, *z));
                *z -= 1;
                map.insert((x, y, *z), floating_brick_with_lowest_z);
            }
        }
    }

    let mut restee_to_resters: BTreeMap<usize, BTreeSet<usize>> =
        BTreeMap::from_iter((0..brick_count).map(|brick_id| (brick_id, BTreeSet::new())));
    for (&rester, restees) in rester_to_restees.iter() {
        for &restee in restees {
            restee_to_resters.entry(restee).or_default().insert(rester);
        }
    }

    for (rester, restees) in &rester_to_restees {
        eprintln!("{rester} is resting on {restees:?}");
    }

    for (restee, resters) in &restee_to_resters {
        eprintln!("{restee} is rested on by {resters:?}");
    }

    (0..brick_count)
        .filter(|brick_id| {
            for rester in &restee_to_resters[&brick_id] {
                if rester_to_restees[&rester].len() == 1 {
                    return false;
                }
            }
            true
        })
        .count()
}

fn part_2(input: &str) -> usize {
    let mut next_brick_id = {
        // Note: brick id usize::MAX is the ground
        let mut next_id = 0;
        move || {
            next_id += 1;
            next_id - 1
        }
    };

    // map from position to brick id
    let mut map: HashMap<(usize, usize, usize), usize> = HashMap::new();
    // map from brick id to positions
    let mut positions: HashMap<usize, Vec<(usize, usize, usize)>> = HashMap::new();

    for line in input.lines() {
        let (start, end) = line.trim().split_once('~').unwrap();
        let (x1, y1, z1) = parse_xyz(start);
        let (x2, y2, z2) = parse_xyz(end);

        let brick_id = next_brick_id();
        let positions = positions
            .entry(brick_id)
            .and_modify(|_| unreachable!())
            .or_default();
        for x in sorted_range_inclusive(x1, x2) {
            for y in sorted_range_inclusive(y1, y2) {
                for z in sorted_range_inclusive(z1, z2) {
                    map.insert((x, y, z), brick_id);
                    positions.push((x, y, z));
                }
            }
        }
    }
    let brick_count = positions.len();

    let mut rester_to_restees: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();

    while rester_to_restees.len() < brick_count {
        // find lowest non-resting cell and move its brick down until it rests on something.
        // Note that cells at z=0 rest on the floor. (z is vertical)

        let (floating_brick_with_lowest_z, the_z) = (0..brick_count)
            .filter(|brick| !rester_to_restees.contains_key(brick))
            .map(|brick| {
                // The positions in a brick are sorted by increasing coordinates (only one coordinate changes)
                // so the lowest Z occurs in the position at index 0.
                (brick, positions[&brick][0].2)
            })
            .min_by_key(|&(_brick, low_z)| low_z)
            .unwrap();
        if the_z == 0 {
            rester_to_restees.insert(floating_brick_with_lowest_z, BTreeSet::from([usize::MAX]));
            continue;
        }
        let mut resting_on_any = false;
        let mut is_resting_on = |brick_id: usize| {
            rester_to_restees
                .entry(floating_brick_with_lowest_z)
                .or_default()
                .insert(brick_id);
            resting_on_any = true;
        };
        for &(x, y, z) in &positions[&floating_brick_with_lowest_z] {
            if let Some(&resting_on_brick_id) = map.get(&(x, y, z - 1)) {
                // don't count vertical bricks as resting on themselves
                if resting_on_brick_id != floating_brick_with_lowest_z {
                    is_resting_on(resting_on_brick_id);
                }
            }
        }
        if !resting_on_any {
            // lower all z of this brick by one
            for &mut (x, y, ref mut z) in positions.get_mut(&floating_brick_with_lowest_z).unwrap()
            {
                map.remove(&(x, y, *z));
                *z -= 1;
                map.insert((x, y, *z), floating_brick_with_lowest_z);
            }
        }
    }

    let mut restee_to_resters: BTreeMap<usize, BTreeSet<usize>> =
        BTreeMap::from_iter((0..brick_count).map(|brick_id| (brick_id, BTreeSet::new())));
    for (&rester, restees) in rester_to_restees.iter() {
        for &restee in restees {
            restee_to_resters.entry(restee).or_default().insert(rester);
        }
    }

    for (rester, restees) in &rester_to_restees {
        eprintln!("{rester} is resting on {restees:?}");
    }

    for (restee, resters) in &restee_to_resters {
        eprintln!("{restee} is rested on by {resters:?}");
    }

    (0..brick_count)
        .map(|brick_id| {
            let mut would_disintegrate: BTreeSet<usize> = BTreeSet::from([brick_id]);
            let mut queue = VecDeque::from([brick_id]);
            while let Some(brick_id) = queue.pop_front() {
                for &rester in &restee_to_resters[&brick_id] {
                    if rester_to_restees[&rester]
                        .difference(&would_disintegrate)
                        .next()
                        .is_none()
                    {
                        if would_disintegrate.insert(rester) {
                            queue.push_back(rester);
                        }
                    }
                }
            }
            would_disintegrate.len() - 1
        })
        .sum()
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    if let Err(error) = aoc_magic!(session, 2023:22:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    if let Err(error) = aoc_magic!(session, 2023:22:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}

#[test]
fn example() {
    let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
    assert_eq!(part_1(input), 5);
    assert_eq!(part_2(input), 7);
}
