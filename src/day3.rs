use aoc_runner_derive::aoc;
use nom::{branch::alt, bytes::complete::tag, character::complete::anychar, multi::many0, IResult};
use regex::Regex;

pub fn puzzle3() {
    let input = std::fs::read_to_string("./data/puzzle3-input.txt").unwrap();
    let result = part2(&input);
    println!("{}", result);
}

#[aoc(day3, part1)]
pub fn mul(input: &str) -> u64 {
    let re = Regex::new(r"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)").unwrap();

    let mut sum: u64 = 0;

    for (_, [a, b]) in re.captures_iter(input).map(|x| x.extract()) {
        let a_num = a
            .parse::<u64>()
            .unwrap_or_else(|astr| panic!("error parsing {} as number", astr));

        let b_num = b
            .parse::<u64>()
            .unwrap_or_else(|astr| panic!("error parsing {} as number", astr));

        sum += a_num * b_num;
    }

    sum
}

fn do_(input: &str) -> IResult<&str, &str> {
    tag("do()")(input)
}

fn dont(input: &str) -> IResult<&str, &str> {
    tag("don't()")(input)
}

fn mul_numbers(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, _) = tag("mul(")(input)?;
    let (input, num1) = nom::character::complete::u64(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, num2) = nom::character::complete::u64(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, (num1, num2)))
}

// Parser for garbage - consumes one char
fn garbage(input: &str) -> IResult<&str, char> {
    anychar(input)
}

#[derive(Debug)]
enum PatternType<'a> {
    Mul((u64, u64)),
    Do(&'a str),
    Dont(&'a str),
    Garbage(char),
}

// Combined parser for any valid pattern
fn pattern(input: &str) -> IResult<&str, PatternType> {
    alt((
        |input| mul_numbers(input).map(|(next, (a, b))| (next, PatternType::Mul((a, b)))),
        |input| do_(input).map(|(next, res)| (next, PatternType::Do(res))),
        |input| dont(input).map(|(next, res)| (next, PatternType::Dont(res))),
    ))(input)
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u64 {
    let mut parser = many0(alt((
        |input| pattern(input),
        |input| garbage(input).map(|(next, res)| (next, PatternType::Garbage(res))),
    )));

    let mut enabled = true;
    let mut sum: u64 = 0;

    let (extra, instructions) = parser(input).expect("parsed successfully");

    // println!("{:?} \nextra: {:?}", instructions, extra);

    for instruction in instructions {
        match instruction {
            PatternType::Mul((a, b)) => {
                if enabled && a < 1000 && b < 1000 {
                    sum += a * b;
                }
            }
            PatternType::Do(_) => {
                enabled = true;
            }
            PatternType::Dont(_) => {
                enabled = false;
            }
            PatternType::Garbage(_) => {}
        }
    }

    sum
}
