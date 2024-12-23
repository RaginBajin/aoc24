advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u64> {
    // Step 1: Pull mul(x,y) out of the inputs and put it into a vector
    let re = Regex::new(r"mul\(\d{1,3}\,\d{1,3}\)").unwrap();
    let iter_match: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();
    println!("{:?}", iter_match);
    // Step 2: Parse the numbers out of the strings and multiply them
    let mut total = 0;
    let re = Regex::new(r"\((\d+),(\d+)\)").unwrap();

    for i in iter_match {
        if let Some(caps) = re.captures(i) {
            let x: u64 = caps[1].parse().unwrap();
            let y: u64 = caps[2].parse().unwrap();
            total += x * y;
        }
    }
    println!("Total: {}", total);
    Some(total)
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
