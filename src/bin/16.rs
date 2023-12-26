advent_of_code::solution!(16);

fn find_next_block_vertical_up(v: Vec<Vec<char>>, start: (usize, usize), initial: bool) -> Option<(usize, usize)> {
    if start.0 <= 0 { return None; }
    let offset = if initial { 1 } else { 0 };
    for y in (0..(start.0 + offset)).rev() {
        if v.get(y).is_none() { continue; }
        let symbol = v.get(y).unwrap().get(start.1).unwrap();
        if symbol == &'.' { continue; }
        if symbol == &'|' { continue; }
        return Some((y, start.1));
    }
    None
}

fn find_next_block_vertical_down(v: Vec<Vec<char>>, start: (usize, usize), initial: bool) -> Option<(usize, usize)> {
    if start.0 >= v.len() { return None; }
    let offset = if initial { 1 } else { 0 };
    for y in (start.0 + 1 - offset)..v.len() {
        if v.get(y).is_none() { continue; }
        let symbol = v.get(y).unwrap().get(start.1).unwrap();
        if symbol == &'.' { continue; }
        if symbol == &'|' { continue; }
        return Some((y, start.1));
    }
    None
}

fn find_next_block_horizontal_left(v: Vec<Vec<char>>, start: (usize, usize), initial: bool) -> Option<(usize, usize)> {
    if start.1 <= 0 { return None; }
    let offset = if initial { 1 } else { 0 };
    for x in (0..(start.1 + offset)).rev() {
        if v.get(start.0).unwrap().get(x).is_none() { continue; }
        let symbol = v.get(start.0).unwrap().get(x).unwrap();
        if symbol == &'.' { continue; }
        if symbol == &'-' { continue; }
        return Some((start.0, x));
    }
    None
}

fn find_next_block_horizontal_right(v: Vec<Vec<char>>, start: (usize, usize), initial: bool) -> Option<(usize, usize)> {
    if start.1 >= v.get(start.0).unwrap().len() { return None; }
    let offset = if initial { 1 } else { 0 };
    for x in (start.1 + 1 - offset)..v.get(start.0).unwrap().len() {
        if v.get(start.0).unwrap().get(x).is_none() { continue; }
        let symbol = v.get(start.0).unwrap().get(x).unwrap();
        if symbol == &'.' { continue; }
        if symbol == &'-' { continue; }
        return Some((start.0, x));
    }
    None
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn calc_covered(array_2d: Vec<Vec<char>>, intial_ray: ((usize, usize), Direction, bool)) -> u32 {
    let mut array_2d_copy = array_2d.clone().iter().map(|f| f.clone().iter().map(|_| ' ').collect()).collect::<Vec<Vec<char>>>();
    let mut covered_paths: Vec<((usize, usize), (usize, usize))> = Vec::new();
    let mut rays_pending: Vec<((usize, usize), Direction, bool)> = Vec::new();
    rays_pending.push(intial_ray);
    while !rays_pending.is_empty() {
        let ray = rays_pending.pop().unwrap();
        let mut next_block: Option<(usize, usize)> = None;
        match ray.1 {
            Direction::Up => next_block = find_next_block_vertical_up(array_2d.clone(), ray.0, ray.2),
            Direction::Down => next_block = find_next_block_vertical_down(array_2d.clone(), ray.0, ray.2),
            Direction::Left => next_block = find_next_block_horizontal_left(array_2d.clone(), ray.0, ray.2),
            Direction::Right => next_block = find_next_block_horizontal_right(array_2d.clone(), ray.0, ray.2),
        }
        if next_block.is_none() {
            match ray.1 {
                Direction::Up => covered_paths.push((ray.0, (0, ray.0.1))),
                Direction::Down => covered_paths.push((ray.0, (array_2d.len() - 1, ray.0.1))),
                Direction::Left => covered_paths.push((ray.0, (ray.0.0, 0))),
                Direction::Right => covered_paths.push((ray.0, (ray.0.0, array_2d.get(ray.0.0).unwrap().len() - 1))),
            }
            continue;
        }
        let next_block = next_block.unwrap();
        let already_covered = covered_paths.iter().any(|path| {
            (path.0 == ray.0 && path.1 == next_block) || (path.1 == ray.0 && path.0 == next_block) 
        });
        if already_covered { continue; }
        covered_paths.push((ray.0, next_block));
        let next_block_symbol = array_2d.get(next_block.0).unwrap().get(next_block.1).unwrap();
        if (ray.1 == Direction::Up || ray.1 == Direction::Down) && next_block_symbol == &'-' { 
            rays_pending.push((next_block, Direction::Left, false));
            rays_pending.push((next_block, Direction::Right, false));
            continue;
        }
        if (ray.1 == Direction::Left || ray.1 == Direction::Right) && next_block_symbol == &'|' { 
            rays_pending.push((next_block, Direction::Up, false));
            rays_pending.push((next_block, Direction::Down, false));
            continue;
        }
        if next_block_symbol == &'/' {
            match ray.1 {
                Direction::Up => rays_pending.push((next_block, Direction::Right, false)),
                Direction::Down => rays_pending.push((next_block, Direction::Left, false)),
                Direction::Left => rays_pending.push((next_block, Direction::Down, false)),
                Direction::Right => rays_pending.push((next_block, Direction::Up, false)),
            }
            continue;
        }
        if next_block_symbol == &'\\' {
            match ray.1 {
                Direction::Up => rays_pending.push((next_block, Direction::Left, false)),
                Direction::Down => rays_pending.push((next_block, Direction::Right, false)),
                Direction::Left => rays_pending.push((next_block, Direction::Up, false)),
                Direction::Right => rays_pending.push((next_block, Direction::Down, false)),
            }
            continue;
        }
    }
    let mut result: u32 = 0;
    for path in covered_paths {
        let (start, end) = if (path.0.0 <= path.1.0 && path.0.1 <= path.1.1) { (path.0, path.1) } else { (path.1, path.0) };
        for x in start.0..=end.0 {
            for y in start.1..=end.1 {
                let val = array_2d_copy.get_mut(x).unwrap().get_mut(y).unwrap();
                if *val == '#' { continue; }
                *val = '#';
                result += 1;
            }
        }
    }
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let array_2d = input.trim().split("\n").map(|f| f.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let initial_ray = ((0, 0), Direction::Right, true);
    Some(calc_covered(array_2d, initial_ray))
}

pub fn part_two(input: &str) -> Option<u32> {
    let array_2d = input.trim().split("\n").map(|f| f.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut max: u32 = 0;
    let mut initial_rays: Vec<((usize, usize), Direction, bool)> = Vec::new();
    for x in 0..array_2d[0].len() {
        initial_rays.push(((0, x), Direction::Down, true));
        initial_rays.push(((array_2d.len() - 1, x), Direction::Up, true));
    }
    for y in 0..array_2d.len() {
        initial_rays.push(((y, 0), Direction::Right, true));
        initial_rays.push(((y, array_2d[0].len() - 1), Direction::Left, true));
    }
    initial_rays.iter().for_each(|ray| {
        let result = calc_covered(array_2d.clone(), *ray);
        if result > max { max = result; }
    });
    Some(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
