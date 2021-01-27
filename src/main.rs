mod day1;
mod aoc;

fn main() {
    println!("Currently doing day 1:");
    println!("Example 1: {}", day1::part_one(aoc::InputData::Example));
    println!("Part    1: {}", day1::part_one(aoc::InputData::Real));
    println!("Example 2: {}", day1::part_two(aoc::InputData::Example));
}
