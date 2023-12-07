advent_of_code::solution!(5);


#[derive(Debug)]
struct Map<'a> {
    from: &'a str,
    to: &'a str,
    maps: Vec<(i64, i64, i64)>
}

fn parse_maps(entry: &str) -> Map {

    let mut entry_iter = entry.split("\n").into_iter();

    let mut name_mapping = entry_iter.next()
            .unwrap()
            .split(" ")
            .next()
            .unwrap()
            .split("-");

    let from = name_mapping.next().unwrap();
    name_mapping.next(); // "to"
    let to = name_mapping.next().unwrap();

    let mut ranges = Vec::new();

    for m in entry_iter {
        if m.is_empty() {
            continue;
        }
        let mut map_iter = m.split(" ");
        let dest_start = map_iter.next().unwrap().parse::<i64>().unwrap();
        let source_start = map_iter.next().unwrap().parse::<i64>().unwrap();
        let range_length = map_iter.next().unwrap().parse::<i64>().unwrap();

        // Lets expand the ranges here
        // Ups, to expensive...
        ranges.push((dest_start, source_start, range_length));
    }

    Map {from, to, maps: ranges}
}


pub fn part_one(input: &str) -> Option<u32> {

    let mut input_iter = input.split("\n\n").into_iter();

    let mut seeds_iter = input_iter.next().unwrap().split(" ").into_iter();
    seeds_iter.next(); // skip seeds prefix

    let mut seeds = seeds_iter.map(|s| s.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let entries: Vec<Map> = input_iter.map(parse_maps).collect();

    let mut found_location = false;
    let mut current_step = "seed";
    let mut current_numbers: Vec<i64> = Vec::from(seeds);

    while !found_location {

        let current_map = entries.iter().find(|m| m.from == current_step).unwrap();

        let mut new_seeds: Vec<i64> = Vec::new();

        for seed in &current_numbers {
            let mut found = false;
            for map in &current_map.maps {
                // Found a match
                if seed >= &map.1 && seed <= &(map.1 + map.2) {
                    new_seeds.push((map.0 - map.1) + seed);
                    found = true;
                }
            }

            if !found {
                new_seeds.push(*seed);
            }
        }

        current_numbers = new_seeds;

        current_step = current_map.to;

        if current_step == "location" {
            found_location = true;
        }
    }

    Some(*current_numbers.iter().min().unwrap() as u32)
}

/// Well this just took one hour to compute...
pub fn part_two(input: &str) -> Option<u32> {
    let mut input_iter = input.split("\n\n").into_iter();

    let mut seeds_iter = input_iter.next().unwrap().split(" ").into_iter();
    seeds_iter.next(); // skip seeds prefix
    let mut seeds_iter = seeds_iter.map(|s| s.trim().parse::<i64>().unwrap()).peekable();


    let mut seed_ranges = Vec::new();
    while seeds_iter.peek().is_some() {
        let start = seeds_iter.next().unwrap();
        let range = seeds_iter.next().unwrap();
        seed_ranges.push(start..(start+range));
    }

    let entries: Vec<Map> = input_iter.map(parse_maps).collect();

    let mut min_of_ranges: Vec<i64> = Vec::new();
    for range in seed_ranges {

        let mut found_location = false;
        let mut current_step = "seed";
        let mut current_numbers: Vec<i64> = Vec::from(range.collect::<Vec<i64>>());

        while !found_location {

            let current_map = entries.iter().find(|m| m.from == current_step).unwrap();
            println!("Current location: {:?}", current_step);

            let mut new_seeds: Vec<i64> = Vec::new();

            for seed in &current_numbers {
                let mut found = false;
                for map in &current_map.maps {
                    // Found a match
                    if seed >= &map.1 && seed <= &(map.1 + map.2) {
                        new_seeds.push((map.0 - map.1) + seed);
                        found = true;
                    }
                }

                if !found {
                    new_seeds.push(*seed);
                }
            }

            current_numbers = new_seeds;

            current_step = current_map.to;

            if current_step == "location" {
                found_location = true;
            }
        }

        min_of_ranges.push(*current_numbers.iter().min().unwrap());
    }

    Some(*min_of_ranges.iter().min().unwrap() as u32)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_part_one() {
//         let result = part_one(&advent_of_code::template::read_file("examples", DAY));
//         assert_eq!(result, None);
//     }
//
//     #[test]
//     fn test_part_two() {
//         let result = part_two(&advent_of_code::template::read_file("examples", DAY));
//         assert_eq!(result, None);
//     }
// }
