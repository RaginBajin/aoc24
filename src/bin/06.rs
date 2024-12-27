advent_of_code::solution!(6);
use std::collections::HashSet;


fn find_starting_spot(map: &Map) -> ((usize, usize), char) {
    let directions = ['^', '>', 'v', '<'];
    let mut starting_spot = ((0, 0), ' ');
    for (row_index, row) in map.mapdata.iter().enumerate() {
        for (col_index, &value) in row.iter().enumerate() {
            if directions.contains(&value) {
                starting_spot = ((row_index, col_index), value);
            }
        }
    }
    starting_spot
}

fn unique_count(vec: Vec<(usize, usize)>) -> usize {
    let unique_set: HashSet<_> = vec.into_iter().collect(); // Convert Vec to HashSet
    unique_set.len() // Return the count of unique elements
}


fn look_right(map: &Map, current_location: (usize, usize), direction: char) -> Option<((usize, usize), char)> {
    let (dx, dy, new_dir) = match direction {
        '^' => (0, 1, '>'),
        '>' => (1, 0, 'v'),
        'v' => (0, -1, '<'),
        '<' => (-1, 0, '^'),
        _ => return None,
    };
    let new_location = (
        (current_location.0 as isize + dx) as usize,
        (current_location.1 as isize + dy) as usize,
    );
    if map.mapdata[new_location.0][new_location.1] == '#' {
        None
    } else {
        Some((new_location, new_dir))
    }
}

fn next_step(map: &mut Map) -> Option<((usize, usize), char)> {
    // Get the current location and direction
    let current = *map.current_location.last().unwrap();
    let move_where = match map.current_direction {
        '^' => (-1, 0),
        '>' => (0, 1),
        'v' => (1, 0),
        '<' => (0, -1),
        _ => return None,
    };

    // Calculate the next step
    let mut check_step = (
        (current.0 as isize + move_where.0) as usize,
        (current.1 as isize + move_where.1) as usize,
    );

    // Check bounds
    if check_step.0 >= map.rows || check_step.1 >= map.cols {
        return None;
    }

    // Check for walls
    if map.mapdata[check_step.0][check_step.1] == '#' {
        // Check for Right Turn, but use the current location
        match look_right(map, *map.current_location.last().unwrap(), map.current_direction) {
            Some(new_spot) =>  {
                check_step = new_spot.0;
                map.current_direction = new_spot.1;
            },
            None => {
                return None;
            }
        }
    }

    Some((check_step, map.current_direction))
}

#[derive(Debug, Clone)]
struct Map {
    mapdata: Vec<Vec<char>>, // Updated type
    rows: usize,
    cols: usize,
    current_location: Vec<(usize, usize)>,
    current_direction: char,
}

impl Map {
    // Constructor
    fn new(mapdata: Vec<&str>) -> Self {
        let rows = mapdata.len();
        let cols = if rows > 0 { mapdata[0].len() } else { 0 };
        Self {
            mapdata: mapdata.iter().map(|x| x.chars().collect()).collect(),
            rows,
            cols,
            current_location: Vec::new(),
            current_direction: ' ',
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    // Collect lines and meta data about the map
    let mapdata: Vec<&str> = input.lines().collect();
    let mut map = Map::new(mapdata);

    // Step 1: Find Location and Direction - insert into current_location
    println!("{}", input);
    let (starting_location, starting_direction) = find_starting_spot(&map);
    map.current_location.push(starting_location);
    map.current_direction = starting_direction;
    println!("Starting Point: {:?}", map.current_location);

    // Step 2: Loop through next steps
    loop {
        match next_step(&mut map) {
            Some(move_spot) => {
                map.current_location.push(move_spot.0);
                map.current_direction = move_spot.1;
            }
            None => break,
        }
    }

    // Step 3: Print out current_location
    println!("Final Path: {:?}", map.current_location);
    let mut final_map = map.mapdata.clone();
    for rows in map.current_location.iter() {
        final_map[rows.0][rows.1] = 'X';
    }

    // Convert the final_map into a single String
    let map_as_string: String = final_map
    .iter()
    .map(|row| row.iter().collect::<String>()) // Convert each row (Vec<char>) into a String
    .collect::<Vec<String>>()                 // Collect all rows into a Vec<String>
    .join("\n");

    println!("{}", map_as_string);
    // Step 4: Return current_location.len()
    Some(unique_count(map.current_location) as u64)

}

pub fn part_two(input: &str) -> Option<u64> {
    let _ = input;
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
