use std::collections::HashMap;

advent_of_code::solution!(14);

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn find_next_block(v: Vec<char>, start: usize) -> Option<usize> {
    for x in start..v.len() {
        let symbol = v.get(x).unwrap();
        if symbol == &'.' { continue; }
        return Some(x);
    }
    None
}

fn tilt_left(v: Vec<char>) -> Vec<char> {
    let mut result = v.clone();
    for x in 0..v.len() {
        let symbol = result.get(x).unwrap();
        if symbol != &'.' { continue; }
        let next_block_option = find_next_block(result.clone(), x+1);
        if next_block_option.is_none() { continue; }
        let next_block = result.get(next_block_option.unwrap()).unwrap();
        if next_block == &'O' { 
            result.swap(next_block_option.unwrap(), x); 
        }
    }
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = input.trim().split("\n").map(|f| f.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut tiltable_map = transpose(map.clone());
    tiltable_map = tiltable_map.iter().map(|f| tilt_left(f.clone())).collect::<Vec<Vec<char>>>();
    let mut result: u32 = 0;
    for ele in tiltable_map {
        for x in 0..ele.len() {
            let symbol = ele.get(x).unwrap();
            if symbol != &'O' { continue; }
            result += (ele.len() - x) as u32;
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = input.trim().split("\n").map(|f| f.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut tiltable_map = map.clone();
    let mut cache: HashMap<Vec<Vec<char>>, Vec<Vec<char>>> = HashMap::new();
    let cache_handle = |map: Vec<Vec<char>>, remaining: i32, cache: HashMap<Vec<Vec<char>>, Vec<Vec<char>>>| -> Vec<Vec<char>> {
        let mut cache_loop: Vec<Vec<Vec<char>>> = Vec::new();
        cache_loop.push(map.clone());
        let mut next = cache.clone().get(&map).unwrap().clone();
        while next != map {
            cache_loop.push(next.clone());
            next = cache.clone().get(&next).unwrap().clone();
        }
        let mut result = cache_loop.get((remaining % cache_loop.len() as i32) as usize).unwrap().clone();
        result
    };
    for p in 0..1000000000 {
        if cache.contains_key(&tiltable_map) {
            tiltable_map = cache_handle(tiltable_map, 1000000000 - p, cache.clone());
            break;   
        }
        //north tilt
        let mut north_titler = transpose(tiltable_map.clone());
        north_titler = north_titler.iter().map(|f| tilt_left(f.clone())).collect::<Vec<Vec<char>>>();
        north_titler = transpose(north_titler);
        //west tilt
        north_titler = north_titler.iter().map(|f| tilt_left(f.clone())).collect::<Vec<Vec<char>>>();
        //south tilt
        north_titler = transpose(north_titler);
        north_titler = north_titler.iter().map(|f| {
            let mut reversed = f.clone();
            reversed.reverse();

            reversed = tilt_left(reversed);
            reversed.reverse();
            return reversed;
        }).collect::<Vec<Vec<char>>>();
        north_titler = transpose(north_titler);
        //east tilt
        north_titler = north_titler.iter().map(|f| {
            let mut reversed = f.clone();
            reversed.reverse();

            reversed = tilt_left(reversed);
            reversed.reverse();
            return reversed;
        }).collect::<Vec<Vec<char>>>();
        if tiltable_map == north_titler { break; }
        cache.insert(tiltable_map.clone(), north_titler.clone());
        tiltable_map = north_titler;
    }
    tiltable_map = transpose(tiltable_map);
    let mut result: u32 = 0;
    for ele in tiltable_map {
        for x in 0..ele.len() {
            let symbol = ele.get(x).unwrap();
            if symbol != &'O' { continue; }
            result += (ele.len() - x) as u32;
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
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
