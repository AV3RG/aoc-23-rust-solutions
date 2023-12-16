use regex::Regex;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let regex = Regex::new(r"Time:\s+(?P<time>.+)\nDistance:\s+(?P<distance>.+)").unwrap();
    let Some(captured) = regex.captures(input) else {
        panic!("Invalid input: {}", input);
    };
    let split_regex = Regex::new(r"\s+").unwrap();
    let times = split_regex.split(&captured["time"]).map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let distances = split_regex.split(&captured["distance"]).map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let mut result = 1;
    for x in 0..times.len() {
        let time = times[x];
        let distance = distances[x];
        let determinant = (time.pow(2) as i32) - (4 * distance as i32);
        if determinant < 0 { continue; }
        let root = (determinant as f32).sqrt();
        let alpha = ((time as f32) + root) / 2.0;
        let beta = ((time as f32) - root) / 2.0;
        let mut alpha_u32 = alpha as u32;
        let beta_u32 = beta as u32;
        if alpha_u32 as f32 == alpha { alpha_u32 -= 1 };
        result *= alpha_u32 - beta_u32;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let regex = Regex::new(r"Time:\s+(?P<time>.+)\nDistance:\s+(?P<distance>.+)").unwrap();
    let Some(captured) = regex.captures(input) else {
        panic!("Invalid input: {}", input);
    };
    let split_regex = Regex::new(r"\s+").unwrap();
    let time: u64 = split_regex.replace_all(&captured["time"], "").parse::<u64>().unwrap();
    let distance: u64 = split_regex.replace_all(&captured["distance"], "").parse::<u64>().unwrap();
    let determinant = (time.pow(2) as i64) - ((4 as i64) * (distance as i64));
    if determinant < 0 { return Some(0); }
    let root = (determinant as f64).sqrt();
    let alpha = ((time as f64) + root) / 2.0;
    let beta = ((time as f64) - root) / 2.0;
    let mut alpha_u32 = alpha as u64;
    let beta_u32 = beta as u64;
    if alpha_u32 as f64 == alpha { alpha_u32 -= 1 };
    Some(alpha_u32 - beta_u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
