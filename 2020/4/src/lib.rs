extern crate adventofcode;

use std::collections::HashMap;

use anyhow::{Result as AnyhowResult};
#[allow(unused_imports)]
use itertools::{Itertools};

pub type InputInner = Passport;
pub type Input = Vec<InputInner>;
pub type Output = usize;

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
#[allow(dead_code)]
pub struct Passport {
    byr: usize,
    iyr: usize,
    eyr: usize,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: usize,
}

fn parse_passport(passport_text: &str) -> Option<Passport> {
    let fields = passport_text.split_ascii_whitespace();
    let dict = HashMap::new();
    let passport = Passport{  };
    for field in fields {
        let (key, value) = field.split_at(3);
        passport = value;
    }
    
    let dummy = Passport {
        byr: 0, iyr: 0, eyr: 0, hgt: String::from("0"), hcl: String::from("0"), ecl: String::from("0"), pid: 0,
    };
    Some(dummy)
}
/**
Each passport is represented as a sequence of key:value pairs separated by spaces or newlines. 
Passports are separated by blank lines.

ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm
**/
#[allow(clippy::unnecessary_wraps)]
pub fn parse_input(input: &str) -> AnyhowResult<Input> {

    let passports:Vec<Passport> = Vec::new();

    let _passports:Vec<Passport> = input
        .split("\n\n")
        .map(|l| parse_passport(l).unwrap())
        .collect();

    Ok(passports)
}

pub fn part1(_input: &Input) -> usize {
0
}

pub fn part2(_input: &Input) -> usize {
0
}