#[macro_use] extern crate lazy_static;

extern crate adventofcode;
#[allow(unused_imports)]
use std::{collections::HashMap, str::FromStr};

#[allow(unused_imports)]
use adventofcode::{type_of};

#[allow(unused_imports)]
use anyhow::{anyhow, Result as AnyhowResult, Error as AnyhowError};

#[allow(unused_imports)]
use itertools::{Itertools, iproduct, Combinations};

use regex::Regex;

pub type InputNum = String;
pub type Input = Vec<InputNum>;
pub type Output = usize;

lazy_static! {
    static ref VALID_LINE: Regex = Regex::new(r"^(?P<minimum>\d+)-(?P<maximum>\d+) (?P<letter>\w): (?P<phrase>\w+)$").unwrap();
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_input(input: &str) -> AnyhowResult<Input> {

    Ok(input
        .lines()
        .filter(|line| VALID_LINE.is_match(line))
        .map(|line| String::from_str(line).unwrap())
        .collect())
}

pub fn part1(_input: &Input) -> Output {
    // let input = _input.into_iter();
    // let totals:Option<InputNum> = _input.into_iter()
    //     .cartesian_product(input)
    //     .find(|p| p.0 + p.1 == 2020)
    //     .map(|(a,b)| a * b);
    // totals.unwrap()
    println!("Total of {:?} lines", _input.len());
    let mut valid_lines: Output = 0;
    for line in _input.into_iter() {
        let caps = VALID_LINE.captures(line).unwrap();
        let minimum: Output = caps["minimum"].parse().unwrap();
        let maximum: Output = caps["maximum"].parse().unwrap();
        let letter = &caps["letter"].chars().next().unwrap();
        let phrase = &caps["phrase"];

        // println!("Valid line: from {} to {} times {} in {}", minimum, maximum, letter, phrase);

        let lettercount_in_line = phrase.chars()
                                    .filter(|c| c == letter )
                                    .count();
        if minimum <= lettercount_in_line && lettercount_in_line <= maximum {
            valid_lines += 1;
        }                            
    }
    valid_lines
}


pub fn part2(_input: &Input) -> Output {
0
}