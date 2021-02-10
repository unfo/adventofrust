#![allow(unused_imports)]
// #![allow(dead_code)]

const INPUT: &str = include_str!("../input");
// const INPUT: &str = include_str!("../input");

extern crate adventofcode;
#[allow(unused_imports)]
use adventofcode::{type_of};
#[allow(unused_imports)]
use anyhow::{anyhow, Result as AnyhowResult, Error as AnyhowError};
#[allow(unused_imports)]
use itertools::{Itertools, enumerate};
type Input = Vec<u64>;

#[allow(clippy::unnecessary_wraps)]
fn parse_input() -> AnyhowResult<Input> {
    Ok(INPUT
        .lines()
        .map(|l| l.to_owned())
        .map(|s| u64::from_str_radix(&s, 10))
        .filter(|x| !x.is_err())
        .map(|x|x.unwrap())
        .unique()
        .collect())
}

#[test]
fn test_input() {
    let input = parse_input().unwrap();

    println!("let input: {} = {:?}", type_of(&input), &input);
}

fn main() -> AnyhowResult<()> {
    let items = parse_input()?;

    println!("{:#?}", items);

    Ok(())
}
