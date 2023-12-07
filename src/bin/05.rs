advent_of_code::solution!(5);


struct Map<'a> {
    from: &'a str,
    to: &'a str,
    maps: Vec<(u32, u32, u32)>
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
        println!("{}", m);
        let mut map_iter = m.split(" ");
        let dest_start = map_iter.next().unwrap().parse::<u32>().unwrap();
        let source_start = map_iter.next().unwrap().parse::<u32>().unwrap(); 
        let range_length = map_iter.next().unwrap().parse::<u32>().unwrap();

        // Lets expand the ranges here
        // Ups, to expensive...
        ranges.push((dest_start, source_start, range_length));
    }

    Map {from, to, maps: ranges}
}


pub fn part_one(input: &str) -> Option<u32> {

    let mut input_iter = input.split("\n\n").into_iter();
    let mut seeds = input_iter.next().unwrap();

    let entries: Vec<Map> = input_iter.map(parse_maps).collect();
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
