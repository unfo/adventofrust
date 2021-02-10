#![allow(unused_imports)]

extern crate adventofcode;
use std::collections::HashMap;

#[allow(unused_imports)]
use adventofcode::{type_of};

#[allow(unused_imports)]
use anyhow::{anyhow, Result as AnyhowResult, Error as AnyhowError};

#[allow(unused_imports)]
use itertools::{Itertools, iproduct, Combinations};

pub type InputNum = usize;
pub type Input = Vec<InputNum>;

#[allow(clippy::unnecessary_wraps)]
pub fn parse_input(input: &str) -> AnyhowResult<Input> {
    Ok(input
        .lines()
        .map(|l| l.to_owned())
        .map(|s| InputNum::from_str_radix(&s, 10))
        .filter(|x| !x.is_err())
        .map(|x|x.unwrap())
        .collect())
}

pub fn part1(_input: &Input) -> usize {
    // let mut input = _input.clone();
0
}


pub fn part2(_input: &Input) -> usize {
    // let mut input = _input.clone();
0
}