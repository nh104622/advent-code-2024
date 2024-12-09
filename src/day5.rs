use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone)]
struct Rule {
    x: i32,
    y: i32,
}

pub fn part2() {
    if let Ok(lines) = read_lines("./data/puzzle5-input.txt") {
        let mut rules: Vec<Rule> = Vec::new();
        let mut total_middle = 0;

        for line in lines.flatten() {
            // add rules from input
            if line.contains("|") {
                let (a, b) = line.split_once('|').unwrap();
                let num1: i32 = a.parse().unwrap();
                let num2: i32 = b.parse().unwrap();

                rules.push(Rule { x: num1, y: num2 });
            }

            // process update rows
            if line.contains(",") {
                let split: Vec<&str> = line.split(",").collect();
                let mut nums: Vec<i32> = Vec::new();
                let mut is_safe = true;

                // store nums from input
                for s in split.as_slice() {
                    let num: i32 = s.parse().unwrap();
                    nums.push(num);
                }

                let rules_for_row: Vec<&Rule> = rules
                    .iter()
                    .filter(|r| nums.contains(&r.x) && nums.contains(&r.y))
                    .collect();
                let mut running_nums: Vec<i32> = nums.clone();
                let mut fixed_row: Vec<i32> = Vec::new();

                // keep iterating until all pages are in the right order
                loop {
                    let mut found_invalid_page = false;
                    for num in running_nums.as_slice() {
                        // fun lil hack here that assumes a row cannot have the same number twice.
                        // we use that logic to add page X infront of page Y with respect to its rules.
                        if fixed_row.contains(num) {
                            continue;
                        }
                        let broken_rules: Vec<&&Rule> = rules_for_row
                            .iter()
                            .filter(|r| r.y == *num && !fixed_row.contains(&r.x))
                            .collect();

                        let broken_len = broken_rules.len();
                        if broken_len > 0 {
                            found_invalid_page = true;
                            is_safe = false;
                            print!("broken rules count {broken_len} ");
                            for broken in broken_rules {
                                fixed_row.push(broken.x);
                            }
                        }
                        fixed_row.push(*num);
                    }

                    if !found_invalid_page {
                        break; // no invalid orders were found during this iteration so we can get out of here.
                    }

                    // reset
                    running_nums.clear();
                    running_nums = fixed_row.clone(); // update iterating row to be validated
                    fixed_row.clear(); // start over reorganizing row
                }

                // only add ones we had to fix
                if !is_safe {
                    let middle_index = fixed_row.len().div_ceil(2) - 1;
                    total_middle += fixed_row[middle_index];
                }
            }
        }

        println!("{}", total_middle);
    } else {
        println!("failed");
    }
}

pub fn part1() {
    if let Ok(lines) = read_lines("./data/puzzle5-input.txt") {
        let mut rules: Vec<Rule> = Vec::new();
        let mut total_middle = 0;

        for line in lines.flatten() {
            // add rules from input
            if line.contains("|") {
                let (a, b) = line.split_once('|').unwrap();
                let num1: i32 = a.parse().unwrap();
                let num2: i32 = b.parse().unwrap();

                rules.push(Rule { x: num1, y: num2 });
            }

            // process update rows
            if line.contains(",") {
                let split: Vec<&str> = line.split(",").collect();
                let mut nums: Vec<i32> = Vec::new();
                let mut is_safe = true;

                // store nums from input
                for s in split.as_slice() {
                    let num: i32 = s.parse().unwrap();
                    nums.push(num);
                }

                let rules_for_row: Vec<&Rule> = rules
                    .iter()
                    .filter(|r| nums.contains(&r.x) && nums.contains(&r.y))
                    .collect();
                let mut running_nums: Vec<i32> = Vec::new();

                for num in nums.as_slice() {
                    let invalid = rules_for_row
                        .iter()
                        .any(|r| r.y == *num && !running_nums.contains(&r.x));

                    if invalid {
                        is_safe = false;
                        break;
                    }

                    running_nums.push(*num);
                }

                if is_safe {
                    let middle_index = nums.len().div_ceil(2) - 1;
                    total_middle += nums[middle_index];
                }
            }
        }

        println!("{}", total_middle);
    } else {
        println!("failed");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
