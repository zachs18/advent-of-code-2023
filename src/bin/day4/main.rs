#![allow(unused_imports)]
use aoc_2023::*;
use aoc_driver::*;
use itertools::Itertools;

struct Card {
    idx: usize,
    winning: Vec<u32>,
    have: Vec<u32>,
}

impl Card {
    fn matching_cards(&self) -> usize {
        let mut have_winning = 0;
        let mut hidx = 0;
        let mut widx = 0;
        while hidx < self.have.len() && widx < self.winning.len() {
            match Ord::cmp(&self.have[hidx], &self.winning[widx]) {
                std::cmp::Ordering::Less => hidx += 1,
                std::cmp::Ordering::Equal => {
                    have_winning += 1;
                    hidx += 1;
                    widx += 1;
                }
                std::cmp::Ordering::Greater => widx += 1,
            }
        }
        have_winning
    }

    fn point_value(&self) -> u64 {
        (1 << self.matching_cards()) / 2
    }
}

fn parse(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(str::trim)
        .map(|line| {
            let (idx, rest) = line["Card ".len()..].split_once(':').unwrap();
            let (winning, have) = rest.split_once('|').unwrap();
            let idx = idx.trim().parse().unwrap();
            let mut winning = winning
                .split_whitespace()
                .map(|value| value.parse().unwrap())
                .collect_vec();
            winning.sort_unstable();
            let mut have = have
                .split_whitespace()
                .map(|value| value.parse().unwrap())
                .collect_vec();
            have.sort_unstable();
            Card { idx, winning, have }
        })
        .collect_vec()
}

fn part_1(cards: &Vec<Card>) -> u64 {
    cards.iter().map(|card| card.point_value()).sum()
}
fn part_2(cards: &Vec<Card>) -> u64 {
    let mut cards_won = [0; 11];
    let mut total_cards = 0;
    for card in cards {
        let this_count = cards_won[0] + 1;
        total_cards += this_count;
        cards_won[0] = 0;
        cards_won.rotate_left(1);
        let counts = card.matching_cards();
        for idx in 0..counts {
            cards_won[idx] += this_count;
        }
    }

    total_cards
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    let mut both = PreParsed::new(parse, part_1, part_2);
    let part_2 = both.part_2();
    if let Err(error) = aoc_magic!(session, 2023:4:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    let part_1 = both.part_1();
    if let Err(error) = aoc_magic!(session, 2023:4:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}

#[test]
fn example() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    let mut both = PreParsed::new(parse, part_1, part_2);
    let part_1 = both.part_1();
    assert_eq!(part_1(input), &13);
    let part_2 = both.part_2();
    assert_eq!(part_2(input), &30);
}
