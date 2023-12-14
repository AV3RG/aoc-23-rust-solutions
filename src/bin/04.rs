use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let day_regex = Regex::new(r"Card\s+\d+: (?<winning>.+) \| (?<owned>.+)").unwrap();
    let mut sum = 0;
    input.split("\n")
        .for_each(|day| {
            if day.is_empty() { return; }
            let Some(captures) = day_regex.captures(day) else {
                panic!("Invalid input line: {}", day);
            };
            let winning_str = (&captures["winning"]).trim();
            let owned_str = (&captures["owned"]).trim();
            
            let split_regex = Regex::new(r"\s+").unwrap();
            let winning = split_regex.split(winning_str)
                .map(|x| x.to_string())
                .map(|card| {
                    return card.parse::<u32>().unwrap();
                }).collect::<Vec<u32>>();

            let owned = split_regex.split(owned_str)
                .map(|x| x.to_string())
                .map(|card| {
                    return card.parse::<u32>().unwrap();
                }).collect::<Vec<u32>>();

            let count = owned.iter().filter(|owned_card| {
                winning.contains(owned_card)
            }).count();
            if count == 0 { return; }
            let base: u32 = 2;
            sum += base.pow((count - 1) as u32);
        });
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let day_regex = Regex::new(r"Card\s+(?<card_no>\d+): (?<winning>.+) \| (?<owned>.+)").unwrap();
    let mut card_copies: HashMap<u32, u32> = HashMap::new();
    input.split("\n")
        .for_each(|day| {
            if day.is_empty() { return; }
        
            let Some(captures) = day_regex.captures(day) else {
                panic!("Invalid input line: {}", day);
            };
            let card_no: u32 = (&captures["card_no"]).trim().parse::<u32>().unwrap();
            card_copies.insert(card_no, card_copies.get(&card_no).unwrap_or(&0).to_owned() + 1);
            let winning_str = (&captures["winning"]).trim();
            let owned_str = (&captures["owned"]).trim();
            
            let split_regex = Regex::new(r"\s+").unwrap();
            let winning = split_regex.split(winning_str)
                .map(|x| x.to_string())
                .map(|card| {
                    return card.parse::<u32>().unwrap();
                }).collect::<Vec<u32>>();

            let owned = split_regex.split(owned_str)
                .map(|x| x.to_string())
                .map(|card| {
                    return card.parse::<u32>().unwrap();
                }).collect::<Vec<u32>>();

            let count = owned.iter().filter(|owned_card| {
                winning.contains(owned_card)
            }).count();

            let binding = card_copies.to_owned();
            let repeat = binding.get(&card_no).unwrap();

            for x in 1..count+1  {
                let mut existing = card_copies.to_owned().get(&(card_no + x as u32)).unwrap_or(&0).to_owned();
                existing += repeat;
                if existing == 0 { continue; }
                card_copies.insert(card_no + x as u32, existing);
            }
        });
    let mut sum = 0;
    for ele in card_copies.into_iter() {
        sum += ele.1;
    }
    Some(sum)
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
