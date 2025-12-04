use aoc2025::utils;
use prime_factorization::Factorization;
use std::collections::HashSet;

enum ReadState {
    First,
    Second,
}

pub fn main() {
    utils::execute_day(
        utils::Day::Day2,
        include_str!("test"),
        include_str!("input"),
        part1,
        part2,
    );
}

pub fn get_details() -> utils::ExecuteDetails {
    return utils::ExecuteDetails {
        day: utils::Day::Day2,
        test: include_str!("test").to_string(),
        input: include_str!("input").to_string(),
    };
}

fn find_doubles(x: String, target: u64) -> u64 {
    let mut start: u64;
    let length = x.len() as u64;
    let mut boundary: u64;
    let mut power: u64;
    let base: u64 = 10;

    let mut count = 0;

    if length % 2 == 1 {
        power = base.pow(((length as u32) / 2) + 1);
        start = power / 10;
        boundary = (power) - start;
    } else {
        let half = (length / 2) as usize;
        let left: u64 = x[0..half].parse().unwrap();
        let right: u64 = x[half..].parse().unwrap();

        start = if right > left { left + 1 } else { left };

        power = base.pow(half as u32);
        boundary = (power) - start;
    }

    let mut val = start + (start * power);

    while val <= target {
        start += 1;
        boundary -= 1;
        if boundary == 0 {
            power *= 10;
            boundary = power;
        }
        count += val;
        val = start + (start * power);
    }
    return count;
}

pub fn part1(input: &str) -> u64 {
    let mut x: String = String::from("");
    let mut y: u64 = 0;
    let mut state = ReadState::First;
    let mut count: u64 = 0;
    for c in input.chars() {
        match state {
            ReadState::First => {
                if c != '-' {
                    x.push(c);
                } else {
                    state = ReadState::Second;
                }
            }
            ReadState::Second => {
                if c != ',' {
                    y = (y * 10) + (c.to_digit(10).unwrap() as u64);
                } else {
                    state = ReadState::First;
                    count += find_doubles(x.clone(), y);
                    x.clear();
                    y = 0;
                }
            }
        }
    }
    return count + find_doubles(x.clone(), y);
}

fn test_sequence(
    start: u64,
    end: u64,
    sequence_length: u8,
    target_length: u8,
    found_ids: &mut HashSet<u64>,
) {
    if 1 == sequence_length || target_length % sequence_length != 0 {
        return;
    }
    let base: u64 = 10;

    let sequence_count = target_length / sequence_length;
    let power = base.pow((sequence_count - 1) as u32);
    let upper = power * 10;

    let mut value: u64 = 0;
    let mut increment = 0;

    {
        let gap_power = power * 10;
        for _ in 0..sequence_length {
            value = (value * gap_power) + power;
            increment = (increment * gap_power) + 1;
            // println!("building: {value} {gap_power} {power}");
        }
    }

    for _ in power..upper {
        if value < start {
            value += increment;
            continue;
        }

        if value > end {
            return;
        }

        found_ids.insert(value);

        value += increment;
    }
}

fn find_sequences(x: String, target: u64) -> u64 {
    let base: u64 = 10;
    let length = x.len() as u64;

    let mut found_ids: HashSet<u64> = HashSet::new();

    let start: u64 = x.parse().unwrap();

    let mut lower_boundary: u64 = base.pow((length - 1) as u32);
    let mut test_length = length;
    while lower_boundary < target {
        let mut unique_factors = HashSet::new();
        {
            let factor_repr = Factorization::run(test_length);

            unique_factors.insert(1);

            let usable_factors = &factor_repr.factors;

            for &factor in usable_factors {
                unique_factors.insert(factor);
            }
        }
        for &factor in &unique_factors {
            test_sequence(
                start,
                target,
                factor as u8,
                test_length as u8,
                &mut found_ids,
            );
        }
        test_length += 1;
        lower_boundary *= 10;
    }

    let mut count: u64 = 0;
    for id in found_ids {
        count += id;
    }

    return count;
}

pub fn part2(input: &str) -> u64 {
    let mut x: String = String::from("");
    let mut y: u64 = 0;
    let mut state = ReadState::First;
    let mut count: u64 = 0;
    for c in input.chars() {
        match state {
            ReadState::First => {
                if c != '-' {
                    x.push(c);
                } else {
                    state = ReadState::Second;
                }
            }
            ReadState::Second => {
                if c != ',' {
                    y = (y * 10) + (c.to_digit(10).unwrap() as u64);
                } else {
                    state = ReadState::First;
                    count += find_sequences(x.clone(), y);
                    x.clear();
                    y = 0;
                }
            }
        }
    }
    return count + find_sequences(x.clone(), y);
}
