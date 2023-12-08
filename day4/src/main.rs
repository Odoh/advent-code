use log::{debug, info, warn, error};
use itertools::Itertools;
use env_logger;
use nom::character::complete::{digit1, space1};
use nom::character::is_space;
use nom::multi::{many1, many_till, separated_list1};
use regex::CaptureMatches;
use once_cell::sync::Lazy;
use nom::{bytes::complete::*, combinator::*, error::*, sequence::*, IResult, Parser};

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Range;

use advent::{InputSnake, FromRegex};

#[derive(Debug)]
struct Card {
    id: u32,
    instances: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = take_while(|c: char| c.is_whitespace())(input)?;
    let (input, id) = nom::character::complete::u32(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = take_while(|c: char| c.is_whitespace())(input)?;
    let (input, winning_numbers) = separated_list1(
        nom::character::complete::space1,
        nom::character::complete::u32,
    )(input)?;
    let (input, _) = tag(" |")(input)?;
    let (input, _) = take_while(|c: char| c.is_whitespace())(input)?;
    let (input, numbers) = separated_list1(
        nom::character::complete::space1,
        nom::character::complete::u32,
    )(input)?;

    Ok((input, Card {
        id,
        instances: 1,
        winning_numbers,
        numbers,
    }))
}

fn card_points(card: &Card) -> u32 {
    let num_overlap = card.winning_numbers.iter()
        .filter(|num| card.numbers.contains(num))
        .count();
    if num_overlap == 0 {
        return 0;
    }

    let base: u32 = 2;
    let points = base.pow((num_overlap as u32) - 1);
    debug!("card {:?} points {}", card, points);

    points
}

fn all_card_points(cards: &Vec<Card>) -> u32 {
    cards.iter()
        .fold(0, |acc, card| acc + card_points(card) * card.instances)
}

fn generate_copies(cards: &mut Vec<Card>) {
    for i in (Range { start: 0, end: cards.len() }) {
        let card = cards.get(i).unwrap();
        let instances = card.instances;
        let num_overlap = card.winning_numbers.iter()
            .filter(|num| card.numbers.contains(num))
            .count();

        for _ in (Range { start: 0, end: card.instances }) {
            for j in (Range { start: i + 1, end: (i + 1) + num_overlap }) {
                let copied_card = cards.get_mut(j).map(|card: &mut Card| card.instances += 1);
            }
        }
    }
}

// Hacking away at rust traits

pub struct NomOutput<O> {
    pub line: String,
    pub parser: Box<dyn for<'a> nom::Parser<&'a str, O, nom::error::Error<&'a str>>>,
}

impl <O> NomOutput<O> {
   pub fn parse(&mut self) -> O {
        self.parser.parse(&self.line).unwrap().1
   }
}

pub struct NomOutputGeneric<P, O> {
    pub line: String,
    pub parser: P,
    _output: PhantomData<O>,
}

impl <P, O> NomOutputGeneric<P, O> 
    where
        P: for<'a> nom::Parser<&'a str, O, nom::error::Error<&'a str>> {

   pub fn parse(&mut self) -> O {
        self.parser.parse(&self.line).unwrap().1
   }
}

/// ------
/// Part 1
/// ------

fn part_one_test() {
    let input = InputSnake::new("test_input");
    let cards: Vec<Card> = input.nom_snake(parse_card)
        .map(|mut n| n.parse())
        .collect();
    let points = all_card_points(&cards);
    info!("{:?}", points);
}

fn part_one() {
    let input = InputSnake::new("input");
    let cards: Vec<Card> = input.nom_snake(parse_card)
        .map(|mut n| n.parse())
        .collect();
    let points = all_card_points(&cards);
    info!("{:?}", points);
}

/// ------
/// Part 2
/// ------

fn part_two_test() {
    let input = InputSnake::new("test_input");
    let mut cards: Vec<Card> = input.nom_snake(parse_card)
        .map(|mut n| n.parse())
        .collect();
    generate_copies(&mut cards);
    let total_cards = cards.iter().fold(0, |acc, card| acc + card.instances);
    info!("{:?}", total_cards);
}

fn part_two() {
    let input = InputSnake::new("input");
    let mut cards: Vec<Card> = input.nom_snake(parse_card)
        .map(|mut n| n.parse())
        .collect();
    generate_copies(&mut cards);
    let total_cards = cards.iter().fold(0, |acc, card| acc + card.instances);
    info!("{:?}", total_cards);

    let nom_output = NomOutput {
        line: String::new(),
        parser: Box::new(parse_card),
    };

    let nom_output_generic = NomOutputGeneric {
        line: String::new(),
        parser: parse_card,
        _output: Default::default(),
    };
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));

    info!("Part One Test");
    part_one_test();
    info!("Part One");
    part_one();
 
    info!("Part Two Test");
    part_two_test();
    info!("Part Two");
    part_two();

    // let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_owned();
    // let mut nom_output = NomOutputGeneric {
    //     line,
    //     parser: parse_card,
    //     _output: Default::default(),
    // };
    // let card = nom_output.parse();
    let card = parse_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
    info!("{:?}", card);
}
