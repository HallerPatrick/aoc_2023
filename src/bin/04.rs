advent_of_code::solution!(4);


use std::collections::VecDeque;

type Game = Vec<Vec<u32>>;

fn parse_input(input: &str) -> Vec<Game> {

    let raw_games: Vec<Vec<Vec<u32>>> = input.split("\n").into_iter()
        .map(|x| x.split(":")) // Remove Game id prefix
        .map(|x| x.last().unwrap()) // Take games
        .map(|x| x.split("|")
             .map(str::trim)
             .map(|y| y.split(" ")
                  .map(str::trim)
                  .filter(|i| !i.is_empty())
                  .map(str::parse::<u32>)
                  .map(|i| i.unwrap())
                  .collect())
             .collect()
        ).collect();

    raw_games.into_iter().filter(|x| x[0].len() > 0).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let games = parse_input(input);

    let mut total_points = 0;

    for game in games {
        let winning_numbers = game.first().unwrap();
        let our_numbers = game.last().unwrap();

        let mut intersection = Vec::new();

        for number in our_numbers {
            if winning_numbers.contains(number) {
                intersection.push(number);
            }
        }

        if intersection.len() > 0 {
            let points = 2_i32.pow(intersection.len() as u32 - 1) ;
            total_points += points;
        }
    }


    Some(total_points as u32)
}

fn matching_numbers(game: Game) -> u32 {

    let winning_numbers = game.first().unwrap();
    let our_numbers = game.last().unwrap();

    let mut overlap: u32 = 0;
    for number in our_numbers {
        if winning_numbers.contains(number) {
            overlap += 1;
        }
    }
    overlap
}

fn process_scratchcards(games: Vec<Game>, id: u32) -> u32 {
    let mut current_games = VecDeque::from(games.clone());

    let current_game = current_games.pop_front().unwrap();
    let copies = matching_numbers(current_game);

    let mut instances = 1;

    for next_game_i in 0..copies {
        instances += process_scratchcards(current_games.clone().into(), id + next_game_i + 1);
        current_games.pop_front();
    }

    instances
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut games = VecDeque::from(parse_input(input));

    let mut total_instances = 0;

    for i in 0..games.len() {
        total_instances += process_scratchcards(games.clone().into(), (i+1).try_into().unwrap());
        games.pop_front();
    }

    Some(total_instances.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
