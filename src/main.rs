use itertools::Itertools;
use std::collections::HashMap;

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
            let num1 = get(1).parse::<u64>().unwrap();
            let num2 = get(2).parse::<u64>().unwrap();
            let constrained_letter = get(3);
            assert!(constrained_letter.len() == 1);
            let constrained_letter = constrained_letter.chars().next().unwrap();
            let password = get(4);

            (num1, num2, constrained_letter, password)
        })
        .collect::<Vec<_>>();

    let n_valid_passwords = match part {
        Part::One => passwords
            .iter()
            .filter(|&&(num1, num2, constrained_letter, password)| {
                let mut letter_counts = HashMap::new();
                for ch in password.chars() {
                    *letter_counts.entry(ch).or_insert(0) += 1;
                }
                let letter_count = letter_counts.get(&constrained_letter).copied().unwrap_or(0);
                (num1..=num2).contains(&letter_count)
            })
            .count(),
        Part::Two => {
            passwords
                .iter()
                .filter(|&&(num1, num2, constrained_letter, password)| {
                    // linear searches instead of O(1) indexing, but performance is irrelevant here
                    let nth_ch = |n| password.chars().nth((n - 1) as usize).unwrap();
                    (nth_ch(num1) == constrained_letter) ^ (nth_ch(num2) == constrained_letter)
                })
                .count()
        }
    };

    println!("{}", n_valid_passwords);
}

fn day3(part: Part) {
    let input = include_str!("day3_input.txt");
    let forest = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| {
                    assert!(ch == '.' || ch == '#');
                    ch == '#'
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    fn trees_encountered_on_slope(
        (speed_x, speed_y): (usize, usize),
        forest: &[Vec<bool>],
    ) -> usize {
        let forest_height = forest.len();
        let forest_width = forest[0].len();

        let positions = std::iter::successors(Some((0, 0)), |&(x, y)| {
            let next_x = (x + speed_x) % forest_width;
            let next_y = y + speed_y;
            match next_y < forest_height {
                true => Some((next_x, next_y)),
                false => None,
            }
        });
        let tree_on_pos = |&(x, y): &(usize, usize)| forest[y][x];
        positions.filter(tree_on_pos).count()
    }

    let solution = match part {
        Part::One => trees_encountered_on_slope((3, 1), &forest),
        Part::Two => {
            let speeds = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
            speeds
                .iter()
                .map(|&speed| trees_encountered_on_slope(speed, &forest))
                .product()
        }
    };
    println!("{}", solution);
}

fn main() {
    // keep solutions for old days here to avoid unused code warnings
    if false {
        day1(Part::One);
        day1(Part::Two);
        day2(Part::One);
        day2(Part::Two);
        day3(Part::One);
    }
    day3(Part::Two);
}
