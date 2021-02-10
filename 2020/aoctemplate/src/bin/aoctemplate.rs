const EXAMPLE1: &str = include_str!("../../example1");
// const EXAMPLE2: &str = include_str!("../../example2");
const INPUT: &str = include_str!("../../input");

use anyhow::Result as AnyhowResult;

use aoctemplate::*;

fn main() -> AnyhowResult<()> {
    // let INPUTS = [EXAMPLE1, EXAMPLE2, INPUT];
    // const INPUTS: [&str; 2] = [EXAMPLE1, INPUT];
    const INPUTS: [&str; 1] = [EXAMPLE1];

    for input in INPUTS.iter() {
        let items = parse_input(input)?;

        let result1 = part1(&items);
        println!("part1: {}", &result1);
    
        let result2 = part2(&items);
        println!("part2: {}", &result2);    
    }

    Ok(())
}

