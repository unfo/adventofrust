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
    let input = _input.into_iter();
    let mut input2 = _input.into_iter();
    let totals:Vec<Vec<InputNum>> = _input.into_iter()
        .cartesian_product(input)
        // .filter(|p| input2.any(|&a| a + p.0 + p.1 == 2020))
        // .map(|(a,b)| (2020 - (a + b)) * a * b)
        .map(|(a,b)| vec![*a,*b])
        .collect();
    println!("{:?}", &totals);
    let _winrar:Vec<Vec<usize>> = totals.into_iter()
        .filter(|v| input2.any(|&c| c + v[0] + v[1] == 2020))
        .collect::<Vec<Vec<usize>>>();
    // totals[0]
    println!("{:?}", _winrar[0]);
0
}