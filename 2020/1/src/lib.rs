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
        .map(|l| l.parse().unwrap())
        .collect())
}

pub fn part1(_input: &Input) -> InputNum {
    let input = _input.into_iter();
    let totals:Option<InputNum> = _input.into_iter()
        .cartesian_product(input)
        .find(|p| p.0 + p.1 == 2020)
        .map(|(a,b)| a * b);
    totals.unwrap()
}


pub fn part2(_input: &Input) -> usize {
    let input = _input.into_iter();
    let input_slice = &_input[..];
    let chosen:Option<InputNum> = _input.into_iter()
        .cartesian_product(input)
        .filter(|p| p.0 + p.1 < 2020)
        .find(|p| input_slice.contains(&(2020 - p.0 - p.1)))
        .map(|p| p.0 * p.1 * (2020 - p.0 - p.1));
    chosen.unwrap_or_default()
}