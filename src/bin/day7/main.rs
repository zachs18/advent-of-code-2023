#![allow(unused_imports)]
use aoc_2023::*;
use aoc_driver::*;
use itertools::Itertools;
use zachs18_stdx::*;

struct Hand {
    cards: Vec<char>,
    bid: usize,
}

fn rank(card: char) -> u8 {
    match card {
        '2'..='9' => card as u8 - b'0',
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => unreachable!(),
    }
}

fn rank2(card: char) -> u8 {
    match card {
        'J' => 1,
        '2'..='9' => card as u8 - b'0',
        'T' => 10,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => unreachable!(),
    }
}

fn categorize(hand: &Hand) -> (u32, [u8; 5]) {
    let cards: [char; 5] = hand.cards[..].try_into().unwrap();
    let ranks = cards.map(rank);
    let mut sorted_cards = cards;
    sorted_cards.sort_unstable();
    if sorted_cards[4] == sorted_cards[0] {
        // five of a kind
        (7, ranks)
    } else if sorted_cards[0] == sorted_cards[3] || sorted_cards[1] == sorted_cards[4] {
        // four of a kind
        (6, ranks)
    } else if sorted_cards[2] == sorted_cards[4]
        || sorted_cards[2] == sorted_cards[0]
        || sorted_cards[1] == sorted_cards[3]
    {
        // three of a kind or full house
        if (sorted_cards[0] == sorted_cards[2] && sorted_cards[3] == sorted_cards[4])
            || (sorted_cards[0] == sorted_cards[1] && sorted_cards[2] == sorted_cards[4])
        {
            // full house
            (5, ranks)
        } else {
            // three of a kind
            (4, ranks)
        }
    } else if (sorted_cards[0] == sorted_cards[1] || sorted_cards[1] == sorted_cards[2])
        && (sorted_cards[2] == sorted_cards[3] || sorted_cards[3] == sorted_cards[4])
    {
        // two pair
        let mut paired = [sorted_cards[1], sorted_cards[3]].map(rank);
        paired.sort_unstable();

        (3, ranks)
    } else if sorted_cards[0] == sorted_cards[1]
        || sorted_cards[1] == sorted_cards[2]
        || sorted_cards[2] == sorted_cards[3]
        || sorted_cards[3] == sorted_cards[4]
    {
        // one pair
        (2, ranks)
    } else {
        // high card
        (1, ranks)
    }
}

fn can_be_full_house(jokers: usize, sorted_non_joker_cards: &[char]) -> bool {
    jokers >= 3
        || (jokers == 2
            && (sorted_non_joker_cards[0] == sorted_non_joker_cards[1]
                || sorted_non_joker_cards[1] == sorted_non_joker_cards[2]))
        || (jokers == 1
            && ((sorted_non_joker_cards[0] == sorted_non_joker_cards[1]
                && sorted_non_joker_cards[2] == sorted_non_joker_cards[3])
                || sorted_non_joker_cards[0] == sorted_non_joker_cards[2]
                || sorted_non_joker_cards[1] == sorted_non_joker_cards[3]))
}

fn can_be_two_pair(jokers: usize, sorted_non_joker_cards: &[char]) -> bool {
    jokers >= 2
        || (jokers == 1
            && (sorted_non_joker_cards[0] == sorted_non_joker_cards[1]
                || sorted_non_joker_cards[1] == sorted_non_joker_cards[2]
                || sorted_non_joker_cards[2] == sorted_non_joker_cards[3]))
}

fn categorize2(hand: &Hand) -> (u32, [u8; 5]) {
    if !hand.cards.contains(&'J') {
        return categorize(hand);
    }
    // We know there's a joker
    let cards: [char; 5] = hand.cards[..].try_into().unwrap();
    let ranks = cards.map(rank2);
    let jokers = cards.iter().filter(|&&card| card == 'J').count();
    let mut sorted_non_joker_cards = cards
        .iter()
        .copied()
        .filter(|&card| card != 'J')
        .collect_vec();
    sorted_non_joker_cards.sort_unstable();
    if jokers >= 4
        || (jokers == 3 && sorted_non_joker_cards[0] == sorted_non_joker_cards[1])
        || (jokers == 2 && sorted_non_joker_cards[0] == sorted_non_joker_cards[2])
        || (jokers == 1 && sorted_non_joker_cards[0] == sorted_non_joker_cards[3])
    {
        // five of a kind
        (7, ranks)
    } else if jokers == 3
        || (jokers == 2
            && (sorted_non_joker_cards[0] == sorted_non_joker_cards[1]
                || sorted_non_joker_cards[1] == sorted_non_joker_cards[2]))
        || (jokers == 1
            && (sorted_non_joker_cards[0] == sorted_non_joker_cards[2]
                || sorted_non_joker_cards[1] == sorted_non_joker_cards[3]))
    {
        // four of a kind
        (6, ranks)
    } else if jokers == 2
        || (jokers == 1
            && (sorted_non_joker_cards[0] == sorted_non_joker_cards[1]
                || sorted_non_joker_cards[1] == sorted_non_joker_cards[2]
                || sorted_non_joker_cards[2] == sorted_non_joker_cards[3]))
    {
        // three of a kind or full house
        if can_be_full_house(jokers, &sorted_non_joker_cards) {
            // full house
            (5, ranks)
        } else {
            // three of a kind
            (4, ranks)
        }
    } else if can_be_two_pair(jokers, &sorted_non_joker_cards) {
        // two pair
        (3, ranks)
    } else {
        // one pair (we know we have at least this since there's a joker)
        (2, ranks)
    }
}

fn part_1(input: &str) -> usize {
    let mut hands = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.trim().split_once(' ').unwrap();
            Hand {
                cards: hand.trim().chars().collect_vec(),
                bid: bid.trim().parse().unwrap(),
            }
        })
        .collect_vec();
    hands.sort_by_cached_key(categorize);
    hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| (idx + 1) * hand.bid)
        .sum()
}

fn part_2(input: &str) -> usize {
    let mut hands = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.trim().split_once(' ').unwrap();
            Hand {
                cards: hand.trim().chars().collect_vec(),
                bid: bid.trim().parse().unwrap(),
            }
        })
        .collect_vec();
    hands.sort_by_cached_key(categorize2);
    hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| (idx + 1) * hand.bid)
        .sum()
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    if let Err(error) = aoc_magic!(session, 2023:7:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    if let Err(error) = aoc_magic!(session, 2023:7:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}

#[test]
fn example() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    assert_eq!(part_1(input), 6440);
    assert_eq!(part_2(input), 5905);
}
