advent_of_code::solution!(6);
use std::{collections::HashSet, ops::Add};


fn find_starting_spot(map: &Map) -> Option<((usize, usize), char)> {
    let directions = ['^', '>', 'v', '<'];
    map.mapdata.iter().enumerate().find_map(|(row_index, row)| {
        row.iter().enumerate().find_map(|(col_index, &value)| {
            if directions.contains(&value) {
                Some(((row_index, col_index), value))
            } else {
                None
            }
        })
    })
}

fn look_right(map: &Map, current_location: (usize, usize), direction: char) -> Option<((usize, usize), char)> {
    let (row_offset, col_offset, new_dir) = match direction {
        '^' => (0, 1, '>'),
        '>' => (1, 0, 'v'),
        'v' => (0, -1, '<'),
        '<' => (-1, 0, '^'),
        _ => return None, // Invalid direction
    };

    let new_location = (
        current_location.0 as isize + row_offset,
        current_location.1 as isize + col_offset,
    );

    if new_location.0 < 0 || new_location.1 < 0 {
        return None; // Out of bounds
    }

    let new_location = (new_location.0 as usize, new_location.1 as usize);

    if !is_within_bounds(map, new_location) || map.mapdata[new_location.0][new_location.1] == '#' || map.mapdata[new_location.0][new_location.1] == '0' {
        None
    } else {
        Some((new_location, new_dir))
    }
}

fn is_within_bounds(map: &Map, location: (usize, usize)) -> bool {
    location.0 < map.mapdata.len() && location.1 < map.mapdata[0].len()
}

fn next_step(map: &mut Map) -> Option<((usize, usize), char)> {
    // Retrieve the current location and direction safely
    let current_location = map.current_location.last()?;
    let direction = map.current_direction;

    // Determine the movement offset
    let move_where  = match direction {
        '^' => (-1, 0),
        '>' => (0, 1),
        'v' => (1, 0),
        '<' => (0, -1),
        _ => return None,
    };

    // Calculate the next step
    let next_step = (
        current_location.0 as isize + move_where.0,
        current_location.1 as isize + move_where.1,
    );

    // Check bounds
    if next_step.0 < 0 || next_step.1 < 0 ||
       next_step.0 as usize >= map.rows || next_step.1 as usize >= map.cols {
        return None;
    }

    let next_step = (next_step.0 as usize, next_step.1 as usize);

    // Check for walls
    if map.mapdata[next_step.0][next_step.1] == '#' || map.mapdata[next_step.0][next_step.1] == '0' {
        // Attempt a right turn
        if let Some((new_location, new_direction)) = look_right(map, *current_location, direction) {
            return Some((new_location, new_direction));
        } else {
            return None; // No valid move available
        }
    }

    // Update the map with the next step
    Some((next_step, direction))
}

fn setup_map(input: &str) -> Option<Map> {
    let mapdata: Vec<&str> = input.lines().collect();
    let mut map = Map::new(mapdata);

    if let Some((starting_location, starting_direction)) = find_starting_spot(&map) {
        map.current_location.push(starting_location);
        map.current_direction = starting_direction;
        Some(map)
    } else {
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Map {
    mapdata: Vec<Vec<char>>, // Updated type
    rows: usize,
    cols: usize,
    current_location: Vec<(usize, usize)>,
    current_direction: char,
    guard: Guard,
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
            guard: Guard {
                pos: Pos(0, 0),
                dir: Dir::Up,
            },
        }
    }
    
    fn set_obstactle(&mut self, Pos(x,y): Pos, value: char) {

        if let Some(cell) = self
        .mapdata
        .get_mut(x as usize)
        .and_then(|row| row.get_mut(y as usize))
        {
            *cell = value;
        }
    }

    fn looping(&mut self, origin: Guard, obstacle: Pos) -> bool {
        let mut visited  = HashSet::new();

        self.guard = origin;
        self.set_obstactle(obstacle, '0');

        let looping = loop {
            if !visited.insert((self.guard.pos, self.guard.dir)) {
                break true;
            }
            if self.next().is_none() {
                break false;
            }
        };
        self.set_obstactle(obstacle, '.');
        looping

    }

    fn get(&self, Pos(x, y): Pos) -> Option<char> {
        self.mapdata.get(x as usize)?.get(y as usize).copied()
    }

    fn next(&mut self) -> Option<()> {
        let next = self.guard.pos + self.guard.dir.offset();
        match self.get(next) {
            Some('#' | '0') => {
                self.guard.dir = self.guard.dir.turn();
                Some(())
            }
            Some(_) => {
                self.guard.pos = next;
                Some(())
            }
            None => None,
        }
    }

    fn walk(&mut self) -> HashSet<Pos> {
        let mut visited = HashSet::new();

        loop {
            visited.insert(self.guard.pos);

            if self.next().is_none() {
                break;
            }
        }
        visited
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Guard {
    pos: Pos,
    dir: Dir,
}

struct Off(i32, i32);

impl Add<Off> for Pos {
    type Output = Self;

    fn add(self, Off(dx, dy): Off) -> Self::Output {
        Pos(self.0 + dx, self.1 + dy)
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}

impl Dir {
    fn offset(self) -> Off {
        match self {
            Dir::Up => Off(-1, 0),
            Dir::Down => Off(1, 0),
            Dir::Right => Off(0, 1),
            Dir::Left => Off(0, -1),
        }
    }

    fn turn(self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}


#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, Debug)]
struct Pos(i32, i32);

pub fn part_one(input: &str) -> Option<u64> {
    let mut map = setup_map(input)?;

    loop {
        match next_step(&mut map) {
            Some(move_spot) => {
                map.current_location.push(move_spot.0);
                map.current_direction = move_spot.1;
            }
            None => break,
        }
    }
    // Use HashSet to count unique locations
    let unique_locations: HashSet<_> = map.current_location.into_iter().collect();
    Some(unique_locations.len() as u64)

}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = setup_map(input)?;
    map.guard = match find_starting_spot(&map) {
        Some((pos, dir)) => Guard {
            pos: Pos(pos.0 as i32, pos.1 as i32),
            dir: match dir {
                '^' => Dir::Up,
                '>' => Dir::Right,
                'v' => Dir::Down,
                '<' => Dir::Left,
                _ => unreachable!(),
            },
        },
        None => return None,
    };

    let origin = map.guard;
    Some(map.walk().iter().filter(|&&obstacle| map.looping(origin, obstacle)).count() as u64)
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
        assert_eq!(result, Some(6));
    }
}
