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
mod part_2;

#[derive(Debug, PartialEq, Eq, Ord)]
struct Card {
    rank: char,
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Hand {
    cards: [Card;5],
    bid: u32,
}

#[derive(Debug, PartialEq)]
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
            'J' => 11,
            'T' => 10,
            '1'..='9' => self.rank.to_digit(10).expect("Valid rank"),
            _ => panic!("Unknown character"),
        }
    }
}

impl Hand {
    fn score(&self) -> Score {
        let count_by_rank = self.cards.iter()
            .fold(BTreeMap::new(), |mut acc, card| {
                *acc.entry(card).or_default() += 1;
                acc
            });

        debug!("{:?}: {:?}", self, count_by_rank);

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
}

// impl Ord for Card {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.numeric_value().cmp(&other.numeric_value())
//     }
// }

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.numeric_value().partial_cmp(&other.numeric_value())
    }
}

// impl Ord for Hand {
//     fn cmp(&self, other: &Self) -> Ordering {
//         let score_ordering = self.score().numeric_value().cmp(&other.score().numeric_value());
//         match score_ordering {
//             Ordering::Equal => {
//                 for i in 0..5 {
//                     let self_card = self.cards.get(i).unwrap();
//                     let other_card = self.cards.get(i).unwrap();
//                     let card_ordering = self_card.numeric_value().cmp(&other_card.numeric_value());
//                     if let Ordering::Equal = card_ordering {
//                         continue;
//                     }
// 
//                     return card_ordering;
//                 }
// 
//                 panic!("Cards should break tie-breaker");
//             },
//             _ => score_ordering,
//         }
//     }
// }

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let score_ordering = self.score().numeric_value().partial_cmp(&other.score().numeric_value());
        match score_ordering {
            Some(Ordering::Equal) => {
                for i in 0..5 {
                    let self_card = self.cards.get(i).unwrap();
                    let other_card = other.cards.get(i).unwrap();
                    let card_ordering = self_card.numeric_value().partial_cmp(&other_card.numeric_value());
                    if let Some(Ordering::Equal) = card_ordering {
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

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.numeric_value().partial_cmp(&other.numeric_value())
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
/// Part 1
/// ------

fn part_one_test() {
    let input = InputSnake::new("test_input");
    let hands: Vec<Hand> = input.nom_snake(parse_hand)
        .map(|mut output| output.parse())
        .sorted()
        .collect();

    debug!("{:?}", hands.iter().map(|h| h.score()).collect_vec());

    let winnings = hands.iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (i as u32 + 1) * hand.bid);

    info!("{:?}", winnings);
}

fn part_one() {
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

fn main() {
    env_logger::init_from_env(env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));

    // info!("Part One Test");
    // part_one_test();
    // info!("Part One");
    // part_one();
 
    info!("Part Two Test");
    part_2::part_two_test();
    info!("Part Two");
    part_2::part_two();
}
