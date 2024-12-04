use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq)]
enum Ordering {
    Default,
    Asc,
    Desc,
}

pub fn puzzle2() {
    if let Ok(lines) = read_lines("./data/puzzle2-input-example.txt") {
        let mut safe_reports = 0;
        for line in lines.flatten() {
            // get whole line and separate it to get the numbers as strings
            let split: Vec<&str> = line.split_whitespace().collect();
            let mut report_safe = true;
            let mut ordering = Ordering::Default;
            let mut used_dampener = false;

            let mut prev_num = 0;
            let mut next_num = 0;
            let mut count = 0;
            //1 3 2 4 5
            for str in split.as_slice() {
                let num: i32 = str.parse().unwrap();

                if count + 1 < split.len() {
                    next_num = split.get(count + 1).unwrap().parse().unwrap();
                    println!("num is {num} and next num is {next_num} and count is {count}");
                } else {
                    next_num = 0;
                }

                let next_is_asc = num < next_num;
                let prev_is_asc = prev_num < num;
                let invalid_order = next_num != 0
                    && prev_num != 0
                    && ((next_is_asc && !prev_is_asc) || (!next_is_asc && prev_is_asc));
                let invalid_diff = prev_num != 0 && has_invalid_diff(num, prev_num);
                let invalid_next_diff = next_num != 0 && has_invalid_diff(num, next_num);

                count += 1;
                println!(
                    "invalid diff {invalid_diff} or invalid next diff {invalid_next_diff} or order {invalid_order}. prev num {prev_num}"
                );
                if (invalid_diff || invalid_next_diff || invalid_order) && !used_dampener {
                    used_dampener = true;
                    println!("using dampener");
                    continue;
                }
                if (invalid_diff || invalid_next_diff || invalid_order) && used_dampener {
                    report_safe = false;
                    println!("unsafe");
                    break;
                }

                // only set prev num if no issues found
                prev_num = num;
            }
            if report_safe {
                println!("safe!");
                safe_reports += 1;
            }
        }

        println!("{}", safe_reports);
    }
}

fn has_invalid_diff(num: i32, other: i32) -> bool {
    let diff = num.abs_diff(other);
    return diff > 3 || diff < 1;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
