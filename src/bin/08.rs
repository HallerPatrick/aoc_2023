advent_of_code::solution!(8);

use std::collections::HashMap;
use num::integer::lcm;

#[derive(Debug)]
enum Instruction {
    Left,
    Right
}

fn parse_input(input: &str) -> (Vec<Instruction>, Vec<(String, (String, String))>) {

    let mut input_iter = input.split("\n").into_iter();

    let instructions: Vec<Instruction> = input_iter.next().unwrap().chars().map(|x| {
        match x {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Unknown instruction")
        }
    }).collect();

    input_iter.next();

    let mut lookup_lines: Vec<(String, (String, String))> = vec![];

    for line in input_iter {

        let mut line_iter = line.split(" = ").into_iter();
        let left_side = line_iter.next().unwrap();
        if left_side == "" {
            continue;
        }
        let mut right_side_iter = line_iter.next().unwrap().chars().into_iter();
        right_side_iter.next(); // "("
        let mut left_side_option = "".to_string();
        left_side_option += right_side_iter.next().unwrap().to_string().as_str();
        left_side_option += right_side_iter.next().unwrap().to_string().as_str();
        left_side_option += right_side_iter.next().unwrap().to_string().as_str();
        right_side_iter.next(); // ", "
        right_side_iter.next(); // ", "
        let mut right_side_option = "".to_string();
        right_side_option += right_side_iter.next().unwrap().to_string().as_str();
        right_side_option += right_side_iter.next().unwrap().to_string().as_str();
        right_side_option += right_side_iter.next().unwrap().to_string().as_str();

        lookup_lines.push((left_side.to_string(), (left_side_option, right_side_option)));
    }

    (instructions, lookup_lines)
}

pub fn part_one(input: &str) -> Option<u32> {

    let (instructions, lookup_lines) = parse_input(input);

    let first_location = "AAA".to_string();

    let lookup_lines: HashMap<String, (String, String)> = HashMap::from_iter(lookup_lines.into_iter());

    let mut current_location = first_location.clone();
    let mut current_decisions = lookup_lines.get(&first_location).unwrap();

    let mut num_steps = 0;

    loop {
        for instruction in &instructions {

            match instruction {
                Instruction::Left => {
                    current_location = current_decisions.0.clone();
                },
                Instruction::Right => {
                    current_location = current_decisions.1.clone();

                }
            }

            current_decisions = lookup_lines.get(&current_location).unwrap();
            num_steps += 1;
            if current_location == "ZZZ" {
                break;
            }
        }
        if current_location == "ZZZ" {
            break;
        }
    }

    Some(num_steps)
}

fn lcm_vec(numbers: Vec<u64>) -> u64 {
    numbers.iter().fold(1, |acc, &num| lcm(acc, num))
}

pub fn part_two(input: &str) -> Option<u64> {

    let (instructions, lookup_lines) = parse_input(input);

    let first_locations = vec!["JQA", "NHA", "AAA", "FSA", "LLA", "MNA"].iter().map(|x| x.to_string()).collect::<Vec<String>>();

    let lookup_lines: HashMap<String, (String, String)> = HashMap::from_iter(lookup_lines.into_iter());
    let mut vec_num_steps = vec![];

    let mut final_locations = vec![];

    for first_location in first_locations {

        let mut current_location = first_location.clone();
        let mut current_decisions = lookup_lines.get(&first_location).unwrap();

        let mut num_steps_to_z = 0;
        let mut num_steps = 0;

        loop {
            for instruction in &instructions {

                match instruction {
                    Instruction::Left => {
                        current_location = current_decisions.0.clone();
                    },
                    Instruction::Right => {
                        current_location = current_decisions.1.clone();

                    }
                }

                current_decisions = lookup_lines.get(&current_location).unwrap();
                num_steps += 1;
                num_steps_to_z += 1;
                if current_location.ends_with("Z") {
                    break;
                }
            }

                if current_location.ends_with("Z") {
                    final_locations.push(current_location.clone());
                    break;
                }

        }

        vec_num_steps.push(num_steps);
    }

    let result = lcm_vec(vec_num_steps);
    Some(result.try_into().unwrap())
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_part_one() {
//         let result = part_one(&advent_of_code::template::read_file("examples", DAY));
//         assert_eq!(result, Some(6));
//     }
//
//     #[test]
//     fn test_part_two() {
//         let result = part_two(&advent_of_code::template::read_file("examples", DAY));
//         assert_eq!(result, None);
//     }
// }
