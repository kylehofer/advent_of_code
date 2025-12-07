use aoc2025::utils;

pub fn main() {
    utils::execute_day(
        utils::Day::Day5,
        include_str!("test"),
        include_str!("input"),
        part1,
        part2,
    );
}

pub fn get_details() -> utils::ExecuteDetails {
    return utils::ExecuteDetails {
        day: utils::Day::Day5,
        test: include_str!("test").to_string(),
        input: include_str!("input").to_string(),
    };
}

enum ReadState {
    Sequences,
    Identifiers,
}

pub fn part1(input: &str) -> i32 {
    let mut read_state = ReadState::Sequences;

    let mut x: u64 = 0;
    let mut value: (u64, u64) = (0, 0);

    let mut sequences: Vec<(u64, u64)> = Vec::new();

    let mut count = 0;

    for c in input.chars() {
        match read_state {
            ReadState::Sequences => match c {
                '0'..='9' => {
                    x = (x * 10) + (c.to_digit(10).unwrap() as u64);
                }
                '-' => {
                    value.0 = x;
                    x = 0;
                }
                '\n' => {
                    if x == 0 {
                        read_state = ReadState::Identifiers;
                        continue;
                    }
                    value.1 = x;
                    sequences.push(value);
                    x = 0;
                }
                _ => {}
            },
            ReadState::Identifiers => match c {
                '0'..='9' => {
                    x = (x * 10) + (c.to_digit(10).unwrap() as u64);
                }
                '\n' => {
                    if sequences.iter().any(|&(a, b)| x >= a && x <= b) {
                        count += 1
                    }
                    x = 0;
                }
                _ => {}
            },
        }
    }
    return count;
}

pub fn part2(input: &str) -> u64 {
    let mut x: u64 = 0;
    let mut value: (u64, u64) = (0, 0);

    let mut sequences: Vec<(u64, u64)> = Vec::new();

    for line in input.lines() {
        if line.len() == 0 {
            break;
        }
        for c in line.chars() {
            match c {
                '0'..='9' => {
                    x = (x * 10) + (c.to_digit(10).unwrap() as u64);
                }
                '-' => {
                    value.0 = x;
                    x = 0;
                }
                '\n' => {
                    break;
                }
                _ => {}
            }
        }
        value.1 = x;
        while let Some(sequence) = sequences
            .iter_mut()
            .find(|sequence| value.0 <= sequence.1 && value.1 >= sequence.0)
        {
            if sequence.0 < value.0 {
                value.0 = sequence.0;
            }

            if sequence.1 > value.1 {
                value.1 = sequence.1;
            }

            sequence.0 = 0;
            sequence.1 = 0;
        }
        sequences.push(value);
        x = 0;
    }

    return sequences
        .iter()
        .filter(|(a, b)| *a != 0 && *b != 0)
        .fold(0, |acc, &item| acc + ((item.1 - item.0) + 1));
}
