/*
Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.

For example, suppose your expense report contained the following:

1721
979
366
299
675
1456
In this list, the two entries that sum to 2020 are 1721 and 299. 
Multiplying them together produces 1721 * 299 = 514579, so the correct answer is 514579.

Of course, your expense report is much larger. 
Find the two entries that sum to 2020; what do you get if you multiply them together?
*/
use std::fs;
use crate::aoc;
pub fn part_one(input_type: aoc::InputData) -> usize {
    let mut dir = "data";
    if let aoc::InputData::Example = input_type {
        dir = "example";
    }
    let filename = format!("{}/day1.txt", dir);
    let data = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let numbers: Vec<usize> = data.split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    for num in numbers.iter() {
        let diff = 2020 - num;
        if numbers.iter().any(|&n| n == diff) {
            return num * diff;
        }
    }
    return 0;
}

/*
--- Part Two ---
The Elves in accounting are thankful for your help; one of them even offers you a starfish coin they had left over from a past vacation. They offer you a second one if you can find three numbers in your expense report that meet the same criteria.

Using the above example again, the three entries that sum to 2020 are 979, 366, and 675. Multiplying them together produces the answer, 241861950.

In your expense report, what is the product of the three entries that sum to 2020?

*/

pub fn part_two(input_type: aoc::InputData) -> usize {
    let mut dir = "data";
    if let aoc::InputData::Example = input_type {
        dir = "example";
    }
    let filename = format!("{}/day1.txt", dir);
    let data = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let numbers: Vec<usize> = data.split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    for num in numbers.iter() {
        let middle_point = 2020 - num;
        for num2 in numbers.iter() {
            if *num2 < middle_point {
                let third = middle_point - num2;
                if numbers.iter().any(|&n| n == third) {
                    return num * num2 * third;
                }
            }
        }
    }

    return 0
}

#[cfg(test)]
mod tests {
    use crate::aoc;
    use crate::day1;

    #[test]
    fn example_one() {
        assert_eq!(day1::part_one(aoc::InputData::Example), 514579);
    }
    #[test]
    fn example_two() {
        assert_eq!(day1::part_two(aoc::InputData::Example), 241861950);
    }
}