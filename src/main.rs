use std::collections::HashSet;

enum Part {
    One,
    Two,
}

fn day1(part: Part) {
    let input = include_str!("day1_input.txt");
    let nums = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<HashSet<_>>();

    let target = 2020;

    let (num1, num2) = nums
        .iter()
        .find_map(|&num| {
            let matching_num = nums.get(&(target - num))?;
            Some((num, matching_num))
        })
        .unwrap();
    println!("{} * {} = {}", num1, num2, num1 * num2);
}

fn main() {
    day1(Part::One);
}
