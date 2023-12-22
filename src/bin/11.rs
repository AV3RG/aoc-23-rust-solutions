advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let mut array_2d: Vec<Vec<char>> = Vec::new();
    input.trim().split('\n').for_each(|line| {
        array_2d.push(line.chars().collect::<Vec<char>>());
    });

    let mut horizontal_empty: Vec<u32> = Vec::new();
    let mut vertical_empty: Vec<u32> = Vec::new();

    for x in 0..array_2d.len() {
        if array_2d[x].iter().all(|c| c == &'.') {
            horizontal_empty.push(x as u32);
        }
    }

    for x in 0..array_2d[0].len() {
        let mut empty = true;
        for y in 0..array_2d.len() {
            if array_2d[y][x] != '.' {
                empty = false;
                break;
            }
        }
        if empty {
            vertical_empty.push(x as u32);
        }
    }
    let mut galaxies: Vec<(i32, i32)> = Vec::new();
    for x in 0..array_2d.len() {
        for y in 0..array_2d[0].len() {
            if array_2d[x][y] == '#' {
                galaxies.push((x as i32, y as i32));
            }
        }
    }
    let mut result: u32 = 0;
    for x in 0..galaxies.len()-1 {
        for y in x+1..galaxies.len() {
            let (x1, y1) = galaxies[x];
            let (x2, y2) = galaxies[y];
            let max_x = (if x1 > x2 { x1 } else { x2 }) as u32;
            let min_x = (if x1 < x2 { x1 } else { x2 }) as u32;
            let max_y = (if y1 > y2 { y1 } else { y2 }) as u32;
            let min_y = (if y1 < y2 { y1 } else { y2 }) as u32;
            let h_empty = horizontal_empty.iter().filter(|horizontal| {
                **horizontal >= min_x && **horizontal <= max_x
            }).count();
            let v_empty = vertical_empty.iter().filter(|vertical| {
                **vertical >= min_y && **vertical <= max_y
            }).count();
            result += (max_x - min_x) + (max_y - min_y) + h_empty as u32 + v_empty as u32;
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut array_2d: Vec<Vec<char>> = Vec::new();
    input.trim().split('\n').for_each(|line| {
        array_2d.push(line.chars().collect::<Vec<char>>());
    });

    let mut horizontal_empty: Vec<u32> = Vec::new();
    let mut vertical_empty: Vec<u32> = Vec::new();

    for x in 0..array_2d.len() {
        if array_2d[x].iter().all(|c| c == &'.') {
            horizontal_empty.push(x as u32);
        }
    }

    for x in 0..array_2d[0].len() {
        let mut empty = true;
        for y in 0..array_2d.len() {
            if array_2d[y][x] != '.' {
                empty = false;
                break;
            }
        }
        if empty {
            vertical_empty.push(x as u32);
        }
    }
    let mut galaxies: Vec<(i32, i32)> = Vec::new();
    for x in 0..array_2d.len() {
        for y in 0..array_2d[0].len() {
            if array_2d[x][y] == '#' {
                galaxies.push((x as i32, y as i32));
            }
        }
    }
    let mut result: u64 = 0;
    let large_factor = (1000000 - 1) as u64;
    for x in 0..galaxies.len()-1 {
        for y in x+1..galaxies.len() {
            let (x1, y1) = galaxies[x];
            let (x2, y2) = galaxies[y];
            let max_x = (if x1 > x2 { x1 } else { x2 }) as u32;
            let min_x = (if x1 < x2 { x1 } else { x2 }) as u32;
            let max_y = (if y1 > y2 { y1 } else { y2 }) as u32;
            let min_y = (if y1 < y2 { y1 } else { y2 }) as u32;
            let h_empty = horizontal_empty.iter().filter(|horizontal| {
                **horizontal >= min_x && **horizontal <= max_x
            }).count();
            let v_empty = vertical_empty.iter().filter(|vertical| {
                **vertical >= min_y && **vertical <= max_y
            }).count();
            result += (max_x - min_x) as u64 + (max_y - min_y) as u64 + (h_empty as u64 * large_factor) + (v_empty as u64 * large_factor);
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
