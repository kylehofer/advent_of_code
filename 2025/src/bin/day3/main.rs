use aoc2025::utils;
use std::cmp;

pub fn main() {
    utils::execute_day(
        utils::Day::Day3,
        include_str!("test"),
        include_str!("input"),
        part1,
        part2,
    );
}

pub fn get_details() -> utils::ExecuteDetails {
    return utils::ExecuteDetails {
        day: utils::Day::Day3,
        test: include_str!("test").to_string(),
        input: include_str!("input").to_string(),
    };
}

pub fn part1(input: &str) -> u32 {
    let mut result: u32 = 0;
    for line in input.lines() {
        let mut x: u32 = 0;
        let mut y: u32 = 0;
        let mut index = 0;
        let upper = line.len() - 1;
        for char in line.chars() {
            let value = char.to_digit(10).unwrap();
            if value > x && index < upper {
                x = value;
                y = 0;
            } else {
                y = cmp::max(value, y);
            }
            index += 1;
        }
        result += (x * 10) + y;
    }
    return result;
}

pub fn part2(input: &str) -> u64 {
    const MAX_SIZE: usize = 12;
    let mut result: u64 = 0;
    let mut array = [0; MAX_SIZE];
    for line in input.lines() {
        array.fill(0);
        let mut remaining = line.len();
        for char in line.chars() {
            let value = char.to_digit(10).unwrap();
            let start = if remaining >= MAX_SIZE {
                0
            } else {
                MAX_SIZE - remaining
            };
            for i in start..MAX_SIZE {
                let x = array[i];
                if value > x {
                    array[i] = value;
                    if i < MAX_SIZE - 1 {
                        array[i + 1] = 0;
                    }
                    break;
                }
            }
            remaining -= 1;
        }
        let calculated_value: u64 = array.iter().fold(0, |acc, &num| (acc * 10) + num as u64);
        result += calculated_value;
    }
    return result;
}
