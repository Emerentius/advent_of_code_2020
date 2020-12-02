use itertools::Itertools;
use std::collections::{HashMap, HashSet};

enum Part {
    One,
    Two,
}

fn day1(part: Part) {
    let input = include_str!("day1_input.txt");
    let mut nums = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    nums.sort();
    let nums = nums;

    let target = 2020;

    let matching_nums = match part {
        Part::One => nums
            .iter()
            .find_map(|&num| {
                let matching_num = target - num;
                nums.binary_search(&matching_num).ok()?;
                Some(vec![num, matching_num])
            })
            .unwrap(),
        Part::Two => nums
            .iter()
            .enumerate()
            .flat_map(|(i, &num1)| {
                nums[i + 1..]
                    .iter()
                    .take_while(move |&&num2| num1 + num2 < target)
                    .map(move |&num2| (num1, num2))
            })
            .find_map(|(num1, num2)| {
                let matching_num = target - num1 - num2;
                nums.binary_search(&matching_num).ok()?;
                Some(vec![num1, num2, matching_num])
            })
            .unwrap(),
    };
    println!(
        "{} = {}",
        matching_nums.iter().join(" * "),
        matching_nums.iter().product::<u64>()
    );
}

fn day2(part: Part) {
    let input = include_str!("day2_input.txt");
    let pattern = regex::Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();

    let passwords = input
        .lines()
        .map(|line| {
            let m = pattern.captures(line).unwrap();
            let get = |i| m.get(i).unwrap().as_str();
            let min = get(1).parse::<u64>().unwrap();
            let max = get(2).parse::<u64>().unwrap();
            let constrained_letter = get(3);
            assert!(constrained_letter.len() == 1);
            let constrained_letter = constrained_letter.chars().next().unwrap();
            let password = get(4);

            (min..=max, constrained_letter, password)
        })
        .collect::<Vec<_>>();

    let n_valid_passwords = passwords
        .iter()
        .filter(|(letter_count_range, constrained_letter, password)| {
            let mut letter_counts = HashMap::new();
            for ch in password.chars() {
                *letter_counts.entry(ch).or_insert(0) += 1;
            }
            let letter_count = letter_counts.get(&constrained_letter).copied().unwrap_or(0);
            letter_count_range.contains(&letter_count)
        })
        .count();

    println!("{}", n_valid_passwords);
}

fn main() {
    // keep solutions for old days here to avoid unused code warnings
    if false {
        day1(Part::One);
        day1(Part::Two);
    }
    day2(Part::One);
}
