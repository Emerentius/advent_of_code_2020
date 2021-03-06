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

fn day13(part: Part) {
    let input = include_str!("day13_input.txt");
    let mut lines = input.lines();
    let min_timestamp = lines.next().unwrap().parse::<i64>().unwrap();
    let bus_times = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|&(_, time)| time != "x")
        .map(|(offset, time)| (offset, time.parse::<i64>().unwrap()))
        .sorted_by_key(|&(_, time)| std::cmp::Reverse(time))
        .collect::<Vec<_>>();

    match part {
        Part::One => {
            let (bus_id, next_arrival) = bus_times
                .iter()
                .map(|&(_, time)| {
                    let next_arrival = (min_timestamp + time - 1) / time * time;
                    (time, next_arrival - min_timestamp)
                })
                .min_by_key(|&(_, next_arrival)| next_arrival)
                .unwrap();

            println!("{}", bus_id * next_arrival);
        }
        Part::Two => {
            // all the ids are pairwise coprime (because they are prime)
            // so we can use the chinese remainder theorem

            // t + offset_0 = 0 (mod bus_period_0)
            // t + offset_1 = 0 (mod bus_period_1)
            // etc...
            // equivalent to
            // t = -offset_i = bus_period_i - offset_i  (mod bus_period_i)

            // sieving
            let (offset, time) = bus_times[0];
            let mut solution = time as usize - offset;
            let mut multiple = time as usize;
            for &(offset, time) in &bus_times[1..] {
                solution = (solution..)
                    .step_by(multiple)
                    .find(|num| (num + offset) % time as usize == 0)
                    .unwrap();
                multiple *= time as usize;
            }

            println!("{}", solution);
        }
    }
}

fn day14(part: Part) {
    let input = include_str!("day14_input.txt");
    let mut mask_positions = 0u64;
    let mut mask_bits = 0u64;
    let mut mem = HashMap::new();

    let mem_pattern = regex::Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    for line in input.lines() {
        if line.starts_with("mask") {
            let new_mask = &line[7..];
            mask_positions = 0;
            mask_bits = 0;

            for (i, ch) in new_mask
                .chars()
                .rev()
                .enumerate()
                .map(|(i, ch)| (i as u32, ch))
            {
                match ch {
                    'X' => (),
                    '1' | '0' => {
                        mask_positions |= 1 << i;
                        mask_bits |= (ch.to_digit(2).unwrap() as u64) << i;
                    }
                    _ => unreachable!(),
                };
            }
        } else {
            let captures = mem_pattern.captures(line).unwrap();
            let pos = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let num = captures.get(2).unwrap().as_str().parse::<u64>().unwrap();

            match part {
                Part::One => {
                    let val_to_write = (num & !mask_positions) | mask_bits;

                    *mem.entry(pos).or_insert(0) = val_to_write;
                }
                Part::Two => {
                    let pos = (pos & mask_positions) | mask_bits;

                    // computing all the masks from the floating bits.
                    // this should actually be done when reading in the masks, but I'm not changing
                    // data structures.
                    let floating_bits = !mask_positions;
                    let floating_bits = (0..36).filter(|pos| floating_bits & 1 << pos != 0);

                    let n_floating = floating_bits.clone().count();

                    for choice in 0..2u64.pow(n_floating as u32) {
                        let mask = floating_bits
                            .clone()
                            .enumerate()
                            .map(|(floating_bit_num, floating_bit_pos)| {
                                let choice = choice & 1 << floating_bit_num != 0;
                                (choice as u64) << floating_bit_pos
                            })
                            .fold(0, std::ops::BitOr::bitor);

                        *mem.entry(pos | mask).or_insert(0) = num;
                    }
                }
            }
        }
    }

    println!("{}", mem.values().sum::<u64>());
}

fn day15(part: Part) {
    let input = vec![14, 3, 1, 0, 9, 5];
    // let input = vec![0, 3, 6];
    let mut prev_seen = HashMap::new();

    for (i, &num) in input[..input.len() - 1].iter().enumerate() {
        prev_seen.insert(num, i);
    }

    let mut pos = input.len() - 1;
    let mut next_num = *input.last().unwrap();
    let first = input.iter().copied().take(prev_seen.len());
    let following = std::iter::from_fn(|| {
        use std::collections::hash_map::Entry;
        let new_next = match prev_seen.entry(next_num) {
            Entry::Occupied(mut o) => {
                let last_seen_pos = o.get_mut();
                pos - std::mem::replace(last_seen_pos, pos)
            }
            Entry::Vacant(v) => {
                v.insert(pos);
                0
            }
        };
        pos += 1;
        Some(std::mem::replace(&mut next_num, new_next))
    });
    let mut iter = first.chain(following);

    let nth = match part {
        Part::One => 2020,
        Part::Two => 30_000_000,
    };
    // -1 because it's 0 based
    println!("{}", iter.nth(nth - 1).unwrap());
}

fn day16(part: Part) {
    let input = include_str!("day16_input.txt");
    let mut blocks = input.split("\n\n");
    let conditions = blocks.next().unwrap();
    let my_ticket = blocks.next().unwrap();
    let other_tickets = blocks.next().unwrap();

    let condition_pattern = regex::Regex::new(r"([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    let fields = conditions
        .lines()
        .map(|condition| {
            let captures = condition_pattern.captures(condition).unwrap();

            let name = captures.get(1).unwrap().as_str();
            let get_num = |group_num| {
                captures
                    .get(group_num)
                    .unwrap()
                    .as_str()
                    .parse::<u64>()
                    .unwrap()
            };
            (name, [get_num(2)..=get_num(3), get_num(4)..=get_num(5)])
        })
        .collect::<Vec<_>>();

    let parse_ticket = |ticket: &str| -> Result<Vec<u64>, u64> {
        let mut error_rate = 0;
        ticket
            .split(",")
            .map(|num| {
                let num = num.parse::<u64>().unwrap();
                let all_valid = fields
                    .iter()
                    .flat_map(|(_, valid_ranges)| valid_ranges)
                    .any(|rg| rg.contains(&num));
                match all_valid {
                    true => Some(num),
                    false => {
                        error_rate += num;
                        None
                    }
                }
            })
            .collect::<Option<Vec<_>>>()
            .ok_or(error_rate)
    };

    let mut error_rate = 0;
    let valid_tickets = other_tickets
        .lines()
        .skip(1)
        .chain(my_ticket.lines().skip(1))
        .filter_map(|line| {
            // keep only valid lines and parse them into vecs of nums

            match parse_ticket(line) {
                Ok(vals) => Some(vals),
                Err(errors) => {
                    error_rate += errors;
                    None
                }
            }
        })
        .collect::<Vec<_>>();

    // Algorithm for part 2:
    // Imagine a boolean (sparse) matrix of field positions (indicated by a number) <-> fields (indicated by a name)
    // where `matrix[field, position] == true` if the field could be at the position, i.e. all tickets
    // have data at that position which is valid for the field.
    // If any row or column has only a single true entry, then that pairing MUST be correct
    // and we can remove the row and column from the matrix and repeat the procedure until everything has been
    // deduced.
    //
    // The matrix is implemented here through a HashMap<FieldPosition (== usize), HashSet<FieldName (== &str)>>
    // which could surely be implemented more concisely and with more clarity but I've spent enough time here already.

    let n_fields = valid_tickets[0].len();
    let mut unsolved_field_nums = (0..n_fields)
        .map(|field_num| {
            let possible_fields = fields
                .iter()
                .filter(|(_, ranges)| {
                    valid_tickets
                        .iter()
                        .map(|ticket_fields| ticket_fields[field_num])
                        .all(|num| ranges.iter().any(|rg| rg.contains(&num)))
                })
                .map(|(name, _)| name)
                .collect::<HashSet<_>>();
            (field_num, possible_fields)
        })
        .collect::<HashMap<_, _>>();

    let mut field_to_num = HashMap::new();
    while field_to_num.len() != n_fields {
        // positions where only 1 field is valid for all tickets
        let unique_by_field =
            unsolved_field_nums
                .iter()
                .filter_map(|(field_num, possible_fields)| {
                    if possible_fields.len() == 1 {
                        Some((*field_num, **possible_fields.iter().next().unwrap()))
                    } else {
                        None
                    }
                });

        // fields where only 1 position is valid for all tickets
        let unsolved_fields = fields
            .iter()
            .filter(|(field_name, _)| !field_to_num.contains_key(field_name))
            .map(|&(name, _)| name);
        let unique_by_position = unsolved_fields.filter_map(|name| {
            let possible_positions = unsolved_field_nums
                .iter()
                .filter(|&(_, possible_fields)| possible_fields.contains(&name))
                .map(|(&field_num, _)| field_num)
                .collect::<Vec<_>>();
            if possible_positions.len() == 1 {
                Some((possible_positions[0], name))
            } else {
                None
            }
        });

        let new_uniques = unique_by_field
            .chain(unique_by_position)
            .collect::<Vec<_>>();
        let n_unique = new_uniques.len();

        for &(field_num, field_name) in &new_uniques {
            field_to_num.insert(field_name, field_num);
            unsolved_field_nums.remove(&field_num);
            for possible_fields in unsolved_field_nums.values_mut() {
                possible_fields.remove(&field_name);
            }
        }

        assert!(n_unique != 0);
    }

    match part {
        Part::One => println!("{}", error_rate),
        Part::Two => {
            let my_ticket = parse_ticket(my_ticket.lines().skip(1).next().unwrap()).unwrap();
            let solution = field_to_num
                .iter()
                .filter(|(name, _)| name.starts_with("departure"))
                .map(|(_, field_num)| field_num)
                .map(|&num| my_ticket[num as usize])
                .product::<u64>();
            println!("{}", solution);
        }
    }
}

fn day17(part: Part) {
    let input = include_str!("day17_input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '#' => true,
                    '.' => false,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let n_cycles = 6;

    let n_width = input[0].len();
    let n_height = input.len();
    let n_depth = 1;

    // make grid big enough to fit the result of the boot cycle
    // 6 cycles => +6 cells in every direction that could have an active cube
    // +1 in every direction so the adjacent cells exist as well
    let offset = 6 + 1;

    let grid_width = n_width + 2 * offset;
    let grid_height = n_height + 2 * offset;
    let grid_depth = n_depth + 2 * offset;
    let mut grid = vec![vec![vec![vec![false; grid_depth]; grid_depth]; grid_height]; grid_width];

    for (y, row) in input.iter().enumerate() {
        for (x, &is_active) in row.iter().enumerate() {
            grid[x + offset][y + offset][offset][offset] = is_active;
        }
    }

    let mut next_grid = grid.clone();
    for _ in 0..n_cycles {
        for (x, y, z, zz) in
            itertools::iproduct!(1..grid_width - 1, 1..grid_height - 1, 1..grid_depth - 1, 1..grid_depth-1)
            .filter(|&(_, _,_, zz)| zz == offset || part == Part::Two)
        {
            let n_neighbors_active =
                itertools::iproduct!(x - 1..=x + 1, y - 1..=y + 1, z - 1..=z + 1, zz - 1..=zz+1)
                    .filter(|&(_,_,_, zz)| {
                        zz == offset || part == Part::Two // 4D sim only for part 2
                    })
                    .filter(|&neighbor_pos| neighbor_pos != (x, y, z, zz))
                    .filter(|&(x, y, z, zz)| grid[x][y][z][zz])
                    .count();
            next_grid[x][y][z][zz] =
                n_neighbors_active == 3 || n_neighbors_active == 2 && grid[x][y][z][zz];
        }
        grid = next_grid.clone();
    }

    println!(
        "{}",
        grid.iter()
            .flatten()
            .flatten()
            .flatten()
            .filter(|&&is_active| is_active)
            .count()
    );
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
        day12(Part::Two);
        day13(Part::One);
        day13(Part::Two);
        day14(Part::One);
        day14(Part::Two);
        day15(Part::One);
        day15(Part::Two);
        day16(Part::One);
        day16(Part::Two);
        day17(Part::One);
    }
    day17(Part::Two);
}
