use std::collections::HashMap;
use regex::Regex;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    input.trim().split(",").for_each(|f| {
        let mut hashed: u32 = 0;
        f.chars().for_each(|c| {
            hashed += c as u32;
            hashed *= 17;
            hashed %= 256;
        });
        result += hashed;
    });
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map: HashMap<u8, Vec<(String, u8)>> = HashMap::new();
    let mut regex = Regex::new(r"(?P<to_hash>[a-z]+)(?P<symbol>[=-])(?P<lens>\d)?").unwrap();
    input.trim().split(",").for_each(|f| {
        let Some(captured) = regex.captures(f.trim()) else { panic!("Invalid input") };
        let to_hash = captured.name("to_hash").unwrap().as_str().to_string();
        let hashed = to_hash.chars().fold(0, |mut acc, c| {
            acc += c as u32;
            acc *= 17;
            acc %= 256;
            acc
        }) as u8;
        let symbol = captured.name("symbol").unwrap().as_str().chars().next().expect("Symbol empty");
        let lens_option = captured.name("lens").map(|f| f.as_str().to_string().parse::<u8>().unwrap());
        if symbol == '-' {
            if !map.contains_key(&hashed) { return; }
            let mut vec = map.get(&hashed).unwrap().clone();
            let mut index: Option<usize> = None;
            vec.iter().enumerate().for_each(|(i, (key, _))| {
                if key == &to_hash { index = Some(i); }
            });
            if index.is_none() { return; }
            vec.remove(index.unwrap());
            map.insert(hashed, vec);
        }
        if symbol == '=' {
            let mut vec: Vec<(String, u8)> = if !map.contains_key(&hashed) { Vec::new() } else { map.get(&hashed).unwrap().clone() };
            let lens = lens_option.expect("Lens empty");
            let mut index: Option<usize> = None;
            vec.iter().enumerate().for_each(|(i, (key, _))| {
                if key == &to_hash { index = Some(i); }
            });
            if index.is_some() { 
                vec[index.unwrap()] = (to_hash, lens); 
            } else {
                vec.push((to_hash, lens));
            }
            map.insert(hashed, vec);
        }
    });
    let mut result: u32 = 0;
    map.iter().for_each(|(key, lens)| {
        lens.iter().enumerate().for_each(|(j, (to_hash, lens_focal))| {
            result += ((*key as u32) + 1) as u32 * (*lens_focal as u32) * (j as u32 + 1);
        });
    });
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
