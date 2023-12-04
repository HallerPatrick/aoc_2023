advent_of_code::solution!(1);

use std::collections::HashMap;

use fancy_regex::Regex;

fn extract_first_and_list_digit(s: &str) -> u32 {
    let mut integers: Vec<char> = Vec::new();
    for char in s.chars().into_iter() {
        if char.is_numeric() {
            integers.push(char);
        }
    }

    if let Some(first_digit) = integers.first() {
        if let Some(last_digit) = integers.last() {
            let str_repr = format!("{}{}", first_digit, last_digit);
            if let Ok(value) = str_repr.parse::<u32>() {
                return value
            }
        }

    }
    0
}

fn extract_first_and_list_digit_with_string_value(s: &str, m: &HashMap<&str, &str>) -> u32 {
    let re_digit_or_str_digit = Regex::new(r"(?=(one|two|three|four|five|six|seven|eight|nine|[0-9]))").unwrap();

    let mut digits: Vec<&str> = Vec::new();

    for capture in re_digit_or_str_digit.captures_iter(s).map(|x| x.unwrap()) {
        digits.push(capture.get(1).expect("Not Found").as_str());
    }

    if let Some(first_digit_str) = digits.first() {
        if let Some(last_digit_str) = digits.last() {
            let first_digit: &str;
            let last_digit: &str;
            if m.contains_key(first_digit_str) {
                first_digit = m.get(first_digit_str).unwrap();
            } else {
                first_digit = first_digit_str;
            }

            if m.contains_key(last_digit_str) {
                last_digit = m.get(last_digit_str).unwrap();
            } else {
                last_digit = last_digit_str;
            }
            let str_repr = format!("{}{}", first_digit, last_digit);
            println!("{}", str_repr);
            if let Ok(value) = str_repr.parse::<u32>() {
                return value
            }
        }

    }

    0
}

pub fn part_one(input: &str) -> Option<u32> {
    let result: u32 = input.split("\n").into_iter().map(extract_first_and_list_digit).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {

    let str_to_digit = HashMap::from([
        ("one",   "1"),
        ("two",   "2"),
        ("three",  "3"),
        ("four",  "4"),
        ("five",  "5"),
        ("six",   "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine",  "9"),
        ("zero",  "0")
    ]);
    let result: u32 = input.split("\n").into_iter().map(|x| extract_first_and_list_digit_with_string_value(x, &str_to_digit)).sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }
}
