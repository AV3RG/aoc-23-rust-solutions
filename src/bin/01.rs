use std::collections::HashMap;
use std::str::Chars;
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {

    fn extract_nums(input: Chars) -> u32 {
        let mut num1: Option<u32> = None;
        let mut num2: u32 = 0;
        input.for_each(|c| {
            if c.is_numeric() {
                num2 = c.to_digit(10).unwrap();
                if num1 == None {
                    num1 = Some(num2);
                }
            }
        });
        if num1.is_none() { num1 = Some(0); }
        num1.unwrap() * 10 + num2
    }

    let mut sum: u32 = 0;
    input.split('\n')
        .map(|line| extract_nums(line.chars()))
        .for_each(|num| sum += num);
    Some(sum)

}

pub fn part_two(input: &str) -> Option<u32> {

    let mut word_text_map: HashMap<String, u32> = HashMap::new();
    word_text_map.insert("one".to_owned(), 1);
    word_text_map.insert("two".to_owned(), 2);
    word_text_map.insert("three".to_owned(), 3);
    word_text_map.insert("four".to_owned(), 4);
    word_text_map.insert("five".to_owned(), 5);
    word_text_map.insert("six".to_owned(), 6);
    word_text_map.insert("seven".to_owned(), 7);
    word_text_map.insert("eight".to_owned(), 8);
    word_text_map.insert("nine".to_owned(), 9);
    for x in 1..10 {
        let x_str = String::from(x.to_string());
        word_text_map.insert(x_str, x);
    }
    let mut sum: u32 = 0;
    let extract_nums = |input: &str| -> u32 {
        let mut num1: Option<u32> = None;
        let mut num2: u32 = 0;
        let mut num1_index: i32 = i32::MAX;
        let mut num2_index: i32 = i32::MIN;
        word_text_map.iter().for_each(|(k, v)| {
            let matched: Vec<_> = input.match_indices(k).collect();
            if matched.len() == 0 { return; };
            let (start, _) = matched[0];
            let (end, _) = matched[matched.len() - 1];
            if (start as i32) < num1_index {
                num1_index = start as i32;
                num1 = Some(*v);
            }
            if (end as i32) > num2_index {
                num2_index = end as i32;
                num2 = *v;
            }
        });
        if num1.is_none() { num1 = Some(0); }
        num1.unwrap() * 10 + num2
    };

    input.split('\n')
        .map(|line| { extract_nums(line) })
        .for_each(|num| sum += num);
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
