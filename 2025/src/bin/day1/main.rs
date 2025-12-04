use aoc2025::utils;
enum Direction {
    Left = -1,
    Right = 1,
}

struct Movement {
    direction: i16,
    distance: i16,
}

pub fn main() {
    utils::execute_day(
        utils::Day::Day1,
        include_str!("test"),
        include_str!("input"),
        part1,
        part2,
    );
}

pub fn get_details() -> utils::ExecuteDetails {
    return utils::ExecuteDetails {
        day: utils::Day::Day1,
        test: include_str!("test").to_string(),
        input: include_str!("input").to_string(),
    };
}

pub fn part1(input: &str) -> i16 {
    let mut count = 0;
    let mut position = 50;
    for line in input.lines() {
        let direction = line.chars().next().unwrap();

        let movement = Movement {
            direction: if direction == 'L' { -1 } else { 1 },
            distance: line[1..].parse().unwrap(),
        };

        position = update_dial(position, movement);
        if position == 0 {
            count += 1;
        }
    }

    return count;
}

pub fn part2(input: &str) -> i16 {
    let mut count = 0;
    let mut position = 50;
    for line in input.lines() {
        let direction = line.chars().next().unwrap();

        let movement = Movement {
            direction: if direction == 'L' { -1 } else { 1 },
            distance: line[1..].parse().unwrap(),
        };

        count += count_dial(&mut position, movement);
    }

    if position == 0 {
        count += 1;
    }

    return count;
}

fn update_dial(position: i16, movement: Movement) -> i16 {
    let update = position + ((movement.distance % 100) * (movement.direction as i16));

    if update > 99 {
        return update - 100;
    }

    if update < 0 {
        return update + 100;
    }

    return update;
}

fn count_dial(position: &mut i16, movement: Movement) -> i16 {
    let mut count = movement.distance / 100;
    let update = *position + ((movement.distance % 100) * (movement.direction as i16));

    if update > 99 {
        *position = update - 100;
        return count + 1;
    }

    if update < 0 {
        if *position != 0 {
            count += 1;
        }

        return count;
    }

    *position = update;
    if update == 0 {
        return count + 1;
    } else {
        return count;
    }
}
