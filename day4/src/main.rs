use std::{cell::Cell, ops::{Range, RangeInclusive}};
use log::{debug, info, warn, error};
use env_logger;
use regex::{self, Regex};

use std::fmt::Debug;

use advent::{InputSnake, FromRegex};

#[derive(Debug)]
struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

struct PassportParser {
    passports: Vec<Passport>,
    passport: Option<Passport>,
}

struct PassportValidator {
    birth_year_regex: Regex,
    issue_year_regex: Regex,
    expiration_year_regex: Regex,
    height_regex: Regex,
    hair_color_regex: Regex,
    eye_color_regex: Regex,
    passport_id_regex: Regex,
}

impl Passport {
    pub fn new() -> Self {
        Passport {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None,
        }
    }
}

impl PassportValidator {
    const VALID_EYE_COLORS: [&'static str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    pub fn new() -> Self {
        PassportValidator {
            birth_year_regex: Regex::new(r"\d{4}").unwrap(),
            issue_year_regex: Regex::new(r"\d{4}").unwrap(),
            expiration_year_regex: Regex::new(r"\d{4}").unwrap(),
            height_regex: Regex::new(r"(\d{3}cm)|(\d{2}in)").unwrap(),
            hair_color_regex: Regex::new(r"#[0-9a-f]{6}").unwrap(),
            eye_color_regex: Regex::new(r"\w{3}").unwrap(),
            passport_id_regex: Regex::new(r"\d{9}").unwrap(),
        }
    }

    /// Whether this passport is valid: contains values for each field (excluding country_id)
    pub fn is_valid_part1(&self, passport: &Passport) -> bool {
        vec![
            &passport.birth_year,
            &passport.issue_year,
            &passport.expiration_year,
            &passport.height,
            &passport.hair_color,
            &passport.eye_color,
            &passport.passport_id,
            // country_id doesn't need to exist for a passport to be valid
        ].iter().all(|o| o.is_some())
    }

    /// Whether this passport is valid: contains values for each field (excluding country_id)
    /// plus some additional field specific validation
    pub fn is_valid_part2(&self, passport: &Passport) -> bool {
        vec![
            passport.birth_year.as_ref().map_or(false, |birth_year| self.is_valid_birth_year(&birth_year)),
            passport.issue_year.as_ref().map_or(false, |issue_year| self.is_valid_issue_year(&issue_year)),
            passport.expiration_year.as_ref().map_or(false, |expiration_year| self.is_valid_expiration_year(&expiration_year)),
            passport.height.as_ref().map_or(false, |height| self.is_valid_height(&height)),
            passport.hair_color.as_ref().map_or(false, |hair_color| self.is_valid_hair_color(&hair_color)),
            passport.eye_color.as_ref().map_or(false, |eye_color| self.is_valid_eye_color(&eye_color)),
            passport.passport_id.as_ref().map_or(false, |passport_id| self.is_valid_passport_id(&passport_id)),
            // country_id doesn't need to exist for a passport to be valid
        ].iter().all(|&b| b == true)
    }

    fn is_valid_birth_year(&self, birth_year: &str) -> bool {
        PassportValidator::is_valid_year(&self.birth_year_regex, birth_year, 1920..=2002)
    }

    fn is_valid_issue_year(&self, issue_year: &str) -> bool {
        PassportValidator::is_valid_year(&self.issue_year_regex, issue_year, 2010..=2020)
    }

    fn is_valid_expiration_year(&self, expiration_year: &str) -> bool {
        PassportValidator::is_valid_year(&self.expiration_year_regex, expiration_year, 2020..=2030)
    }

    fn is_valid_height(&self, height: &str) -> bool {
        if !self.height_regex.is_match(height) {
            return false;
        }

        let value = &height[..(height.len()-2)].parse::<u64>().unwrap();
        let range = if height.contains("cm") { 150..=193 } else { 59..=76 };
        range.contains(value)
    }

    fn is_valid_hair_color(&self, hair_color: &str) -> bool {
        self.hair_color_regex.is_match(hair_color)
    }

    fn is_valid_eye_color(&self, eye_color: &str) -> bool {
        if !self.eye_color_regex.is_match(eye_color) {
            return false;
        }

        PassportValidator::VALID_EYE_COLORS.contains(&eye_color)
    }

    fn is_valid_passport_id(&self, passport_id: &str) -> bool {
        self.passport_id_regex.is_match(passport_id)
    }

    fn is_valid_year(regex: &Regex, year_text: &str, valid_range: RangeInclusive<u64>) -> bool {
        if !regex.is_match(year_text) {
            return false;
        }
        let year = year_text.parse::<u64>().unwrap();
        valid_range.contains(&year)
    }
}

impl PassportParser {
    const KEY_VAL_DELIM: char = ':';
    const FIELD_DELIM: char = ' ';

    pub fn new() -> Self {
        PassportParser {
            passports: Vec::new(),
            passport: Some(Passport::new()),
        }
    }

    /// Parse a line of the passport scanner input.
    pub fn parse_line(&mut self, line: &str) {
        // an empty line denotes the current passport data is complete
        if line.is_empty() {
            let passport = self.passport.take();
            self.passport = Some(Passport::new());
            debug!("Saving passport: {:?}", passport);

            passport.map(|p| self.passports.push(p));
            return;
        }

        // apply the fields in the line to the current passport
        line.split(PassportParser::FIELD_DELIM)
            .for_each(|field|
                PassportParser::apply_field(self.passport.as_mut().unwrap(), field));
    }

    /// Parse and apply the given field to a passport.
    pub fn apply_field(passport: &mut Passport, field: &str) {
        let mut items = field.split(PassportParser::KEY_VAL_DELIM);
        let key: &str = items.next().unwrap();
        let value: &str = items.next().unwrap();

        let field: &mut Option<String> = match key {
            "byr" => &mut passport.birth_year,
            "iyr" => &mut passport.issue_year,
            "eyr" => &mut passport.expiration_year,
            "hgt" => &mut passport.height,
            "hcl" => &mut passport.hair_color,
            "ecl" => &mut passport.eye_color,
            "pid" => &mut passport.passport_id,
            "cid" => &mut passport.country_id,
            _ => panic!("Unhandled key {}", key),
        };
        field.replace(value.to_string());
    }
}

fn part_one() {
    let mut passport_parser = PassportParser::new();
    let passport_validator = PassportValidator::new();
    InputSnake::new("input").snake()
        .for_each(|line| passport_parser.parse_line(&line));

    let num_valid = passport_parser.passports.iter()
        .filter(|p| passport_validator.is_valid_part1(p))
        .count();

    info!("Part One: {:?}", num_valid);
}

fn part_two() {
    let mut passport_parser = PassportParser::new();
    let passport_validator = PassportValidator::new();
    InputSnake::new("input").snake()
        .for_each(|line| passport_parser.parse_line(&line));

    let num_valid = passport_parser.passports.iter()
        .filter(|p| passport_validator.is_valid_part2(p))
        .count();

    info!("Part Two: {:?}", num_valid);
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));
    part_one();
    part_two();
}
