advent_of_code::solution!(4);
use std::error::Error;
use regex::Regex;

fn extract_diagonals(input: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines[0].len();

    if rows < 4 || cols < 4 {
        return Err("Input must have at least 4 rows and 4 columns.".into());
    }

    let mut diagonals = Vec::new();

    // Top-left to bottom-right diagonals
    for r in 0..=(rows - 4) {
      //  println!("TL Data: {}", lines[r] );
        for c in 0..=(cols - 4) {
            let diagonal: String = (0..4)
                .map(|i| {
                    let pair = (r + i, c + i); // Current (row, column) pair
             //       println!("TL pair: {:?}", pair);
                    lines[pair.0].chars().nth(pair.1).unwrap()
                })
                .collect();
         //   println!("TL Diagonal: {}", diagonal);
            diagonals.push(diagonal);
        }
    }

    // Top-right to bottom-left diagonals
    for r in 0..=(rows - 4) {
      //  println!("TR Data: {}", lines[r] );
        for c in 3..cols {
            let diagonal: String = (0..4)
                .map(|i| {
                    let pair = (r + i, c - i); // Current (row, column) pair
                 //   println!("TL pair: {:?}", pair);
                    lines[pair.0].chars().nth(pair.1).unwrap()
                })
                .collect();
          //  println!("TR Diagonal: {}", diagonal);
            diagonals.push(diagonal);
            }
    }

    // Verticals
    for r in 0..=(rows - 4) {
     //   println!("V Data: {}", lines[r] );
        for c in 0..cols {
          //  println!("V-V c: {}", c);
            let diagonal: String = (0..4)
                .map(|i| {
                    let pair = (r + i, c); // Current (row, column) pair
                   // println!("TL pair: {:?}", pair);
                    lines[pair.0].chars().nth(pair.1).unwrap()
                })
                .collect();
           // println!("V Diagonal: {}", diagonal);
            diagonals.push(diagonal);
            }
    }


    Ok(diagonals)
}


pub fn part_one(input: &str) -> Option<u64> {
    let word = "XMAS";

    // Reverse the word for the backward regex
    let reverse_word = word.chars().rev().collect::<String>();
    let foward_re = Regex::new(word).unwrap();
    let backward_re = Regex::new(&reverse_word).unwrap();

    // Collect lines and diagonals into a unified data vector
    let mut data: Vec<&str> = input.lines().collect();
    let diagonals = extract_diagonals(input).unwrap();
    let mut diagonals_refs: Vec<&str> = diagonals.iter().map(|s| s.as_str()).collect();
    data.append(&mut diagonals_refs);

   // println!("Data: {:?}", data);
    let mut total = 0;

    // Iterate over each line to find all matches
    for line in data.iter() {
        // Count all forward matches in the line
        total += foward_re.find_iter(line).count() as u64;

        // Count all backward matches in the line
        total += backward_re.find_iter(line).count() as u64;
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
