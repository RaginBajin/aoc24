advent_of_code::solution!(5);
use std::collections::HashMap;


fn build_lookup(record: Vec<String>) -> HashMap<String, usize> {
    let mut row_map = HashMap::new();
    for (col_index, value) in record.iter().enumerate() {
        row_map.insert(value.clone(), col_index);
    }
    row_map
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut rules: HashMap<String, Vec<String>> = HashMap::new();
    let mut data: Vec<Vec<String>> = Vec::new();

    // Step 1: Parse Input File for Rules and Data
    for line in input.lines() {
        if line.contains('|') {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 2 {
                let key = parts[0].trim().to_string();
                let value = parts[1].trim().to_string();

                // Insert into the HashMap, appending to the vector if the key exists
                rules.entry(key).or_insert_with(Vec::new).push(value);
            }
        } else if line.contains(',') {
            let entries: Vec<String> = line
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
            data.push(entries);
        }
    }

    let mut passed_records = Vec::new();

    // Step 2: Process each Data
    for record in &data {
        // Step 2a: Build lookup table
        let lookup_table = build_lookup(record.clone());
        let mut rules_passed = true;
        // Step 3: Each Column in a row, check rules
        for (col_index, value) in record.iter().enumerate() {
            // Fetch rules for the value
            if let Some(check_rules) = rules.get(value) {
                // Step 3b: Check each rule against each value and their location
                for rule_value in check_rules {
                    // Check rule_value in the lookup_table
                    let rule_value_location = match lookup_table.get(rule_value) {
                        Some(value) => value,
                        None => {
                            continue;
                        }
                    };
                    if !col_index.lt(rule_value_location) {
                        rules_passed = false;
                    }
                }
            }
        }
        if rules_passed {
            passed_records.push(record);
        }
    }
    let mut total  = 0;
    for record in passed_records {
        let mid_index = record.len() / 2;
        total += record[mid_index].parse::<u64>().unwrap();
    }

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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
