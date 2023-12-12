#![feature(iter_map_windows)]

advent_of_code::solution!(9);

fn is_all_zeroes(seq: &Vec<i32>) -> bool {
    seq.iter().all(|x| *x == 0)
}

fn extrapolate_last(history: &Vec<i32>) -> i32 {

    let mut sequences: Vec<Vec<i32>> = Vec::new();

    let mut current_sequence: Vec<i32> = history.clone();

    sequences.push(current_sequence.clone());

    for i in 0..history.len()  {
        let mut reduced_sequence: Vec<i32> = Vec::new();
        current_sequence.iter().map_windows(|[x, y]| {
            *y - *x
        }).for_each(|x| reduced_sequence.push(x));
        if is_all_zeroes(&reduced_sequence) {
            break;
        }
        sequences.push(reduced_sequence.clone());
        current_sequence = reduced_sequence;
    }

    let mut extrapolated = 0;

    for i in (0..sequences.len()).rev() {
        extrapolated = sequences[i].last().unwrap() + extrapolated;
    }
    extrapolated
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<Vec<i32>> = input.split('\n')
        .map(|x| x.split(" "))
        .map(|x| x.filter(|y| y != &""))
        .map(|x| x.map(|y| y.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .filter(|x| x.len() > 0)
        .collect();

    Some(lines.iter().map(|x| extrapolate_last(x)).sum::<i32>() as u32)
}

fn extrapolate_first(history: &Vec<i32>) -> i32 {

    let mut sequences: Vec<Vec<i32>> = Vec::new();

    let mut current_sequence: Vec<i32> = history.clone();

    sequences.push(current_sequence.clone());

    for i in 0..history.len()  {
        let mut reduced_sequence: Vec<i32> = Vec::new();
        current_sequence.iter().map_windows(|[x, y]| {
            *y - *x
        }).for_each(|x| reduced_sequence.push(x));
        if is_all_zeroes(&reduced_sequence) {
            break;
        }
        sequences.push(reduced_sequence.clone());
        current_sequence = reduced_sequence;
    }

    let mut extrapolated = 0;

    for i in (0..sequences.len()).rev() {
        extrapolated = sequences[i].first().unwrap() - extrapolated;
    }
    extrapolated
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<Vec<i32>> = input.split('\n')
        .map(|x| x.split(" "))
        .map(|x| x.filter(|y| y != &""))
        .map(|x| x.map(|y| y.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .filter(|x| x.len() > 0)
        .collect();

    Some(lines.iter().map(|x| extrapolate_first(x)).sum::<i32>() as u32)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_part_one() {
//         let result = part_one(&advent_of_code::template::read_file("examples", DAY));
//         assert_eq!(result, Some(114));
//     }
//
//     #[test]
//     fn test_part_two() {
//         let result = part_two(&advent_of_code::template::read_file("examples", DAY));
//         assert_eq!(result, Some(2));
//     }
// }
