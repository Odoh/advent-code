#![allow(dead_code, unused_imports)]

mod day1;
mod day2;
mod day3;
mod day4;

#[macro_use] extern crate itertools;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate maplit;
extern crate regex;

fn main() {
    day4::part1::main();

    println!("\nAdvent of Code - 2018");
}
