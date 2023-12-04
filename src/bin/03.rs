
advent_of_code::solution!(3);

struct Schematic {
    matrix: Vec<Vec<char>>
}



impl Schematic {

    fn solve(&self) {
        
        let mut found_digit = false;

        let mut digit_spans: Vec<(usize, usize)> = Vec::new();

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

                        let adjacent_fields = self.calc_adjacent_fields(digit_spans);

                    }
                }
            }
        }

    }

    fn calc_adjacent_fields(&self, spans: Vec<(usize, usize)>) -> Vec<(i32, i32)> {

        let mut adjacent_fields = Vec::new();

        let offsets: Vec<i32> = vec![-1, 0, 1];

        for span in spans {
            for offset_x in  &offsets {
                for offset_y  in &offsets {
                    adjacent_fields.push((span.0 as i32 + offset_x, span.1 as i32 + offset_y));
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
    schematic.solve();
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
