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

fn day4(part: Part) {
    let input = include_str!("day4_input.txt");

    // required field name and the corresponding validator function. Returns true, if field value is valid.
    let required_fields: Vec<(&str, fn(&str) -> bool)> = vec![
        ("byr", |val| {
            val.len() == 4
                && val
                    .parse::<u16>()
                    .map_or(false, |num| (1920..=2002).contains(&num))
        }),
        ("iyr", |val| {
            val.len() == 4
                && val
                    .parse::<u16>()
                    .map_or(false, |num| (2010..=2020).contains(&num))
        }),
        ("eyr", |val| {
            val.len() == 4
                && val
                    .parse::<u16>()
                    .map_or(false, |num| (2020..=2030).contains(&num))
        }),
        ("hgt", |val| {
            let (num, unit) = val.split_at(val.len().saturating_sub(2));
            num.parse::<u16>().map_or(false, |num| match unit {
                "in" => (59..=76).contains(&num),
                "cm" => (150..=193).contains(&num),
                _ => false,
            })
        }),
        ("hcl", |val| {
            val.len() == 7
                && &val[0..1] == "#"
                && val[1..]
                    .bytes()
                    .all(|ch| (b'0'..=b'9').contains(&ch) || (b'a'..=b'f').contains(&ch))
        }),
        ("ecl", |val| {
            let valid_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            valid_colors.contains(&val)
        }),
        ("pid", |val| {
            val.len() == 9 && val.chars().all(|ch| ch.is_digit(10))
        }), /*"cid"*/
    ];
    // quick and dirty parsing
    let n_valid = input
        .split("\n\n")
        .map(|passport| {
            passport
                .split_whitespace()
                .map(|entry| {
                    let mut iter = entry.split(":");
                    let (key, value) = iter.next_tuple().unwrap();
                    assert!(iter.next().is_none());
                    (key, value)
                })
                .collect::<HashMap<_, _>>()
        })
        .filter(|entries| {
            dbg!(&entries);
            required_fields
                .iter()
                .all(|(required_field, validator)| match part {
                    Part::One => entries.contains_key(required_field),
                    Part::Two => {
                        let value = entries.get(required_field);
                        value.copied().map_or(false, validator)
                    }
                })
        })
        .count();

    println!("{}", n_valid);
}

fn day5(part: Part) {
    let input = include_str!("day5_input.txt");
    // this is just a binary number with different letters.
    // the seats are numbered left to right, front to back
    let seat_ids = input
        .replace("F", "0")
        .replace("B", "1")
        .replace("L", "0")
        .replace("R", "1")
        .lines()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .sorted()
        .collect::<Vec<_>>();

    match part {
        Part::One => {
            let max_seat_id = seat_ids.last().unwrap();
            println!("{}", max_seat_id);
        }
        Part::Two => {
            let my_seat = seat_ids
                .windows(2)
                .find_map(|ids| match ids {
                    &[first, next] if next == first + 2 => Some(first + 1),
                    _ => None,
                })
                .unwrap();
            println!("{}", my_seat);
        }
    }
}

fn day6(part: Part) {
    let input = include_str!("day6_input.txt");
    // vec of questions that anyone in the group answered yes to
    let group_answers = input
        .split("\n\n")
        .map(|group| {
            // bitmask
            let answer_masks = group.lines().map(|answers| {
                answers
                    .bytes()
                    .map(|byte| byte - b'a')
                    .fold(0u32, |mask, answer| mask | 1 << answer)
            });

            match part {
                Part::One => answer_masks.fold(0, std::ops::BitOr::bitor),
                Part::Two => answer_masks.fold(!0, std::ops::BitAnd::bitand),
            }
        })
        .collect::<Vec<_>>();

    let solution = group_answers
        .iter()
        .map(|answers| answers.count_ones())
        .sum::<u32>();
    println!("{}", solution);
}

fn day7(part: Part) {
    let input = include_str!("day7_input.txt");
    let pattern = regex::Regex::new(r"([a-z ]+) bags contain (.*).").unwrap();
    let contained_pattern = regex::Regex::new(r"(\d+) ([a-z ]+) bags?").unwrap();
    let rules = input
        .lines()
        .map(|line| {
            let captures = pattern.captures(line).unwrap();
            let containing_color = captures.get(1).unwrap().as_str();
            let contained_color_list = captures.get(2).unwrap().as_str();
            let contained_colors = match contained_color_list == "no other bags" {
                // this match is technically not necessary,
                // because the captures iter would just fail to find any matches
                // and the vec would end up empty anyway
                true => vec![],
                false => contained_pattern
                    .captures_iter(contained_color_list)
                    .map(|contained_bags| {
                        let number = contained_bags
                            .get(1)
                            .unwrap()
                            .as_str()
                            .parse::<u32>()
                            .unwrap();
                        let color = contained_bags.get(2).unwrap().as_str();
                        (number, color)
                    })
                    .collect::<Vec<_>>(),
            };
            // sanity check, make sure we got everything by reconstructing original string
            debug_assert!({
                let reconstructed_contained_color_list = match contained_colors.is_empty() {
                    true => "no other bags".to_string(),
                    false => contained_colors
                        .iter()
                        .map(|&(num, color)| {
                            format!("{} {} bag{}", num, color, if num == 1 { "" } else { "s" })
                        })
                        .join(", "),
                };
                line == &format!(
                    "{} bags contain {}.",
                    containing_color, reconstructed_contained_color_list
                )
            });

            (containing_color, contained_colors)
        })
        .collect::<HashMap<_, _>>();

    // invert the mapping
    let mut containable_in = HashMap::new();
    for (&color, contained_colors) in rules.iter() {
        for &(_num, contained_color) in contained_colors {
            containable_in
                .entry(contained_color)
                .or_insert(HashSet::new())
                .insert(color);
        }
    }

    let target_color = "shiny gold";
    let mut potential_container_colors = HashSet::new();
    let mut visited_container_colors = HashSet::new();

    fn dfs<'a>(
        potential_color: &'a str,
        containable_in: &HashMap<&'a str, HashSet<&'a str>>,
        potential_container_colors: &mut HashSet<&'a str>,
        visited: &mut HashSet<&'a str>,
    ) {
        let potential_containers = match containable_in.get(potential_color) {
            Some(containers) => containers,
            None => return,
        };
        for &potential_container_color in potential_containers {
            potential_container_colors.insert(potential_container_color);
            if visited.insert(&potential_container_color) {
                dfs(
                    potential_container_color,
                    containable_in,
                    potential_container_colors,
                    visited,
                );
            }
        }
    }

    dfs(
        target_color,
        &containable_in,
        &mut potential_container_colors,
        &mut visited_container_colors,
    );

    println!("{}", potential_container_colors.len());
}

fn main() {
    // keep solutions for old days here to avoid unused code warnings
    if false {
        day1(Part::One);
        day1(Part::Two);
        day2(Part::One);
        day2(Part::Two);
        day3(Part::One);
        day3(Part::Two);
        day4(Part::One);
        day4(Part::Two);
        day5(Part::One);
        day5(Part::Two);
        day6(Part::One);
        day6(Part::Two);
    }
    day7(Part::One);
}
