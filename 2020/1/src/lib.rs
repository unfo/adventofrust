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
    let totals:Vec<InputNum> = _input.into_iter()
    	.cartesian_product(input)
    	.filter(|p| p.0 + p.1 == 2020)
    	.map(|(a,b)| a * b)
    	.collect();
	totals[0]
}


pub fn part2(_input: &Input) -> usize {
    let input = _input.to_vec();
    let inputs:Vec<&Vec<_>> = vec![&input, &input, &input];
    let totals:Vec<InputNum> = inputs.into_iter()
    	.multi_cartesian_product()
    	.filter(|p| p[0] + p[1] + p[2] == 2020)
    	.map(|abc| abc[0] * abc[1] * abc[2])
    	.collect();

	totals[0]

}