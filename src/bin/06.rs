advent_of_code::solution!(6);

use std::ops::{Sub, Mul};



fn is_winnable<T>(time: T, distance: T, hold_time: T) -> bool
where T: std::cmp::PartialOrd + std::ops::Mul<Output = T> + Copy + std::ops::Sub,
    <T as Sub>::Output: Mul<T>,
    <<T as Sub>::Output as Mul<T>>::Output: std::cmp::PartialOrd<T>
{
    let travel_time = time - hold_time;
    let distance_traveled = travel_time * hold_time;
    distance_traveled >= distance
}


fn parse_input_part_one(input: &str) -> Vec<(u32, u32)> {
    let mut input_iter = input.split("\n").into_iter();
    let mut result = vec![];

    let mut time_iter = input_iter.next().unwrap().split(" ").into_iter();
    let mut distance_iter = input_iter.next().unwrap().split(" ").into_iter();

    let mut first = true;

    // NOTE: We edited the input file
    while let Some(time) = time_iter.next() {
        let distance = distance_iter.next().unwrap();

        if first {
            first = false;
            continue;
        }
        if time == "" {
            continue;
        }
        result.push((time.parse().unwrap(), distance.parse().unwrap()));
    }


    result
}

fn parse_input_part_two(input: &str) -> (u64, u64) {
    let mut input_iter = input.split("\n").into_iter();

    let mut time_iter = input_iter.next().unwrap().split(" ").into_iter();
    let mut distance_iter = input_iter.next().unwrap().split(" ").into_iter();

    let mut first = true;

    let mut total_time = "".to_string();
    let mut total_distance = "".to_string();

    // NOTE: We edited the input file
    while let Some(time) = time_iter.next() {
        let distance = distance_iter.next().unwrap();

        if first {
            first = false;
            continue;
        }
        if time == "" {
            continue;
        }
        total_time += time;
        total_distance += distance;
    }

    (total_time.parse().unwrap(), total_distance.parse().unwrap())
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_input_part_one(input);
    let mut possible_times = vec![];
    for game in input.iter() {
        let mut game_possible_times = 0;
        for time in 0..game.0 {
            if is_winnable(game.0, game.1, time) {
                game_possible_times += 1;
            }
        }
        possible_times.push(game_possible_times);
    }

    Some(possible_times.iter().product())
}

pub fn part_two(input: &str) -> Option<u32> {
    let game = parse_input_part_two(input);
    let mut game_possible_times = 0;
    for time in 0..game.0 {
        if is_winnable(game.0, game.1, time) {
            game_possible_times += 1;
        }
    }
    Some(game_possible_times)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
