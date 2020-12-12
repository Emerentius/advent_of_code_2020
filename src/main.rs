use itertools::{iproduct, Itertools};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(PartialEq, Eq)]
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

    match part {
        Part::One => day7_part1(rules),
        Part::Two => day7_part2(rules),
    }
}

fn day7_part1(rules: HashMap<&str, Vec<(u32, &str)>>) {
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

fn day7_part2(rules: HashMap<&str, Vec<(u32, &str)>>) {
    let target_color = "shiny gold";
    let mut n_bags_contained_in_color = HashMap::new();

    fn n_bags_in_bag<'a>(
        bag_color: &'a str,
        rules: &HashMap<&'a str, Vec<(u32, &'a str)>>,
        n_bags_contained_in_color: &mut HashMap<&'a str, u32>,
    ) -> u32 {
        if let Some(&n_bags) = n_bags_contained_in_color.get(bag_color) {
            return n_bags;
        }

        let contained_colors = match rules.get(bag_color) {
            Some(containers) => containers,
            None => return 0, // just the bag itself
        };

        let n_bags_in_color = contained_colors
            .iter()
            .map(|&(num, contained_color)| {
                num * (1 + n_bags_in_bag(contained_color, rules, n_bags_contained_in_color))
            })
            .sum();
        n_bags_contained_in_color.insert(bag_color, n_bags_in_color);
        n_bags_in_color
    }

    let solution = n_bags_in_bag(target_color, &rules, &mut n_bags_contained_in_color);
    println!("{}", solution);
}

fn day8(part: Part) {
    let input = include_str!("day8_input.txt");
    let instructions = input
        .lines()
        .map(|line| {
            let instruction = &line[..3];
            let arg = line[4..].parse::<i32>().unwrap();
            (instruction, arg)
        })
        .collect::<Vec<_>>();

    // Returns the value of the accumulator after the program has either terminated (Ok)
    // or finished the first loop (Err)
    fn analyze_program(instructions: Vec<(&str, i32)>) -> Result<i32, i32> {
        let mut iptr = 0;
        let mut acc = 0;
        let mut previously_executed_instructions = HashSet::new();

        loop {
            if !previously_executed_instructions.insert(iptr) {
                return Err(acc);
            }
            let (instruction, arg) = instructions[iptr as usize];
            match instruction {
                "acc" => {
                    acc += arg;
                    iptr += 1;
                }
                "nop" => iptr += 1,
                "jmp" => iptr += arg,
                _ => unreachable!(),
            }
            if iptr >= instructions.len() as i32 {
                return Ok(acc);
            }
        }
    }

    match part {
        Part::One => println!("{}", analyze_program(instructions).unwrap_err()),
        Part::Two => {
            let solution = instructions
                .iter()
                .enumerate()
                .filter(|(_, &(instr, _))| instr == "jmp" || instr == "nop")
                .find_map(|(idx, _)| {
                    let mut instructions = instructions.clone();
                    instructions[idx].0 = match instructions[idx].0 == "jmp" {
                        true => "nop",
                        false => "jmp",
                    };
                    analyze_program(instructions).ok()
                })
                .unwrap();
            println!("{}", solution);
        }
    }
}

fn day9(part: Part) {
    let input = include_str!("day9_input.txt");
    let nums = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut prev_nums = nums.iter().copied().take(25).collect::<VecDeque<_>>();

    let invalid_num = nums[25..]
        .iter()
        .find(|&&num| {
            let is_sum_of_prev_nums = prev_nums.iter().any(|&prev_num1| {
                let prev_num2 = num - prev_num1;
                // if we were looking at more than just 25 prev nums, keeping a hashset for this check
                // would be worth it
                prev_nums.contains(&prev_num2)
            });
            prev_nums.pop_front();
            prev_nums.push_back(num);
            !is_sum_of_prev_nums
        })
        .unwrap();

    if let Part::One = part {
        println!("{}", invalid_num);
        return;
    }

    // all summands are positive, so making the range larger will always
    // increase the sum
    // => use a sliding window of varying size
    let mut start = 0;
    let mut end = 3;
    let mut sum = nums[start..end].iter().sum::<i64>();
    let range = loop {
        if end > nums.len() {
            unreachable!();
        }
        match sum.cmp(&invalid_num) {
            // add next number to make sum bigger
            std::cmp::Ordering::Less => {
                sum += nums[end];
                end += 1
            }
            std::cmp::Ordering::Equal => break start..end,
            std::cmp::Ordering::Greater => {
                if end - start > 3 {
                    // remove number from the front to make the sum smaller
                    sum -= nums[start];
                    start += 1;
                } else {
                    // the range must have at least 2 numbers
                    // and 2 won't ever suffice by definition of how we found the
                    // invalid num, so it has to be at least 3
                    // => slide window along 1 step
                    sum -= nums[start];
                    sum += nums[end];
                    start += 1;
                    end += 1;
                }
            }
        }
    };
    let nums_in_range = nums[range].iter().copied();
    let min = nums_in_range.clone().min().unwrap();
    let max = nums_in_range.max().unwrap();
    println!("{}", min + max);
}

fn day10(part: Part) {
    let input = include_str!("day10_input.txt");
    let nums = input
        .lines()
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .sorted()
        .collect::<Vec<_>>();

    match part {
        Part::One => {
            let mut diff_counts = HashMap::new();
            diff_counts.insert(nums[0], 1); // from 0 to first adapter
            diff_counts.insert(3, 1); // last step is always 3
            for diff in nums.windows(2).map(|ratings| ratings[1] - ratings[0]) {
                *diff_counts.entry(diff).or_insert(0) += 1;
            }

            println!("{}", diff_counts[&1] * diff_counts[&3]);
        }
        Part::Two => {
            // dynamic programming

            // max adapter is 157 jolt
            let mut n_combinations = [0; 158];
            n_combinations[0] = 1;
            for joltage in nums {
                // `+=` instead of `=` in case there are duplicate adapters
                let joltage = joltage as usize;
                n_combinations[joltage] += n_combinations[joltage.saturating_sub(3)..joltage]
                    .iter()
                    .sum::<u64>();
            }
            println!("{}", n_combinations.last().unwrap());
        }
    }
}

fn day11(part: Part) {
    let input = include_str!("day11_input.txt");
    let (occupied, empty, no_seat) = ('#', 'L', '.');
    let mut seating = input.chars().filter(|&ch| ch != '\n').collect::<Vec<_>>();
    let width = input.lines().next().unwrap().len();
    let height = seating.len() / width;
    debug_assert_eq!(width * height, seating.len());
    let idx = |(row, col)| (row * width + col) as usize;
    loop {
        let next_seating = iproduct!(0..height, 0..width)
            .map(|(row, col)| {
                let prev_seating = seating[idx((row, col))];
                if prev_seating == no_seat {
                    return no_seat;
                }

                let neightbor_seats = iproduct!(-1i32..=1, -1i32..=1)
                    .filter(|&step| step != (0, 0))
                    .filter_map(|(row_step, col_step)| {
                        let mut row = row as i32;
                        let mut col = col as i32;
                        std::iter::from_fn(|| {
                            row = row + row_step;
                            col = col + col_step;
                            Some((row, col))
                        })
                        .take(if part == Part::One { 1 } else { usize::MAX })
                        .take_while(|(row, col)| {
                            (0..height as i32).contains(&row) && (0..width as i32).contains(&col)
                        })
                        .map(|(row, col)| (row as usize, col as usize))
                        .find(|&coords| seating[idx(coords)] != no_seat)
                    });
                let neighbor_count = neightbor_seats
                    .filter(|&coords| seating[idx(coords)] == occupied)
                    .count();

                let unacceptable_threshold = if part == Part::One { 4 } else { 5 };
                if neighbor_count == 0 {
                    occupied
                } else if neighbor_count < unacceptable_threshold {
                    prev_seating
                } else if neighbor_count <= 8 {
                    empty
                } else {
                    unreachable!();
                }
            })
            .collect::<Vec<_>>();

        if seating == next_seating {
            break;
        }

        seating = next_seating;
    }

    println!("{}", seating.iter().filter(|&&ch| ch == occupied).count());
}

fn day12(part: Part) {
    let input = include_str!("day12_input.txt");
    let instructions = input
        .lines()
        .map(|line| (&line[..1], line[1..].parse::<i32>().unwrap()));

    match part {
        Part::One => day12_part1(instructions),
        Part::Two => day12_part2(instructions),
    }
}

fn day12_part1(instructions: impl Iterator<Item = (&'static str, i32)>) {
    let (mut x, mut y) = (0, 0);
    // 0 == north, 1 == east, 2 == south, 3 == west
    let mut heading = 1;
    for (mut instruction, num) in instructions {
        assert!(num >= 0);
        if instruction == "F" {
            instruction = match heading {
                0 => "N",
                1 => "E",
                2 => "S",
                3 => "W",
                _ => unreachable!(),
            }
        }

        match instruction {
            "N" => y += num,
            "S" => y -= num,
            "E" => x += num,
            "W" => x -= num,
            "L" => heading = (heading - num / 90).rem_euclid(4),
            "R" => heading = (heading + num / 90).rem_euclid(4),
            _ => unreachable!(),
        }
    }

    println!("{}", x.abs() + y.abs());
}

fn day12_part2(instructions: impl Iterator<Item = (&'static str, i32)>) {
    use nalgebra::{Matrix2, Vector2};
    let mut pos = Vector2::new(0, 0);
    let mut waypoint = Vector2::new(10, 1);
    let rot_right = Matrix2::new(0, -1, 1, 0);
    let rot_left = Matrix2::new(0, 1, -1, 0);
    let apply_n_times = |matrix, mut vec, n| {
        assert!(n > 0);
        // there doesn't seem to be a matrix.pow() method in nalgebra
        // for integer matrices
        for _ in 0..n {
            vec = matrix * vec;
        }
        vec
    };

    for (instruction, num) in instructions {
        assert!(num >= 0);

        match instruction {
            "N" => waypoint.y += num,
            "S" => waypoint.y -= num,
            "E" => waypoint.x += num,
            "W" => waypoint.x -= num,
            "L" => waypoint = apply_n_times(rot_right, waypoint, num / 90),
            "R" => waypoint = apply_n_times(rot_left, waypoint, num / 90),
            "F" => pos += num * waypoint,
            _ => unreachable!(),
        }
    }

    println!("{}", pos.x.abs() + pos.y.abs());
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
        day7(Part::One);
        day7(Part::Two);
        day8(Part::One);
        day8(Part::Two);
        day9(Part::One);
        day9(Part::Two);
        day10(Part::One);
        day10(Part::Two);
        day11(Part::One);
        day11(Part::Two);
        day12(Part::One);
    }
    day12(Part::Two);
}
