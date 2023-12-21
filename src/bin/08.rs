use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let regex = Regex::new(r"(?P<moves>.+)\n\n(?P<nodes>(.|\n)+)").unwrap();
    let Some(captured) = regex.captures(input.trim()) else {
        panic!("Invalid input: {}", input);
    };
    let moves = &captured["moves"];
    let node_regex = Regex::new(r"(?P<base>.+) = \((?P<left>.+), (?P<right>.+)\)").unwrap();
    let nodes = captured["nodes"].split("\n").map(|f| f.to_string()).collect::<Vec<String>>();
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for ele in nodes {
        let Some(node_captured) = node_regex.captures(&ele) else {
            panic!("Invalid input: {}", ele);
        };
        let base = node_captured["base"].to_owned();
        let left = node_captured["left"].to_owned();
        let right = node_captured["right"].to_owned();
        map.insert(base, (left, right));
    };

    let mut current = "AAA".to_string();
    let mut repitition: u32 = 0;
    loop {
        moves.chars().for_each(|c| {
            let (left, right) = map.get(&current).unwrap();
            if c == 'L' {
                current = left.to_owned();
            } else {
                current = right.to_owned();
            }
            repitition += 1;
        });
        if current == "ZZZ" {
            break;
        }
    }
    Some(repitition)
}

pub fn part_two(input: &str) -> Option<u64> {
    let regex = Regex::new(r"(?P<moves>.+)\n\n(?P<nodes>(.|\n)+)").unwrap();
    let Some(captured) = regex.captures(input.trim()) else {
        panic!("Invalid input: {}", input);
    };
    let moves = &captured["moves"];
    let node_regex = Regex::new(r"(?P<base>.+) = \((?P<left>.+), (?P<right>.+)\)").unwrap();
    let nodes = captured["nodes"].split("\n").map(|f| f.to_string()).collect::<Vec<String>>();
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for ele in nodes {
        let Some(node_captured) = node_regex.captures(&ele) else {
            panic!("Invalid input: {}", ele);
        };
        let base = node_captured["base"].to_owned();
        let left = node_captured["left"].to_owned();
        let right = node_captured["right"].to_owned();
        map.insert(base, (left, right));
    };

    let to_calc =  map.keys().filter(|key| {
        key.ends_with("A")
    }).cloned().map(|start| {
        let mut current = start;
        let mut repitition: u64 = 0;
        loop {
            moves.chars().for_each(|c| {
                let (left, right) = map.get(&current).unwrap();
                if c == 'L' {
                    current = left.to_owned();
                } else {
                    current = right.to_owned();
                }
                repitition += 1;
            });
            if current.ends_with("Z") {
                break;
            }
        }
        repitition
    }).collect::<Vec<u64>>();

    fn lcm(first: u64, second: u64) -> u64 {
        first * second / gcd(first, second)
    }
    
    fn gcd(first: u64, second: u64) -> u64 {
        let mut max = first;
        let mut min = second;
        if min > max {
            let val = max;
            max = min;
            min = val;
        }
    
        loop {
            let res = max % min;
            if res == 0 {
                return min;
            }
    
            max = min;
            min = res;
        }
    }

    let mut result: u64 = to_calc[0];
    for x in 1..to_calc.len() {
        result = lcm(result, to_calc[x]);
    }
    Some(result)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(6));
    }
}
