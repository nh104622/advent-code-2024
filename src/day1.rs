use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn puzzle1() {
    if let Ok(lines) = read_lines("./data/puzzle1-input.txt") {
        let mut list1: Vec<i32> = Vec::new();
        let mut list2: Vec<i32> = Vec::new();

        for line in lines.flatten() {
            let (a, b) = line.split_once(char::is_whitespace).unwrap();
            let num1: i32 = a.parse().unwrap();
            let num2: i32 = b.trim().parse().unwrap(); // trim white space from beginning since we only split on first whitespace char.

            list1.push(num1);
            list2.push(num2);
        }

        //puzzle1_part1(list1, list2);
        puzzle1_part2(list1, list2);
    } else {
        println!("failed");
    }
}

fn puzzle1_part2(list1: Vec<i32>, list2: Vec<i32>) {
    let mut total_similarity = 0;
    for item in list1.iter() {
        let count = list2.iter().filter(|&f| *f == *item).count() as i32;
        let score = count * *item;
        total_similarity += score;
    }

    println!("{}", total_similarity);
}

fn puzzle1_part1(mut list1: Vec<i32>, mut list2: Vec<i32>) {
    list1.sort();
    list2.sort();

    let mut i = 0;
    let mut total_diff = 0;

    let mut list1_iter = list1.iter();
    let mut list2_iter = list2.iter();

    while i < list1.len() {
        let item1 = list1_iter.next().unwrap();
        let item2 = list2_iter.next().unwrap();
        let diff = item1.abs_diff(*item2);
        total_diff += diff;

        i += 1;
    }

    println!("{}", total_diff);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
