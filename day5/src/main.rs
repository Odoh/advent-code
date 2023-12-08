use log::{debug, info, warn, error};
use itertools::Itertools;
use env_logger;
use nom::bytes::complete::tag;
use nom::character::complete::{space1, newline};
use nom::multi::{separated_list0, separated_list1, many0, fold_many1};
use nom::sequence::{terminated, preceded, pair, separated_pair};
use regex::CaptureMatches;
use once_cell::sync::Lazy;

use std::boxed;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::ops::Range;

use advent::{InputSnake, FromRegex};
use nom::{self, character, IResult, Parser};

#[derive(Debug, Clone)]
struct AlmanacMaps {
    maps: Vec<AlmanacMap>,
}

#[derive(Debug, Clone, Copy)]
struct AlmanacMap {
    destination_range_start: u64,
    source_range_start: u64,
    range_len: u64,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    almanac_maps: Vec<AlmanacMaps>,
}

impl AlmanacMaps {
    pub fn get(&self, source: u64) -> u64 {
        for map in self.maps.iter() {
            if map.in_range(source) {
                return map.get(source);
            }
        }

        return source;
    }
}

impl AlmanacMap {
    pub fn in_range(&self, source: u64) -> bool {
        self.source_range_start <= source && source < self.source_range_start + self.range_len
    }

    pub fn get(&self, source: u64) -> u64 {
        if self.source_range_start <= source && source < self.source_range_start + self.range_len {
            let diff = source - self.source_range_start;
            return self.destination_range_start + diff;
        }

        panic!("Must be in range");
    }
}

const ALMANAC_MAP_NAMES: [&'static str; 7] = [
    "seed-to-soil map:",
    "soil-to-fertilizer map:",
    "fertilizer-to-water map:",
    "water-to-light map:",
    "light-to-temperature map:",
    "temperature-to-humidity map:",
    "humidity-to-location map:",
];

/// ------
/// Part 1
/// ------

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    // seeds: 79 14 55 13
    let (input, seeds) = preceded(
        tag("seeds: "),
        separated_list1(
            space1,
            nom::character::complete::u64
        )
    )(input)?;

    let mut map_input = input;
    let mut almanac_maps: Vec<AlmanacMaps> = Vec::new();
    for almanac_map_name in ALMANAC_MAP_NAMES {
        let (input, _) = many0(newline)(map_input)?;
        let (input, _) = pair(
            tag(almanac_map_name),
            newline
        )(input)?;
        let (input, maps) = separated_list1(
            newline,
            separated_list1(
                space1,
                nom::character::complete::u64,
            ).map(|values| AlmanacMap {
                destination_range_start: *values.get(0).unwrap(),
                source_range_start: *values.get(1).unwrap(),
                range_len: *values.get(2).unwrap(),
            })
        )(input)?;

        map_input = input;
        almanac_maps.push(AlmanacMaps { maps });
    }

    Ok((
        map_input,
        Almanac {
            seeds,
            almanac_maps,
        }
    ))
}

fn part_one_test() {
    let input = InputSnake::new("test_input");
    let no_snake = input.no_snake();
    let (_, almanac) = parse_almanac(&no_snake).unwrap();
    // dbg!(almanac);
    
    let closest_location = almanac.seeds.into_iter()
        .map(|seed| almanac.almanac_maps.iter()
            .fold(seed, |acc, maps| maps.get(acc)))
        .min()
        .unwrap();

    info!("{:?}", closest_location);
}

fn part_one() {
    let input = InputSnake::new("input");
    let no_snake = input.no_snake();
    let (_, almanac) = parse_almanac(&no_snake).unwrap();
    
    let closest_location = almanac.seeds.into_iter()
        .map(|seed| almanac.almanac_maps.iter()
            .fold(seed, |acc, maps| maps.get(acc)))
        .min()
        .unwrap();

    info!("{:?}", closest_location);
}

/// ------
/// Part 2
/// ------

#[derive(Debug, Clone, Copy)]
struct Seeds {
    range_start: u64,
    range_len: u64,
}

#[derive(Debug)]
struct AlmanacTwo {
    seeds: Vec<Seeds>,
    almanac_maps: Vec<AlmanacMaps>,
}

impl Seeds {
    pub fn seeds(&self) -> std::ops::Range<u64> {
        self.range_start..(self.range_start + self.range_len)
    }
}

impl AlmanacTwo {
    pub fn all_seeds(&self) -> Box<dyn Iterator<Item=u64>> {
        Box::new(self.seeds.clone().into_iter()
            .flat_map(|seeds| seeds.seeds()))
    }
}

fn parse_almanac_two(input: &str) -> IResult<&str, AlmanacTwo> {
    // seeds: 79 14 55 13
    let (input, seeds) = preceded(
        tag("seeds: "),
        separated_list1(
            space1,
            separated_pair(
                nom::character::complete::u64,
                space1,
                nom::character::complete::u64
            ).map(|(range_start, range_len)| Seeds {
                range_start,
                range_len
            })
        )
    )(input)?;

    let mut map_input = input;
    let mut almanac_maps: Vec<AlmanacMaps> = Vec::new();
    for almanac_map_name in ALMANAC_MAP_NAMES {
        let (input, _) = many0(newline)(map_input)?;
        let (input, _) = pair(
            tag(almanac_map_name),
            newline
        )(input)?;
        let (input, maps) = separated_list1(
            newline,
            separated_list1(
                space1,
                nom::character::complete::u64,
            ).map(|values| AlmanacMap {
                destination_range_start: *values.get(0).unwrap(),
                source_range_start: *values.get(1).unwrap(),
                range_len: *values.get(2).unwrap(),
            })
        )(input)?;

        map_input = input;
        almanac_maps.push(AlmanacMaps { maps });
    }

    Ok((
        map_input,
        AlmanacTwo {
            seeds,
            almanac_maps,
        }
    ))
}

#[derive(Debug)]
struct SeedMap {
    src: (u64, u64),
    dst: (u64, u64),
}

impl SeedMap {
    fn get_dst(&self, src: u64) -> u64 {
        debug_assert!(src >= self.src.0 && src <= self.src.1);
        let offset = src - self.src.0;
        self.dst.0 + offset
    }
}

fn solve(almanac: AlmanacTwo) -> u64 {
    // use inclusive ranges
    let mut seeds: HashSet<(u64, u64)> = almanac.seeds.into_iter()
        .map(|seed| (seed.range_start, seed.range_start + seed.range_len - 1))
        .collect();
    // dbg!(seeds);

    let almanac_maps: Vec<Vec<SeedMap>> = almanac.almanac_maps.iter()
        .map(|almanac_map| almanac_map.maps.iter()
            .map(|map| SeedMap {
                src: (map.source_range_start, map.source_range_start + map.range_len - 1),
                dst: (map.destination_range_start, map.destination_range_start + map.range_len - 1),
            })
            .collect()
        )
        .collect();
    // dbg!(almanac_maps);

    // iterate over the almanc maps, in order, updating the ranges of seeds as they are applied
    for (i, almanac_map) in almanac_maps.iter().enumerate() {
        let mut mapped_seeds = HashSet::new();
        'seedloop: while !seeds.is_empty() {
            let seed = *seeds.iter().next().unwrap();
            seeds.take(&seed);

            'maploop: for map in almanac_map.iter() {
                // the mapped seeds depend on an overlap in seed and seed map ranges,
                // of which there are several cases:
                //

                // 1. complete overlap of map
                //
                // seed |-----------|  ->   |--|=======|--| mapped_seeds
                // map     |=====|
                //
                let is_complete_overlap_of_map = seed.0 < map.src.0
                    && seed.1 > map.src.1;

                if is_complete_overlap_of_map {
                    seeds.insert((seed.0, map.src.0 - 1));
                    mapped_seeds.insert((map.get_dst(map.src.0), map.get_dst(map.src.1)));
                    seeds.insert((map.src.1 + 1, seed.1));
                    debug!("is_complete_map");
                    continue 'seedloop;
                }

                // 2. complete overlap of seed
                //
                // seed    |-----|     ->   |=====| mapped_seeds
                // map  |===========|
                //
                let is_complete_overlap_of_seed = map.src.0 <= seed.0
                    && map.src.1 >= seed.1;

                if is_complete_overlap_of_seed {
                    mapped_seeds.insert((map.get_dst(seed.0), map.get_dst(seed.1)));
                    debug!("is_complete_seed: {:?}", (map.get_dst(seed.0), map.get_dst(seed.1)));
                    continue 'seedloop;
                }

                //
                // 3. left overlap
                //
                // seed |-----|        ->   |---|==| mapped_seeds
                // map      |=====|
                //
                let is_left_overlap = seed.0 < map.src.0
                    && seed.1 >= map.src.0
                    && seed.1 <= map.src.1;

                if is_left_overlap {
                    seeds.insert((seed.0, map.src.0 - 1));
                    mapped_seeds.insert((map.get_dst(map.src.0), map.get_dst(seed.1)));
                    debug!("is_left");
                    continue 'seedloop;
                }

                // 4. right overlap
                //
                // seed    |-----|     ->   |==|---| mapped_seeds
                // map  |=====|
                //
                let is_right_overlap = seed.0 >= map.src.0
                    && seed.0 <= map.src.1
                    && seed.1 > map.src.1;

                if is_right_overlap {
                    mapped_seeds.insert((map.get_dst(seed.0), map.get_dst(map.src.1)));
                    seeds.insert((map.src.1 + 1, seed.1));
                    debug!("is_right");
                    continue 'seedloop;
                }

                // 5. no overlap (only add to mapped once all mappings have been applied)
                //
                // seed         |-----|     ->   |-----| mapped_seeds
                // map  |=====|
                //
                debug!("no_overlap");
            }
            // fall-through case in which none of the mappings were applied
            mapped_seeds.insert((seed.0, seed.1));
        }

        debug!("almanc_map: {:?}", almanac_map);
        debug!("seeds: {:?}", seeds);
        debug!("mapped_seeds: {:?}", mapped_seeds);
        seeds = mapped_seeds;
    }

    // return the lowest "seed" (which is now a location)
    info!("{:?}", seeds);
    seeds.iter()
        .map(|&(start, _end)| start)
        .min()
        .unwrap()
}


fn part_two_test() {
    let input = InputSnake::new("test_input");
    let no_snake = input.no_snake();
    let (_, almanac) = parse_almanac_two(&no_snake).unwrap();
    
    // let closest_location = almanac.all_seeds().into_iter()
    //     .map(|seed| almanac.almanac_maps.iter()
    //         .fold(seed, |acc, maps| maps.get(acc)))
    //     .min()
    //     .unwrap();

    let closest_location = solve(almanac);

    info!("{:?}", closest_location);
}

fn part_two() {
    let input = InputSnake::new("input");
    let no_snake = input.no_snake();
    let (_, almanac) = parse_almanac_two(&no_snake).unwrap();

    // let all_seeds = almanac.all_seeds().into_iter().count();
    // info!("all_seeds: {}", all_seeds);
    
    // let closest_location = almanac.all_seeds().into_iter()
    //     .enumerate()
    //     .inspect(|(i, _seed)| if i % 10000 == 0 { info!("{}%", ((*i as f32) / 1638141121.0) * 100.0) })
    //     .map(|(_i, seed)| almanac.almanac_maps.iter()
    //         .fold(seed, |acc, maps| maps.get(acc)))
    //     .min()
    //     .unwrap();

    let closest_location = solve(almanac);

    info!("{:?}", closest_location);
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));

    // info!("Part One Test");
    // part_one_test();
    // info!("Part One");
    // part_one();
 
    info!("Part Two Test");
    part_two_test();
    info!("Part Two");
    part_two();
}
