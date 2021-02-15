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
/**
Each line gives the password policy and then the password. 
The password policy indicates the lowest and highest number of times
a given letter must appear for the password to be valid. 

For example, 1-3 a means that the password must contain "a" 
at least 1 time and at most 3 times.
**/
pub fn part1(_input: &Input) -> Output {
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


/**
Each policy actually describes two positions in the password, where 
1 means the first character, 
2 means the second character, and so on. 
(Be careful; Toboggan Corporate Policies have no concept of "index zero"!) 

Exactly one of these positions must contain the given letter.
Other occurrences of the letter are irrelevant for the purposes of 
policy enforcement.
**/
pub fn part2(_input: &Input) -> Output {
    println!("Total of {:?} lines", _input.len());
    let mut valid_lines: Output = 0;
    for line in _input.into_iter() {
        let caps = VALID_LINE.captures(line).unwrap();
        let first: Output = caps["minimum"].parse().unwrap();
        let second: Output = caps["maximum"].parse().unwrap();
        let letter = &caps["letter"].chars().next();
        let phrase = &caps["phrase"];
        let len = phrase.chars().count();
        let first_char = phrase.chars().nth(first + 1);
        let second_char = phrase.chars().nth(second + 1);
        if (first_char == *letter) || (len >= second+1 && second_char == *letter) {
            valid_lines += 1;
        }
    }
    valid_lines
}