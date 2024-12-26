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

fn check_rules(record: Vec<String>, rules: HashMap<String, Vec<String>>, lookup: HashMap<String, usize>) -> bool {

    let mut rules_passed = true;
    for (col_index, value) in record.iter().enumerate() {
        // Fetch rules for the value
        if let Some(check_rules) = rules.get(value) {
            // Step 3b: Check each rule against each value and their location
            for rule_value in check_rules {
                // Check rule_value in the lookup_table
                let rule_value_location = match lookup.get(rule_value) {
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
    rules_passed
}

fn correctly_sort(input: Vec<String>, rules: HashMap<String, Vec<String>> ) -> Vec<String> {
    let mut sorted = input.clone();
    loop {
        let mut is_sorted = true;
        for i in 0..sorted.len()-1 {
            // Safely get the value at index i+1
            let check_value = match rules.get(sorted.get(i + 1).unwrap()) {
                Some(val) => val,
                None => continue, // Skip if no rule found
            };
            // Safely get the value at index i
            if let Some(current_value) = sorted.get(i) {
                if check_value.contains(current_value) {
                    is_sorted = false;
                    sorted.swap(i, i + 1);
                }
            }
        }
        if is_sorted {
            return sorted;
        }
    }
}


pub fn part_two(input: &str) -> Option<u64> {
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
        // Step 3: Each Column in a row, check rules
        let rules_passed = check_rules(record.clone(), rules.clone(), lookup_table.clone());
        if !rules_passed {
            passed_records.push(record);
        }
    }
    // Step 4: Fix In-ordered Records
    let mut fixed_records = Vec::new();
    for record in passed_records.clone() {
        // Step 3: Each Column in a row, check rules
        let sort_records = correctly_sort(record.clone(), rules.clone());
        fixed_records.push(sort_records);
    }

    // Final Step - Add up Middle passing Records
    let mut total  = 0;
    for record in fixed_records {
        let mid_index = record.len() / 2;
        total += record[mid_index].parse::<u64>().unwrap();
    }

    Some(total)
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
        assert_eq!(result, Some(123));
    }
}
