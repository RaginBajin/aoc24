advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u64> {
    // Step 1: Pull mul(x,y) out of the inputs and put it into a vector
    let re = Regex::new(r"mul\(\d{1,3}\,\d{1,3}\)").unwrap();
    let iter_match: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();
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

fn collect_do_dont(input: &str) -> Vec<String> {
    // We start collecting from the very beginning
    let mut collecting = true;
    let mut chunks = Vec::new();
    let mut chunk_start = 0;
    let mut i = 0;

    while i < input.len() {
        // Look for "do()" or "don't()" in the remainder of the string
        let sub = &input[i..];
        let do_pos = sub.find("do()");
        let dont_pos = sub.find("don't()");

        // If neither token is found, we're done scanning
        if do_pos.is_none() && dont_pos.is_none() {
            break;
        }

        // Figure out which token occurs earliest
        let (token, token_pos) = match (do_pos, dont_pos) {
            (Some(dp), Some(dp_dont)) => {
                if dp < dp_dont {
                    ("do()", dp)
                } else {
                    ("don't()", dp_dont)
                }
            }
            (Some(dp), None) => ("do()", dp),
            (None, Some(dp_dont)) => ("don't()", dp_dont),
            _ => unreachable!(),
        };

        // Convert position-in-substring to absolute position
        let abs_pos = i + token_pos;

        match token {
            "do()" if !collecting => {
                // Turn collecting ON, starting right after "do()"
                collecting = true;
                chunk_start = abs_pos + token.len();
            }
            "don't()" if collecting => {
                // We were collecting; finalize this chunk
                chunks.push(input[chunk_start..abs_pos].to_string());
                // Turn collecting OFF
                collecting = false;
            }
            // If we see "do()" while already collecting,
            // or "don't()" while not collecting, we just skip over it.
            _ => {}
        }

        // Move `i` to the end of the token we found
        i = abs_pos + token.len();
    }

    // If we're still collecting at the very end, capture the leftover text
    if collecting && chunk_start < input.len() {
        chunks.push(input[chunk_start..].to_string());
    }

    chunks
}

pub fn part_two(input: &str) -> Option<u64> {
    let chunks = collect_do_dont(input);
    let mut total = 0;
    for (_i, chunk) in chunks.iter().enumerate() {
        total += part_one(chunk).unwrap();
    }
    Some(total)
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
        assert_eq!(result, Some(48));
    }
}
