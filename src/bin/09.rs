advent_of_code::solution!(9);

fn generate_lower(nums: &Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for x in 0..(nums.len()-1) {
        result.push(nums[x+1]-nums[x]);
    }
    result
}

fn is_all_zeroes(nums: &Vec<i32>) -> bool {
    for x in nums {
        if x.to_owned() != 0 {
            return false;
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<i32> {
    
    fn add_upper_extra(lower: &Vec<i32>, upper: &mut Vec<i32>) {
        upper.push(upper.last().unwrap() + lower.last().unwrap());
    }

    let mut result: i32 = 0;
    input.trim().split('\n').for_each(|line| {
        let nums = line.split(' ').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let mut stack: Vec<Vec<i32>> = Vec::new();
        stack.push(nums.clone());
        let mut current: Vec<i32> = generate_lower(&nums);
        stack.push(current.clone());
        while !is_all_zeroes(&current) {
            current = generate_lower(&current);
            stack.push(current.clone());
        }
        stack.reverse();
        for x in 0..stack.len() - 1 {
            add_upper_extra(&stack.clone()[x], &mut stack[x+1]);
        }
        result += stack.last().unwrap().last().unwrap();
    });
    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    fn add_upper_extra(lower: &Vec<i32>, upper: &mut Vec<i32>) {
        upper.insert(0, upper.first().unwrap() - lower.first().unwrap());
    }

    let mut result: i32 = 0;
    input.trim().split('\n').for_each(|line| {
        let nums = line.split(' ').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let mut stack: Vec<Vec<i32>> = Vec::new();
        stack.push(nums.clone());
        let mut current: Vec<i32> = generate_lower(&nums);
        stack.push(current.clone());
        while !is_all_zeroes(&current) {
            current = generate_lower(&current);
            stack.push(current.clone());
        }
        stack.reverse();
        for x in 0..stack.len() - 1 {
            add_upper_extra(&stack.clone()[x], &mut stack[x+1]);
        }
        result += stack.last().unwrap().first().unwrap();
    });
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
