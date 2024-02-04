use regex::Regex;
advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<i32> {
    let mut vertices: Vec<(i32, i32)> = Vec::new();
    let mut current: (i32, i32) = (0, 0);
    let line_regex = Regex::new(r"(?P<direction>.) (?P<amount>.+) \(#(?P<color>[0-9a-fA-F]+)\)").unwrap();
    input.trim().split("\n").for_each(|line| {
        vertices.push(current);
        let Some(captured) = line_regex.captures(line) else {
            panic!("Invalid line: {}", line);
        };
        let direction = captured["direction"].to_string().chars().next().expect("No direction found");
        let amount = captured["amount"].to_string().parse::<i32>().unwrap();

        match direction {
            'R' => { current.0 += amount }
            'L' => { current.0 -= amount }
            'U' => { current.1 -= amount }
            'D' => { current.1 += amount }
            _ => {}
        }
    });
    /* Applying shoelace algorithm */
    vertices.push(vertices.first().unwrap().to_owned());
    let mut area: i32 = 0;
    let mut peri: i32 = 0;
    for i in 0..vertices.len() - 1 {
        let first = vertices.get(i).unwrap();
        let second = vertices.get(i + 1).unwrap();
        area += (first.1 * second.0) - (first.0 * second.1);
        peri += (first.0 - second.0).abs() + (first.1 - second.1).abs();
    }
    Some(((area.abs() + peri) / 2) + 1)
}

pub fn part_two(input: &str) -> Option<i128> {
    let mut vertices: Vec<(i128, i128)> = Vec::new();
    let mut current: (i128, i128) = (0, 0);
    let line_regex = Regex::new(r"(?P<direction>.) (?P<amount>.+) \(#(?P<color>[0-9a-fA-F]+)\)").unwrap();
    input.trim().split("\n").for_each(|line| {
        vertices.push(current);
        let Some(captured) = line_regex.captures(line) else {
            panic!("Invalid line: {}", line);
        };
        let color_str = captured["color"].to_string();
        let amount: i128 = i128::from_str_radix(&color_str[0..5], 16).unwrap();
        match &color_str[5..6] {
            "0" => { current.0 += amount }
            "2" => { current.0 -= amount }
            "3" => { current.1 -= amount }
            "1" => { current.1 += amount }
            _ => {panic!("Invalid direction")}
        }
    });
    /* Applying shoelace algorithm */
    vertices.push(vertices.first().unwrap().to_owned());
    let mut area: i128 = 0;
    let mut peri: i128 = 0;
    for i in 0..vertices.len() - 1 {
        let first = vertices.get(i).unwrap();
        let second = vertices.get(i + 1).unwrap();
        area += (first.1 * second.0) - (first.0 * second.1);
        peri += (first.0 - second.0).abs() + (first.1 - second.1).abs();
    }
    Some(((area.abs() + peri) / 2) + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
