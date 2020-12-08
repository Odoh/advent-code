use log::{debug, info, warn, error};
use itertools::Itertools;
use env_logger;
use regex::CaptureMatches;

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use advent::{InputSnake, FromRegex};

#[derive(Debug)]
struct BagAmount {
    bag: String,
    amount: u64,
}

#[derive(Debug)]
struct BagRule {
    bag: String,
    allowed_bags: Vec<BagAmount>
}

struct BagProcessor {
    bag_rules: HashMap<String, Vec<BagAmount>>,
}

impl BagProcessor {
    pub fn new(bag_rules_vec: Vec<BagRule>) -> Self {
        let bag_rules = bag_rules_vec.into_iter()
            .map(|r| (r.bag, r.allowed_bags))
            .collect::<HashMap<String, Vec<BagAmount>>>();
        BagProcessor {
            bag_rules
        }
    }

    pub fn num_bags_can_contain_bag(&self, contained_bag: &str) -> usize {
        let bags = self.bag_rules.keys()
            .filter(|&bag| bag != contained_bag)
            .filter(|&bag| self.can_contain_bag(bag, contained_bag))
            .collect::<Vec<&String>>();
        bags.iter().for_each(|b| debug!("{:?}", b));
        bags.len()
    }

    pub fn num_bags_required_inside(&self, bag: &str) -> usize {
        let allowed_bags = self.bag_rules.get(bag).unwrap();
        if allowed_bags.is_empty() {
            return 0;
        }

        allowed_bags.iter()
            .map(|b| b.amount as usize * (1 + self.num_bags_required_inside(&b.bag)))
            .sum()
    }

    fn can_contain_bag(&self, bag: &str, contained_bag: &str) -> bool {
        if bag == contained_bag {
            return true;
        }

        let allowed_bags = self.bag_rules.get(bag).unwrap();
        allowed_bags.iter().any(|b| self.can_contain_bag(&b.bag, contained_bag))
    }
}

const BAG_RULE_REGEX: &str = r"(\d*\s*\w+ \w+ bags*)+";
impl FromRegex for BagRule {
    fn from(mut capture_matches: CaptureMatches) -> Self {
        let bag = capture_matches.next().unwrap().get(1).unwrap()
            .as_str().replace(" bags", "").to_string();

        let allowed_bags = capture_matches
            .map(|c| c.get(1).unwrap().as_str()
                .replace(" bags", "").replace(" bag", ""))
            .filter(|s| !s.contains("no other"))
            .map(|s| {
                let mut split = s.splitn(2, ' ');
                let amount = split.next().unwrap().parse::<u64>().unwrap();
                let bag = split.next().unwrap().to_string();
                BagAmount {
                    bag,
                    amount,
                }
            })
            .collect();
        BagRule {
            bag,
            allowed_bags,
        }
    }
}

fn part_one() {
    let bag_rules = InputSnake::new("input")
        .regex_snake::<BagRule>(BAG_RULE_REGEX)
        .collect::<Vec<BagRule>>();

    bag_rules.iter().for_each(|b| debug!("{:?}", b));

    let bag_processor = BagProcessor::new(bag_rules);
    let count = bag_processor.num_bags_can_contain_bag("shiny gold");

    info!("Part One: {:?}", count);
}

fn part_two() {
    let bag_rules = InputSnake::new("input")
        .regex_snake::<BagRule>(BAG_RULE_REGEX)
        .collect::<Vec<BagRule>>();

    bag_rules.iter().for_each(|b| debug!("{:?}", b));

    let bag_processor = BagProcessor::new(bag_rules);
    let count = bag_processor.num_bags_required_inside("shiny gold");
    info!("Part Two: {:?}", count);
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));
    part_one();
    part_two();
}
