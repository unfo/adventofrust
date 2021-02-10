use anyhow::{Result as AnyhowResult};
use itertools::{Itertools};

pub type InputInner = String;
pub type Input = Vec<InputInner>;

#[allow(clippy::unnecessary_wraps)]
pub fn parse_input(input: &str) -> AnyhowResult<Input> {
    Ok(input
        .lines()
        // .map(|l| l.to_owned())
        .map(|l| l.into())
        .collect())
}

pub fn part1(_input: &Input) -> usize {
0
}

pub fn part2(_input: &Input) -> usize {
0
}