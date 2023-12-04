
advent_of_code::solution!(3);

use std::process::exit;

struct Schematic {
    matrix: Vec<Vec<char>>
}



impl Schematic {

    fn solve(&self) -> u32 {

        let mut found_digit = false;

        let mut digit_spans: Vec<(usize, usize)> = Vec::new();

        let mut found_numbers = Vec::new();


        // First identify a number
        for (i, row) in self.matrix.iter().enumerate() {
            for (j, char) in row.iter().enumerate() {
                if char.is_digit(10) {
                    if !found_digit {
                        found_digit = true;
                    }

                    digit_spans.push((i, j));

                } else {
                    if found_digit {
                        let adjacent_fields = self.calc_adjacent_fields(&digit_spans);

                        if self.check_adjacent_fields(&adjacent_fields) {
                            let mut number = "".to_string();
                            for span in &digit_spans {
                                number += &self.matrix.get(span.0).unwrap().get(span.1).unwrap().to_string();
                            }
                            found_numbers.push(number.parse::<u32>().unwrap());
                        }
                        digit_spans.clear();
                    }
                }
            }
        }

        found_numbers.into_iter().sum()
    }

    fn solve2(&self) -> u32 {

        let mut found_digit = false;

        let mut digit_spans: Vec<(usize, usize)> = Vec::new();

        let mut found_numbers: Vec<((i32, i32), u32)>  = Vec::new();

        // let mut checked_gears = Vec::new();


        // First identify a number
        for (i, row) in self.matrix.iter().enumerate() {
            for (j, char) in row.iter().enumerate() {
                if char.is_digit(10) {
                    if !found_digit {
                        found_digit = true;
                    }

                    digit_spans.push((i, j));

                } else {
                    if found_digit {
                        let adjacent_fields = self.calc_adjacent_fields(&digit_spans);

                        if let Some(gear) = self.check_gears(&adjacent_fields) {
                            let mut number = "".to_string();
                            for span in &digit_spans {
                                number += &self.matrix.get(span.0).unwrap().get(span.1).unwrap().to_string();
                            }
                            found_numbers.push((gear, number.parse::<u32>().unwrap()));
                        }

                        digit_spans.clear();
                    }
                }
            }
        }

        let mut gear_set = Vec::new();

        for (gear, _) in &found_numbers {
            if !gear_set.contains(gear) {
                gear_set.push(*gear);
            }
        }

        let mut found_gear_numbers = Vec::new();

        for unique_gear in gear_set {
            let mut gear_numbers = Vec::new();
            for (gear, number) in &found_numbers {
                if gear == &unique_gear {
                    gear_numbers.push(number);
                }
            }

            if gear_numbers.len() > 1 {
                found_gear_numbers.push(gear_numbers.into_iter().product::<u32>());
            }
        }

        found_gear_numbers.into_iter().sum()
    }

    fn check_adjacent_fields(&self, adjacent_fields: &Vec<(i32, i32)>) -> bool {
        let mut is_part_number = false;
        for adjacent_field in adjacent_fields {
            if adjacent_field.0 < 0 || adjacent_field.1 < 0 {
                continue;
            }
            match self.matrix.get(adjacent_field.0 as usize) {
                Some(row) => {
                    match row.get(adjacent_field.1 as usize) {
                        Some(char) => {
                            if !char.is_digit(10) && char != &'.' {
                                is_part_number = true;
                            }
                        },
                        None => {
                            continue;
                        }
                    }
                },
                None => {
                    continue;
                }
            }
        }
        is_part_number
    }

    fn check_gears(&self, adjacent_fields: &Vec<(i32, i32)>) -> Option<(i32, i32)> {
        for adjacent_field in adjacent_fields {
            if adjacent_field.0 < 0 || adjacent_field.1 < 0 {
                continue;
            }
            match self.matrix.get(adjacent_field.0 as usize) {
                Some(row) => {
                    match row.get(adjacent_field.1 as usize) {
                        Some(char) => {
                            if char == &'*' {
                                return Some(*adjacent_field);
                            }
                        },
                        None => {
                            continue;
                        }
                    }
                },
                None => {
                    continue;
                }
            }
        }
        None
    }

    fn calc_adjacent_fields(&self, spans: &Vec<(usize, usize)>) -> Vec<(i32, i32)> {

        let mut adjacent_fields = Vec::new();

        let offsets: Vec<i32> = vec![-1, 0, 1];

        for span in spans {
            for offset_x in  &offsets {
                for offset_y  in &offsets {
                    let adjacent_field = (span.0 as i32 + offset_x, span.1 as i32 + offset_y);
                    if adjacent_field.0 < 0 || adjacent_field.1 < 0 {
                        continue;
                    }
                    adjacent_fields.push(adjacent_field);
                }
            }
        }


        adjacent_fields
    }


}

pub fn part_one(input: &str) -> Option<u32> {

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in  input.split("\n").into_iter() {
         matrix.push(line.chars().collect());

    }

    let schematic = Schematic {matrix};
    Some(schematic.solve())
}

pub fn part_two(input: &str) -> Option<u32> {

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in  input.split("\n").into_iter() {
         matrix.push(line.chars().collect());

    }

    let schematic = Schematic {matrix};
    Some(schematic.solve2())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_part_one() {
//         let result = part_one(&advent_of_code::template::read_file("examples", DAY));
//         // assert_eq!(result, None);
//     }
//
//     #[test]
//     fn test_part_two() {
//         let result = part_two(&advent_of_code::template::read_file("examples", DAY));
//         // assert_eq!(result, None);
//     }
// }
