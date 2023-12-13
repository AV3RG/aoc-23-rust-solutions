use core::num;
use std::{char, collections::HashSet};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut array_2d: Vec<Vec<char>> = Vec::new();
    let lines: Vec<String> = input.trim().split("\n").map(|line| { line.to_string() }).collect();
    let count = lines.len();
    for line_no in 0..count  {
        array_2d.push(lines[line_no].chars().collect());
    }

    let valid_symbol_check = |c: char| -> bool {
        if c.is_numeric() { return false; }
        if c == '.' { return false; }
        return true;
    };

    let start_check_func = |line_no, char_no| -> bool {
        let checks: Vec<(i32, i32)> = vec![
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1), (0, 1),
        ];
        for check in checks {
            let line_check = line_no as i32 + check.1;
            let char_check = char_no as i32 + check.0;
            if line_check < 0 || char_check < 0 { continue; }
            if line_check >= array_2d.len() as i32 || char_check >= array_2d[line_check as usize].len() as i32 { continue; }
            if valid_symbol_check(array_2d[line_check as usize][char_check as usize]) { return true; }
        }
        return false;
    };

    let middle_check_fun = |line_no, char_no| -> bool {
        let checks: Vec<(i32, i32)> = vec![
            (0, -1), (0, 1),
        ];
        for check in checks {
            let line_check = line_no as i32 + check.1;
            let char_check = char_no as i32 + check.0;
            if line_check < 0 || char_check < 0 { continue; }
            if line_check >= array_2d.len() as i32 { continue; }
            if valid_symbol_check(array_2d[line_check as usize][char_check as usize]) { return true; }
        }
        return false;
    };

    let end_check_func = |line_no, char_no| -> bool {
        let checks: Vec<(i32, i32)> = vec![
            (1, -1), (1, 0), (1, 1),
            (0, -1), (0, 1),
        ];
        for check in checks {
            let line_check = line_no as i32 + check.1;
            let char_check = char_no as i32 + check.0;
            if line_check < 0 || char_check < 0 { continue; }
            if line_check >= array_2d.len() as i32 || char_check >= array_2d[line_check as usize].len() as i32 { continue; }
            if valid_symbol_check(array_2d[line_check as usize][char_check as usize]) { return true; }
        }
        return false;
    };

    let mut result: u32 = 0;

    for line_no in 0..array_2d.len() {
        let line_len = array_2d[line_no].len();
        let mut start_index: u32;
        let mut end_index: Option<u32> = None;
        for mut char_no in 0..line_len {
            if end_index.is_some() && (char_no as u32) <= end_index.unwrap() {
                continue;
            }
            let mut char_current = array_2d[line_no][char_no];
            if !char_current.is_numeric() { continue; }
            let mut adjacency_check: bool = false;
            start_index = char_no as u32;
            end_index = Some(char_no as u32);
            if !adjacency_check && start_check_func(line_no, char_no) { adjacency_check = true; }
            while char_current.is_numeric() {
                if !adjacency_check && middle_check_fun(line_no, char_no) { adjacency_check = true; }
                end_index = Some(char_no as u32);
                char_no += 1;
                if char_no >= line_len { break; }
                char_current = array_2d[line_no][char_no];
            }
            if !adjacency_check && end_check_func(line_no, end_index.unwrap()) { adjacency_check = true; }
            if !adjacency_check { continue; }
            let mut num: u32 = 0;
            for i in start_index..end_index.unwrap()+1 {
                num = num * 10 + char::to_digit(array_2d[line_no][i as usize], 10).unwrap();
            }
            result += num;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {

    let mut array_2d: Vec<Vec<char>> = Vec::new();
    let lines: Vec<String> = input.trim().split("\n").map(|line| { line.to_string() }).collect();
    let count = lines.len();
    for line_no in 0..count  {
        array_2d.push(lines[line_no].chars().collect());
    }

    let mut num_boxes: Vec<(u32, (u32, u32))> = Vec::new();

    for line_no in 0..array_2d.len() {
        let line_len = array_2d[line_no].len();
        let mut start_index: u32;
        let mut end_index: Option<u32> = None;
        for mut char_no in 0..line_len {
            if end_index.is_some() && (char_no as u32) <= end_index.unwrap() {
                continue;
            }
            let mut char_current = array_2d[line_no][char_no];
            if !char_current.is_numeric() { continue; }
            start_index = char_no as u32;
            end_index = Some(char_no as u32);
            while char_current.is_numeric() {
                end_index = Some(char_no as u32);
                char_no += 1;
                if char_no >= line_len { break; }
                char_current = array_2d[line_no][char_no];
            }
            num_boxes.push((line_no as u32, (start_index, end_index.unwrap())));
        }
    }

    let mut gear_locs: Vec<(u32, u32)> = Vec::new();

    for line_no in 0..array_2d.len() {
        let line_len = array_2d[line_no].len();
        for char_no in 0..line_len {
            if array_2d[line_no][char_no] == '*' { gear_locs.push((line_no as u32, char_no as u32)); }
        }
    }

    let gear_ratio = |(line_no, char_no)| -> Option<u32> {
        let checks: Vec<(i32, i32)> = vec![
            (-1,-1), (-1,0), (-1,1),
            (0, -1),         (0, 1),
            (1, -1), (1, 0), (1, 1)
        ];
        let mut num_box_set: HashSet<(u32, (u32, u32))> = HashSet::new();
        for ele in checks {
            let line_check_i32 = (line_no as i32) + ele.0;
            let char_check_i32 = (char_no as i32) + ele.1;
            if line_check_i32 < 0 || char_check_i32 < 0 { continue; };
            let line_check: usize = line_check_i32 as usize;
            let char_check: usize = char_check_i32 as usize;
            if (line_check >= array_2d.len()) || (char_check >= array_2d[line_check].len()) { continue; }
            for num_ele in num_boxes.as_slice() {
                if num_ele.0 != line_check as u32 { continue; }
                if !(char_check as u32 >= num_ele.1.0 && char_check as u32 <= num_ele.1.1) { continue; }
                num_box_set.insert(num_ele.to_owned());
            }
        }
        if num_box_set.len() != 2 { return None }
        else {
            let mut result: u32 = 1;
            for ele in num_box_set {
                let mut num: u32 = 0;
                for x in ele.1.0..(ele.1.1 + 1) {
                    num = (num * 10) + char::to_digit(array_2d[ele.0 as usize][x as usize], 10).unwrap();
                }
                result *= num;
            }
            return Some(result);
        }
    };

    let mut result: u32 = 0;
    for ele in gear_locs {
        result += gear_ratio(ele).unwrap_or(0);
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
