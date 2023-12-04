advent_of_code::solution!(2);

use fancy_regex::Regex;

#[derive(Debug, Copy, Clone)]
struct Round {
    blue: u32,
    red: u32,
    green: u32
}

impl Round {

    fn from_str(input: &str) -> Self {
        let re = Regex::new(r"((?<blue>\d+) (blue))|((?<red>\d+) (red))|((?<green>\d+) (green))").unwrap();

        let mut green = 0;
        let mut red = 0;
        let mut blue = 0;

        for captures in re.captures_iter(input).map(|x| x.unwrap()) {
            if let Some(green_match) = captures.name("green") {
                green = green_match.as_str().parse::<u32>().unwrap()
            }
            if let Some(red_match) =  captures.name("red") {
                red = red_match.as_str().parse::<u32>().unwrap()
            }
            if let Some(blue_match) = captures.name("blue") {
                blue = blue_match.as_str().parse::<u32>().unwrap()
            }
        }
        Self {green, red, blue}
    }

}

struct Game {
    id: u32,
    rounds: Vec<Round>
}

fn extract_game_id(input: &str) -> u32 {
    let re = Regex::new(r"Game (\d+)").unwrap();
    re.captures(input).unwrap().unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap()
}

impl Game {

    fn from_str(input: &str) -> Self {
        let game_id = extract_game_id(input);

        // "Game <id>: "
        let game_id_offset = 4 + game_id.to_string().len() + 2;

        let rounds_str = &input[game_id_offset..];

        let rounds: Vec<Round> = rounds_str.split(";").into_iter().map(str::trim).map(Round::from_str).collect();

        Self {id: game_id, rounds}
    }

    fn is_possible(&self, round: &Round) -> bool {
        for played_round in &self.rounds {
            if played_round.blue > round.blue {
                return false;
            }
            if played_round.red > round.red {
                return false;
            }

            if played_round.green > round.green {
                return false;
            }

        }
        true
    }

    fn minimal_cubes(&self) -> Round {

        let mut largest_red = 0;
        let mut largest_green = 0;
        let mut largest_blue = 0;

        for played_round in &self.rounds {

            if played_round.red > largest_red {
                largest_red = played_round.red
            }
            if played_round.green > largest_green {
                largest_green = played_round.green
            }
            if played_round.blue > largest_blue {
                largest_blue = played_round.blue
            }

        }

        Round {red: largest_red, blue: largest_blue, green: largest_green}
    }

}

pub fn part_one(input: &str) -> Option<u32> {
    let round = Round { red: 12, green: 13, blue: 14};
    Some(input.split("\n").into_iter().map(Game::from_str).filter(|x| x.is_possible(&round)).map(|x| x.id).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.split("\n").into_iter().map(Game::from_str).map(|x| x.minimal_cubes()).map(|x| x.blue * x.red * x.green).sum())
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
