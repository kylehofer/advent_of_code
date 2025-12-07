use std::{f32::consts::E, time::Instant};

use aoc2025::utils;

const PAPER_ROLL: char = '@';
const EMPTY: char = '.';

pub fn main() {
    utils::execute_day(
        utils::Day::Day4,
        include_str!("test"),
        include_str!("input"),
        part1,
        part2,
    );
}

pub fn get_details() -> utils::ExecuteDetails {
    return utils::ExecuteDetails {
        day: utils::Day::Day4,
        test: include_str!("test").to_string(),
        input: include_str!("input").to_string(),
    };
}

struct Position {
    row: usize,
    column: i16,
}

const POSITIONS: [Position; 8] = [
    Position { row: 0, column: -1 },
    Position { row: 0, column: 0 },
    Position { row: 0, column: 1 },
    Position { row: 1, column: -1 },
    Position { row: 1, column: 1 },
    Position { row: 2, column: -1 },
    Position { row: 2, column: 0 },
    Position { row: 2, column: 1 },
];

fn process_row(
    chars: Vec<char>,
    fields: &mut [Vec<char>; 3],
    length: usize,
    positions: &[Position; 8],
) -> u32 {
    let mut result = 0;
    fields.swap(0, 1);
    fields.swap(1, 2);

    fields[2] = vec![EMPTY; length + 2];
    fields[2].splice(1..length, chars);

    for i in 1..(length + 1) {
        if fields[1][i] != PAPER_ROLL {
            continue;
        }

        let mut count = 0;

        for position in positions {
            if fields[position.row][(position.column + i as i16) as usize] == PAPER_ROLL {
                count += 1;
            }
            if count >= 4 {
                break;
            }
        }

        if count < 4 {
            result += 1;
        }
    }

    return result;
}

pub fn part1(input: &str) -> u32 {
    let mut first = true;
    let mut fields: [Vec<char>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    let mut length: usize = 0;

    let mut result = 0;

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();

        if first {
            first = false;
            length = line.len();
            fields[0] = vec![EMPTY; length + 2];
            fields[1] = vec![EMPTY; length + 2];
            fields[2] = vec![EMPTY; length + 2];
            fields[2].splice(1..(length), chars);
        } else {
            result += process_row(chars, &mut fields, length, &POSITIONS);
        }
    }
    result += process_row(vec![EMPTY; length + 2], &mut fields, length, &POSITIONS);
    return result;
}

const THRESHOLD: u32 = ((4 * EMPTY as u32) + (4 * PAPER_ROLL as u32)) / 8;

pub fn part2(input_template: &str) -> u32 {
    let mut result = 1;
    let mut next_result = 0;

    let mut input: Vec<Vec<char>>;

    let test_length: usize = match input_template.find('\n') {
        Some(index) => index,
        None => 0,
    };

    let height: usize = input_template.len() / test_length;
    let length = test_length + 2;

    let mut checkable_index: Vec<(usize, usize)> = Vec::with_capacity(height * length);

    {
        const EMPTY_ARR: [char; 1] = [EMPTY];
        input = Vec::with_capacity(height + 2);
        input.push(vec![EMPTY; length]);

        let mut i = 1;

        input_template
            .lines()
            .map(|line| {
                let temp: Vec<char> = line.chars().collect();
                return [&EMPTY_ARR, temp.as_slice(), &EMPTY_ARR].concat();
            })
            .for_each(|f| {
                f.iter()
                    // .filter(|&&c| c == PAPER_ROLL)
                    .enumerate()
                    .for_each(|(x, &c)| {
                        if c == PAPER_ROLL {
                            checkable_index.push((i, x))
                        }
                    });
                input.push(f);
                i += 1;
            });

        input.push(vec![EMPTY; length]);
    }

    while result != next_result {
        result = next_result;

        checkable_index = checkable_index
            .into_iter()
            .filter(|&(row, column)| {
                if ((input[row - 1][column - 1] as u32
                    + input[row - 1][column] as u32
                    + input[row - 1][column + 1] as u32
                    + input[row][column - 1] as u32
                    + input[row][column + 1] as u32
                    + input[row + 1][column - 1] as u32
                    + input[row + 1][column] as u32
                    + input[row + 1][column + 1] as u32)
                    >> 3)
                    < THRESHOLD
                {
                    input[row][column] = EMPTY;
                    next_result += 1;
                    return false;
                }
                return true;
            })
            .collect();
    }

    return result;
}
