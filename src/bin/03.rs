use adv_code_2024::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn part1<R: BufRead>(reader: R) -> usize {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut result = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        for (_, [m1, m2]) in re.captures_iter(&line).map(|c| c.extract()) {
            let n1 = m1.parse::<usize>().unwrap();
            let n2 = m2.parse::<usize>().unwrap();
            result += n1 * n2;
        }
    }
    result
}

fn part2<R: BufRead>(reader: R) -> usize {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))").unwrap();
    let mut result = 0;
    let mut state_do = true;
    for line in reader.lines() {
        let line = line.unwrap();
        for ins in re.captures_iter(&line) {
            match ins.get(0).unwrap().as_str() {
                "do()" => state_do = true,
                "don't()" => state_do = false,
                _ => {
                    if state_do {
                        let n1 = ins.get(1).unwrap().as_str().parse::<usize>().unwrap();
                        let n2 = ins.get(2).unwrap().as_str().parse::<usize>().unwrap();
                        result += n1 * n2;
                    }
                }
            }
        }
    }
    result
}

fn main() {
    start_day(DAY);

    let input_file = BufReader::new(File::open(INPUT_FILE).unwrap());
    println!("\n=== Part 1 ===");
    assert_eq!(161, part1(BufReader::new(TEST.as_bytes())));
    let result = time_snippet!(part1(input_file));
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");
    part2(BufReader::new(TEST.as_bytes()));
    assert_eq!(48, part2(BufReader::new(TEST.as_bytes())));

    let input_file = BufReader::new(File::open(INPUT_FILE).unwrap());
    let result = time_snippet!(part2(input_file));
    println!("Result = {}", result);
}
