#[macro_use] extern crate lazy_static;

extern crate adventofcode;

use std::collections::HashMap;

use anyhow::{Result as AnyhowResult};
#[allow(unused_imports)]
use itertools::{Itertools};


pub type Input = Vec<HashMap<String,String>>;
pub type Output = usize;


lazy_static! {
    static ref REQUIRED_FIELDS: [&'static str; 8] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
}

/**
The automatic passport scanners are slow because they're having trouble detecting which passports have all required fields. The expected fields are as follows:

byr (Birth Year)
iyr (Issue Year)
eyr (Expiration Year)
hgt (Height)
hcl (Hair Color)
ecl (Eye Color)
pid (Passport ID)
cid (Country ID)
**/
pub fn validate_passport(_passport: &HashMap<String,String>) -> bool {
    let present_fields: Vec<&String> = _passport.keys().collect();
    let mut required: Vec<&String> = REQUIRED_FIELDS.to_vec().iter().map(|item| String::from(*item)).collect();
    required.retain(|f| ! present_fields.contains(f) );
    println!("Missing fields {:?}", required);
    required.len() == 0
}

fn strs_to_strings(kv: (&str, &str)) -> (String, String) {
    let (key, value) = kv;
    (String::from(key), String::from(value))
}

/**
Each passport is represented as a sequence of key:value pairs separated by spaces or newlines. 
Passports are separated by blank lines.

ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm
**/
#[allow(clippy::unnecessary_wraps)]
pub fn parse_input(input: String) -> AnyhowResult<Vec<HashMap<String,String>>> {
    let passports:Vec<HashMap<String,String>> = input
        .split("\n\n")
        .map(|l| l.split_ascii_whitespace().map(|kv| strs_to_strings(kv.split_at(3))).collect() )
        .collect();

    Ok(passports)
}

pub fn part1(_input: &Vec<HashMap<String,String>>) -> usize {
    _input.into_iter()
    .inspect(|pp| println!(" input> {:?}", pp) )
    .filter(|passport| validate_passport(passport))
    .inspect(|pp| println!(" valid> {:?}", pp) )
    .count()
}

pub fn part2(_input: &Vec<HashMap<String,String>>) -> usize {
0
}