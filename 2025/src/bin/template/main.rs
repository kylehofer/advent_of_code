use aoc2025::utils;

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

pub fn part1(input: &str) -> i32 {
    return 0;
}

pub fn part2(input: &str) -> i16 {
    return 0;
}
