advent_of_code::solution!(7);


#[derive(Debug, PartialEq, PartialOrd)]
enum CardType {
    A = 14,
    K = 13,
    Q = 12,
    J = 11,
    T = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3 ,
    Two = 2
}

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1
}


#[derive(Debug)]
struct Hand {
    cards: Vec<CardType>,
    r#type: HandType
}

fn build_hand(cards: &Vec<&str>) -> Vec<CardType> {
    let mut hand = Vec::new();

    for card in cards {
        match *card {
            "A" => hand.push(CardType::A),
            "K" => hand.push(CardType::K),
            "Q" => hand.push(CardType::Q),
            "J" => hand.push(CardType::J),
            "T" => hand.push(CardType::T),
            "9" => hand.push(CardType::Nine),
            "8" => hand.push(CardType::Eight),
            "7" => hand.push(CardType::Seven),
            "6" => hand.push(CardType::Six),
            "5" => hand.push(CardType::Five),
            "4" => hand.push(CardType::Four),
            "3" => hand.push(CardType::Three),
            "2" => hand.push(CardType::Two),
            _ => panic!("Unknown card type")
        }
    }
    hand
}

fn identify_hand_type(cards: Vec<&str>) -> HandType {
    let mut counts = std::collections::HashMap::new();
    for &card in &cards {
        *counts.entry(card).or_insert(0) += 1;
    }

    let mut frequencies = counts.values().cloned().collect::<Vec<_>>();
    frequencies.sort_unstable();

    if frequencies.contains(&5) {
        return HandType::FiveOfAKind;
    }

    if frequencies.contains(&4) {
        return HandType::FourOfAKind;
    }

    if frequencies.contains(&3) {
        if frequencies.contains(&2) {
            return HandType::FullHouse;
        }
        return HandType::ThreeOfAKind
    }
    let two_count = frequencies.clone().iter().filter(|&x| x == &2).count();
    match two_count {
        2 => return HandType::TwoPair,
        1 => return HandType::OnePair,
        _ => return HandType::HighCard

    }
}

impl Hand {
    fn new(cards: Vec<&str>) -> Self {
        let hand_cards = build_hand(&cards);
        let hand_type = identify_hand_type(cards);
        Self {cards: hand_cards, r#type: hand_type}
    }
}

impl PartialEq for Hand {

    fn eq(&self, other: &Self) -> bool {
        panic!("Should not happen")
    }
}

impl PartialOrd for Hand {

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let cmp_hand_type = self.r#type.partial_cmp(&other.r#type);
        match cmp_hand_type.unwrap() {
            std::cmp::Ordering::Less => return Some(std::cmp::Ordering::Less),
            std::cmp::Ordering::Greater => return Some(std::cmp::Ordering::Greater),
            std::cmp::Ordering::Equal => {
                for i in 0..self.cards.len() {
                    let self_card = self.cards.get(i).unwrap();
                    let other_card = other.cards.get(i).unwrap();
                    if self_card != other_card {
                        return self_card.partial_cmp(other_card);
                    }
                }
            }
        }
        Some(std::cmp::Ordering::Equal)
    }

}


fn parse_input(input: &str) -> Vec<(Hand, u32)> {
    let mut lines: Vec<(Hand, u32)> = input.split("\n").into_iter()
        .map(|x| {
            let mut line_iter = x.split(" ").into_iter();

            let hand: Vec<&str> = line_iter.next().unwrap().split("").filter(|x| !x.is_empty()).collect();
            if hand.len() == 0 {
                return (vec![], 0)
            }
            let value = line_iter.next().unwrap().parse().unwrap();

            (hand, value)
        })
        .map(|x| (Hand::new(x.0), x.1))
        .collect();
    lines.pop();
    lines
}

#[derive(Debug, PartialEq, PartialOrd)]
enum CardType2 {
    A = 14,
    K = 13,
    Q = 12,
    T = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3 ,
    Two = 2,
    J = 1,
}

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType2 {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1
}


#[derive(Debug)]
struct Hand2 {
    cards: Vec<CardType2>,
    r#type: HandType2
}

fn build_hand2(cards: &Vec<&str>) -> Vec<CardType2> {
    let mut hand = Vec::new();

    for card in cards {
        match *card {
            "A" => hand.push(CardType2::A),
            "K" => hand.push(CardType2::K),
            "Q" => hand.push(CardType2::Q),
            "J" => hand.push(CardType2::J),
            "T" => hand.push(CardType2::T),
            "9" => hand.push(CardType2::Nine),
            "8" => hand.push(CardType2::Eight),
            "7" => hand.push(CardType2::Seven),
            "6" => hand.push(CardType2::Six),
            "5" => hand.push(CardType2::Five),
            "4" => hand.push(CardType2::Four),
            "3" => hand.push(CardType2::Three),
            "2" => hand.push(CardType2::Two),
            _ => panic!("Unknown card type")
        }
    }
    hand
}

fn identify_hand_type2(cards: Vec<&str>) -> HandType2 {

    if cards.len() == 0 {
        return HandType2::HighCard;
    }
    let mut counts = std::collections::HashMap::new();
    for &card in &cards {
        *counts.entry(card).or_insert(0) += 1;
    }

    let count_js = counts.get("J").unwrap_or_else(|| &0);

    if *count_js == 5 {
        return HandType2::FiveOfAKind;
    }
    let mut counts = std::collections::HashMap::new();
    for &card in &cards {
        if card.eq("J") {
            continue;
        }
        *counts.entry(card).or_insert(0) += 1;
    }

    let mut frequencies = counts.values().cloned().collect::<Vec<_>>();
    frequencies.sort_unstable();

    let mut highest_freq = 0;

    for i in 0..frequencies.len() {
        if frequencies[i] > frequencies[highest_freq]  {
            highest_freq = i;
        }
    }

    frequencies[highest_freq] += count_js;

    if frequencies.contains(&5) {
        return HandType2::FiveOfAKind;
    }

    if frequencies.contains(&4) {
        return HandType2::FourOfAKind;
    }

    if frequencies.contains(&3) {
        if frequencies.contains(&2) {
            return HandType2::FullHouse;
        }
        return HandType2::ThreeOfAKind
    }
    let two_count = frequencies.clone().iter().filter(|&x| x == &2).count();
    match two_count {
        2 => return HandType2::TwoPair,
        1 => return HandType2::OnePair,
        _ => return HandType2::HighCard

    }
}

impl Hand2 {
    fn new(cards: Vec<&str>) -> Self {
        let hand_cards = build_hand2(&cards);
        let hand_type = identify_hand_type2(cards);
        Self {cards: hand_cards, r#type: hand_type}
    }
}

impl PartialEq for Hand2 {

    fn eq(&self, other: &Self) -> bool {
        panic!("Should not happen")
    }
}

impl PartialOrd for Hand2 {

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let cmp_hand_type = self.r#type.partial_cmp(&other.r#type);
        match cmp_hand_type.unwrap() {
            std::cmp::Ordering::Less => return Some(std::cmp::Ordering::Less),
            std::cmp::Ordering::Greater => return Some(std::cmp::Ordering::Greater),
            std::cmp::Ordering::Equal => {
                for i in 0..self.cards.len() {
                    let self_card = self.cards.get(i).unwrap();
                    let other_card = other.cards.get(i).unwrap();
                    if self_card != other_card {
                        return self_card.partial_cmp(other_card);
                    }
                }
            }
        }
        Some(std::cmp::Ordering::Equal)
    }

}

fn parse_input2(input: &str) -> Vec<(Hand2, u32)> {
    let mut lines: Vec<(Hand2, u32)> = input.split("\n").into_iter()
        .map(|x| {
            let mut line_iter = x.split(" ").into_iter();

            let hand: Vec<&str> = line_iter.next().unwrap().split("").filter(|x| !x.is_empty()).collect();
            if hand.len() == 0 {
                return (vec![], 0)
            }
            let value = line_iter.next().unwrap().parse().unwrap();

            (hand, value)
        })
        .map(|x| (Hand2::new(x.0), x.1))
        .collect();
    lines.pop();
    lines
}


pub fn part_one(input: &str) -> Option<u32> {
    let mut input = parse_input(input);
    input.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut result = 0;
    let mut multiplier = 1;

    for (hand, value) in input {
        result += (value * multiplier);
        multiplier += 1;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input = parse_input2(input);
    input.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut result = 0;
    let mut multiplier = 1;

    for (hand, value) in &input {
        println!("{:?} {}", hand, value);
    }

    for (hand, value) in input {
        result += (value * multiplier);
        multiplier += 1;
    }

    Some(result)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_part_one() {
//         let result = part_one(&advent_of_code::template::read_file("examples", DAY));
//         assert_eq!(result, Some(6592));
//     }
//
//     #[test]
//     fn test_part_two() {
//         let result = part_two(&advent_of_code::template::read_file("examples", DAY));
//         assert_eq!(result, Some(6839));
//     }
// }
