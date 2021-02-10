mod aoc;
mod day1;
mod day2;

fn day1() {
    println!("Currently doing day 1:");
    println!("Example 1: {}", day1::part_one(aoc::InputData::Example));
    println!("Part    1: {}", day1::part_one(aoc::InputData::Real));
    println!("Example 2: {}", day1::part_two(aoc::InputData::Example));
    println!("######################");
}
fn day2() {
    println!("Currently doing day 2:");
    println!("Example 1: {}", day2::part_one(aoc::InputData::Example));
    println!("Part    1: {}", day2::part_one(aoc::InputData::Real));
    println!("Example 2: {}", day2::part_two(aoc::InputData::Example));
    println!("######################");
}

fn main() {
    day1();
    day2();
}
