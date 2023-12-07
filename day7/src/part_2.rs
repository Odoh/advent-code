use log::{debug, info, warn, error};
use itertools::Itertools;
use env_logger;
use nom::multi::{many1, many_m_n};
use nom::{character, sequence, Or};
use nom::character::complete::space1;
use regex::CaptureMatches;
use once_cell::sync::Lazy;
use nom::{bytes::complete::*, combinator::*, error::*, sequence::*, IResult, Parser};

use core::panic;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, BTreeMap};
use std::fmt::Debug;

use advent::{InputSnake, FromRegex};

const NON_JACKS: [Card; 13] = [
    Card { rank: 'A' },
    Card { rank: 'K' },
    Card { rank: 'Q' },
    Card { rank: 'T' },
    Card { rank: '9' },
    Card { rank: '8' },
    Card { rank: '7' },
    Card { rank: '6' },
    Card { rank: '5' },
    Card { rank: '4' },
    Card { rank: '3' },
    Card { rank: '2' },
    Card { rank: '1' },
];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Card {
    rank: char,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card;5],
    bid: u32,
}

#[derive(Debug, PartialEq, Eq)]
enum Score {
    FiveOak,
    FourOak,
    FullHouse,
    ThreeOak,
    TwoPair,
    OnePair,
    HighCard,
}

impl Card {
    fn numeric_value(&self) -> u32 {
        match self.rank {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'T' => 10,
            '1'..='9' => self.rank.to_digit(10).expect("Valid rank"),
            'J' => 0,
            _ => panic!("Unknown character"),
        }
    }
}

fn score_cards<I: Iterator<Item = Card>>(cards: I) -> Score {
    let count_by_rank = cards
        .fold(BTreeMap::new(), |mut acc, card| {
            *acc.entry(card).or_default() += 1;
            acc
        });

    // info!("{:?}", count_by_rank);

    if count_by_rank.len() == 1 {
        return Score::FiveOak;
    }

    if count_by_rank.len() == 2 {
        if count_by_rank.values().contains(&4) {
            return Score::FourOak;
        }
        if count_by_rank.values().contains(&2) && count_by_rank.values().contains(&3) {
            return Score::FullHouse;
        }
    }

    if count_by_rank.len() == 3 {
        if count_by_rank.values().contains(&3) {
            return Score::ThreeOak;
        }
        return Score::TwoPair;
    }

    if count_by_rank.len() == 4 {
        return Score::OnePair;
    }

    return Score::HighCard;
}

impl Hand {
    fn score(&self) -> Score {
        // explode jokers to hands of all other cards
        let (jacks, non_jacks): (Vec<Card>, Vec<Card>) = self.cards.iter().partition(|c| c.rank == 'J');

        // info!("jacks {:?} non_jacks {:?}", jacks, non_jacks);

        // dbg!(NON_JACKS.into_iter().cycle().take(NON_JACKS.len() * 2).combinations(2).collect_vec());
        // todo!();

        let cards_combinations: Vec<Vec<Card>> = NON_JACKS.into_iter()
            .cycle()
            .take(NON_JACKS.len() * jacks.len())
            .combinations(jacks.len())
            .map(|wild_cards| [non_jacks.clone(), wild_cards].concat())
            .collect();

        let score = cards_combinations.into_iter()
            .map(|cards| score_cards(cards.into_iter()))
            .max()
            .unwrap();

        // info!("score: {:?}", score);

        score
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.numeric_value().cmp(&other.numeric_value())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let score_ordering = self.score().numeric_value().cmp(&other.score().numeric_value());
        // info!("{:?} {:?} vs {:?} {:?}", self, self.score(), other, other.score());

        match score_ordering {
            Ordering::Equal => {
                for i in 0..5 {
                    let self_card = self.cards.get(i).unwrap();
                    let other_card = other.cards.get(i).unwrap();
                    let card_ordering = self_card.numeric_value().cmp(&other_card.numeric_value());
                    if let Ordering::Equal = card_ordering {
                        continue;
                    }

                    return card_ordering;
                }

                panic!("Cards should break tie-breaker");
            },
            _ => score_ordering,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Score {
    fn numeric_value(&self) -> u32 {
        match self {
            Score::FiveOak => 7,
            Score::FourOak => 6,
            Score::FullHouse => 5,
            Score::ThreeOak => 4,
            Score::TwoPair => 3,
            Score::OnePair => 2,
            Score::HighCard => 1,
        }
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
        // info!("{:?} {:?} - {:?}", self, other, self.numeric_value().cmp(&other.numeric_value()));
        self.numeric_value().cmp(&other.numeric_value())
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, (cards, bid)) = separated_pair(
        many_m_n(1, 5, character::complete::anychar).map(|labels| [
            Card { rank: *labels.get(0).unwrap() },
            Card { rank: *labels.get(1).unwrap() },
            Card { rank: *labels.get(2).unwrap() },
            Card { rank: *labels.get(3).unwrap() },
            Card { rank: *labels.get(4).unwrap() },
        ]),
        space1,
        character::complete::u32,
    )(input)?;

    Ok((
        input,
        Hand {
            cards,
            bid,
        }
    ))
}

/// ------
/// Part 2
/// ------

pub fn part_two_test() {
    let input = InputSnake::new("test_input");
    let hands: Vec<Hand> = input.nom_snake(parse_hand)
        .map(|mut output| output.parse())
        .sorted()
        .collect();

    for hand in hands.iter() {
        info!("{:?} {:?}", hand, hand.score());
    }

    let winnings = hands.iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (i as u32 + 1) * hand.bid);

    info!("{:?}", winnings);
}

pub fn part_two() {
    let input = InputSnake::new("input");
    let hands: Vec<Hand> = input.nom_snake(parse_hand)
        .map(|mut output| output.parse())
        .sorted()
        .collect();

    let winnings = hands.iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (i as u32 + 1) * hand.bid);

    info!("{:?}", winnings);
}
