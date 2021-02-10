const EXAMPLE1: &str = include_str!("../../example1");
// const EXAMPLE2: &str = include_str!("../../example2");
const INPUT: &str = include_str!("../../input");


extern crate adventofcode;

#[allow(unused_imports)]
use adventofcode::{type_of};

#[allow(unused_imports)]
use anyhow::{anyhow, Result as AnyhowResult, Error as AnyhowError};

#[allow(unused_imports)]
use itertools::{Itertools, iproduct};

use aoc202001::*;

#[test]
fn test_example() {
    let input = parse_input(EXAMPLE1).unwrap();
    println!("let input: {} = {:?}", type_of(&input), &input);
    assert_eq!(514579, part1(&input));
    assert_eq!(241861950, part2(&input));
    
}

#[test]
fn test_real() {
    let input = parse_input(INPUT).unwrap();
    println!("let input: {} = {:?}", type_of(&input), &input);
    assert_eq!(1014171, part1(&input));
    assert_eq!(46584630, part2(&input));

}


fn main() -> AnyhowResult<()> {
    const INPUTS: [&str; 1] = [EXAMPLE1];
    // const INPUTS: [&str; 2] = [EXAMPLE1, INPUT];
    // const INPUTS: [&str; 3] = [EXAMPLE1, EXAMPLE2, INPUT];

    for (row, input) in INPUTS.iter().enumerate() {
        let items = parse_input(input)?;
        // println!("items: {:?}", items);
        let result1 = part1(&items);
        println!("{} part1: {}", row, &result1);
    
        let result2 = part2(&items);
        println!("{} part2: {}", row, result2);
        println!("----------------------------");    
    }

    Ok(())
}

