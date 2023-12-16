use regex::Regex;
use std::collections::HashMap;
use std::cmp::Ordering;

advent_of_code::solution!(7);

#[derive(Debug, PartialEq, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

fn card_rank_p1(card: &char) -> i8 {
    match card {
        'A'=>13,
        'K'=>12,
        'Q'=>11,
        'J'=>10,
        'T'=>9,
        _=>(card.to_digit(10).unwrap() as i8) - 1
    }
}

fn card_rank_p2(card: &char) -> i8 {
    match card {
        'A'=>13,
        'K'=>12,
        'Q'=>11,
        'T'=>10,
        'J'=>1,
        _=>(card.to_digit(10).unwrap() as i8)
    }
}

fn hand_type_rank(hand_type: &HandType) -> i8 {
    match hand_type {
        HandType::HighCard=>7,
        HandType::OnePair=>6,
        HandType::TwoPair=>5,
        HandType::ThreeOfAKind=>4,
        HandType::FullHouse=>3,
        HandType::FourOfAKind=>2,
        HandType::FiveOfAKind=>1,
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(hand_type_rank(self).cmp(&hand_type_rank(other)))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        hand_type_rank(self).cmp(&hand_type_rank(other))
    }
}

fn compare_card_p1(card_1: &char, card_2: &char) -> Ordering {
    let card_1_rank = card_rank_p1(card_1);
    let card_2_rank = card_rank_p1(card_2);
    return card_1_rank.cmp(&card_2_rank).reverse();
}

fn compare_card_p2(card_1: &char, card_2: &char) -> Ordering {
    let card_1_rank = card_rank_p2(card_1);
    let card_2_rank = card_rank_p2(card_2);
    return card_1_rank.cmp(&card_2_rank).reverse();
}

fn hand_type_p1(card_map: &HashMap<char, u8>) -> HandType {
    let mut card_counts = card_map.values().collect::<Vec<&u8>>();
    card_counts.sort();
    card_counts.reverse();
    let mut hand_type = HandType::HighCard;
    if card_counts[0] == &5 {
        hand_type = HandType::FiveOfAKind;
    } else if card_counts[0] == &4 {
        hand_type = HandType::FourOfAKind;
    } else if card_counts[0] == &3 && card_counts[1] == &2 {
        hand_type = HandType::FullHouse;
    } else if card_counts[0] == &3 {
        hand_type = HandType::ThreeOfAKind;
    } else if card_counts[0] == &2 && card_counts[1] == &2 {
        hand_type = HandType::TwoPair;
    } else if card_counts[0] == &2 {
        hand_type = HandType::OnePair;
    }
    hand_type
}

fn hand_type_p2(card_map: &HashMap<char, u8>) -> HandType {
    let mut card_counts = card_map.values().collect::<Vec<&u8>>();
    card_counts.sort();
    card_counts.reverse();

    let mut hand_type = HandType::HighCard;
    if card_counts[0] == &5 {
        hand_type = HandType::FiveOfAKind;
    } else if card_counts[0] == &4 {
        if card_map.contains_key(&'J') { hand_type = HandType::FiveOfAKind; }
        else { hand_type = HandType::FourOfAKind; }
    } else if card_counts[0] == &3 && card_counts[1] == &2 {
        if card_map.contains_key(&'J') { hand_type = HandType::FiveOfAKind; }
        else { hand_type = HandType::FullHouse; }
    } else if card_counts[0] == &3 {
        if card_map.contains_key(&'J') { hand_type = HandType::FourOfAKind; }
        else { hand_type = HandType::ThreeOfAKind; }
    } else if card_counts[0] == &2 && card_counts[1] == &2 {
        if card_map.contains_key(&'J') { 
            if card_map.get(&'J').unwrap() == &2 { hand_type = HandType::FourOfAKind; }
            else { hand_type = HandType::FullHouse; }
        }
        else { hand_type = HandType::TwoPair; }
    } else if card_counts[0] == &2 {
        if card_map.contains_key(&'J') { hand_type = HandType::ThreeOfAKind; }
        else { hand_type = HandType::OnePair; }
    } else if card_map.contains_key(&'J') { hand_type = HandType::OnePair; }
    hand_type
}

fn compare_hand_p1(hand_1: &Vec<char>, hand_2: &Vec<char>) -> Ordering {
    for x in 0..hand_1.len() {
        let card_1 = &hand_1[x];
        let card_2 = &hand_2[x];
        let card_comparison = compare_card_p1(card_1, card_2);
        if card_comparison != Ordering::Equal {
            return card_comparison;
        }
    }
    return Ordering::Equal;
}

fn compare_hand_p2(hand_1: &Vec<char>, hand_2: &Vec<char>) -> Ordering {
    for x in 0..hand_1.len() {
        let card_1 = &hand_1[x];
        let card_2 = &hand_2[x];
        let card_comparison = compare_card_p2(card_1, card_2);
        if card_comparison != Ordering::Equal {
            return card_comparison;
        }
    }
    return Ordering::Equal;
}

pub fn part_one(input: &str) -> Option<u32> {

    let regex = Regex::new(r"(?P<hand>.+) (?P<bid>.+)").unwrap();
    let mut lines = input.trim().split("\n")
        .map(|line| {
            let Some(captured) = regex.captures(line) else {
                panic!("Invalid line: {}", line)
            };
            let hand = &captured["hand"];
            let bid = captured["bid"].parse::<u32>().unwrap();
            let hand_cards = hand.chars().collect::<Vec<char>>();
            let mut card_map: HashMap<char, u8> = HashMap::new();
            for card in hand_cards.as_slice() {
                let count = card_map.entry(*card).or_insert(0);
                *count += 1;
            }
            let hand_type = hand_type_p1(&card_map);
            (hand_type, hand_cards, bid) 
        }).collect::<Vec<(HandType, Vec<char>, u32)>>();
    lines.sort_by(|a, b| {
        let hand_comparison = a.0.cmp(&b.0);
        if hand_comparison != Ordering::Equal { return hand_comparison; }
        return compare_hand_p2(&a.1, &b.1);
    });
    lines.reverse();
    let mut curr_rank = 1;
    let mut result = 0;
    for ele in lines {
        result += ele.2 * curr_rank;
        curr_rank += 1;
    }
    Some(result)
    
}

pub fn part_two(input: &str) -> Option<u32> {
    
    let regex = Regex::new(r"(?P<hand>.+) (?P<bid>.+)").unwrap();
    let mut lines = input.trim().split("\n")
        .map(|line| {
            let Some(captured) = regex.captures(line) else {
                panic!("Invalid line: {}", line)
            };
            let hand = &captured["hand"];
            let bid = captured["bid"].parse::<u32>().unwrap();
            let hand_cards = hand.chars().collect::<Vec<char>>();
            let mut card_map: HashMap<char, u8> = HashMap::new();
            for card in hand_cards.as_slice() {
                let count = card_map.entry(*card).or_insert(0);
                *count += 1;
            }
            let hand_type = hand_type_p2(&card_map);
            (hand_type, hand_cards, bid) 
        }).collect::<Vec<(HandType, Vec<char>, u32)>>();
    lines.sort_by(|a, b| {
        let hand_comparison = a.0.cmp(&b.0);
        if hand_comparison != Ordering::Equal { return hand_comparison; }
        return compare_hand_p2(&a.1, &b.1);
    });
    lines.reverse();
    let mut curr_rank = 1;
    let mut result = 0;
    for ele in lines {
        result += ele.2 * curr_rank;
        curr_rank += 1;
    }
    Some(result)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
