use std::collections::HashMap;
use regex::Regex;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    
    let mut limits: HashMap<String, u32> = HashMap::new();
    limits.insert("red".to_string(), 12);
    limits.insert("green".to_string(), 13);
    limits.insert("blue".to_string(), 14);

    let set_is_possible = |line: &str| -> bool {
        let mut is_possible: bool = true;
        for color_data in line.split(",") {
            if !is_possible { break; }
            let mut color_balls = color_data.trim().split(" ");
            let num_option = color_balls.next();
            let color_option = color_balls.next();
            if num_option.is_none() || color_option.is_none() {
                panic!("Invalid input! WTF");
            }
            let num = num_option.unwrap();
            let color = color_option.unwrap();
            let limit = limits.get(color);
            if limit.is_none() { panic!("Invalid color of ball! WTF"); }
            let limit = limit.unwrap();
            if num.parse::<u32>().unwrap() > *limit {
                is_possible = false;
                return false;
            }
        }
        is_possible
    };

    let line_is_possible = |line: &str| -> bool {
        let mut is_possible: bool = true;
        for color_data in line.split(";") {
            if !is_possible { break; }
            is_possible = set_is_possible(color_data);
        }
        is_possible
    };

    let mut sum: u32 = 0;
    let day_regex = Regex::new(r"Game (?<day>\d+): (?<balls>.+)").unwrap();
    input.split("\n")
        .for_each(|day| {
            if day.is_empty() { return; }
            let Some(captures) = day_regex.captures(day) else {
                panic!("Invalid input line: {}", day);
            };
            let day = &captures["day"];
            let balls = &captures["balls"];
            
            if line_is_possible(balls) {
                sum += day.parse::<u32>().unwrap();
            }

        });
    Some(sum)

}

pub fn part_two(input: &str) -> Option<u32> {
    
    let line_min_balls = |line: &str| -> u32 {
        let mut min_balls: HashMap<&str, u32> = HashMap::new();
        line.split(";").for_each(|set| {
            set.split(",").for_each(|ball_data| {
                let mut color_balls = ball_data.trim().split(" ");
                let num_option = color_balls.next();
                let color_option = color_balls.next();
                if num_option.is_none() || color_option.is_none() {
                    panic!("Invalid input! WTF");
                }
                let num = num_option.unwrap();
                let color = color_option.unwrap();
                let limit = min_balls.get(color);
                if limit.is_none() { 
                    min_balls.insert(color, num.parse::<u32>().unwrap());
                } else {
                    let limit = limit.unwrap();
                    if num.parse::<u32>().unwrap() > *limit {
                        min_balls.insert(color, num.parse::<u32>().unwrap());
                    }
                }
            })
        });
        let mut ans: u32 = 1;
        min_balls.iter().for_each(|(_, v)| ans *= v);
        ans
    };


    let mut sum: u32 = 0;
    let day_regex = Regex::new(r"Game (?<day>\d+): (?<balls>.+)").unwrap();
    input.split("\n")
        .for_each(|day| {
            if day.is_empty() { return; }
            let Some(captures) = day_regex.captures(day) else {
                panic!("Invalid input line: {}", day);
            };
            let balls = &captures["balls"];
            sum += line_min_balls(balls);
        });
    Some(sum)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
