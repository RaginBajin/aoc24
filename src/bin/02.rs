advent_of_code::solution!(2);
use std::io::{BufRead, Cursor};
use polars::prelude::*;

pub fn part_one(input: &str) -> Option<u64> {
    // 1) Preprocess input: replace " " with "\t"
    let processed_input: String = {
        let mut lines_vec = vec![];
        for line_result in Cursor::new(input).lines() {
            let line = line_result.unwrap();
            // Replace spaces with tabs
            lines_vec.push(line.replace(' ', "\t"));
        }
        lines_vec.join("\n")
    };

    // 2) Determine how many columns we have at maximum
    let max_cols = processed_input
        .lines()
        .map(|line| line.split('\t').count())
        .max()
        .unwrap_or(0);

    // 3) Build a Polars schema from these columns
    let mut schema = Schema::default();
    for i in 0..max_cols {
        schema.with_column(format!("column_{}", i).into(), DataType::Int64);
    }

    // 4) Build a new cursor from processed_input for Polars to read
    let cursor = Cursor::new(processed_input);

    // 5) Create the CSV reader with dynamic schema
    let mut df = CsvReadOptions::default()
        .with_has_header(false)
        .with_schema(Some(Arc::new(schema)))
        .map_parse_options(|parse_opts| parse_opts.with_separator(b'\t'))
        .into_reader_with_file_handle(cursor)
        .finish()
        .unwrap();

    println!("Original DataFrame:\n{}", df);

    // We'll accumulate our new flags
    let col_names = df.get_column_names();
    let n = df.height();

    let mut inc_flags = Vec::with_capacity(n);
    let mut dec_flags = Vec::with_capacity(n);
    let mut val_flags = Vec::with_capacity(n);
    let mut meets_all_flags = Vec::with_capacity(n);

    for row_idx in 0..n {
        // Collect the row as Option<i64> (some cells might be None if shorter)
        let row_vals: Vec<Option<i64>> = col_names
            .iter()
            .map(|c| {
                df.column(c)
                    .unwrap()
                    .i64()
                    .unwrap()
                    .get(row_idx) // Option<i64>
            })
            .collect();

        let (is_increasing, is_decreasing, value_check) = check_sequence_no_skip(&row_vals);
        let meets_all = (is_increasing || is_decreasing) && value_check;

        inc_flags.push(is_increasing);
        dec_flags.push(is_decreasing);
        val_flags.push(value_check);
        meets_all_flags.push(meets_all);
    }
       // Create the new Series
       let inc_series = Series::new("is_increasing".into(), inc_flags);
       let dec_series = Series::new("is_decreasing".into(), dec_flags);
       let check_series = Series::new("valid_check".into(), val_flags);
       let meets_all_series = Series::new("meets_all_conditions".into(), meets_all_flags);

       // Add columns to the DataFrame
       let df = df
           .with_column(inc_series).unwrap()
           .with_column(dec_series).unwrap()
           .with_column(check_series).unwrap()
           .with_column(meets_all_series).unwrap();

       println!("\nFinal DataFrame:\n{}", df);

       // Count how many rows had meets_all = true
       let value: u64 = df
           .column("meets_all_conditions")
           .unwrap()
           .bool()
           .unwrap()
           .sum() // sums over bool, treating true=1, false=0
           .unwrap_or(0)
           .into();

       println!("Number of rows that meet all conditions = {value}");
       Some(value)

}

/// A helper that checks a row's sequence with *no* skips.
/// We do a single pass from left to right and return (inc, dec, val).
/// - `inc`: strictly increasing did not fail
/// - `dec`: strictly decreasing did not fail
/// - `val`: all differences were in [1..=3]
fn check_sequence_no_skip(row: &[Option<i64>]) -> (bool, bool, bool) {
    let mut is_increasing = true;
    let mut is_decreasing = true;
    let mut value_check = true;

    // We'll check adjacent pairs
    for w in row.windows(2) {
        match (w[0], w[1]) {
            (Some(a), Some(b)) => {
                // difference in [1..=3]?
                let diff = (b - a).abs();
                if !(1..=3).contains(&diff) {
                    value_check = false;
                    break;
                }
                // strictly increasing / decreasing
                if a >= b {
                    is_increasing = false;
                }
                if a <= b {
                    is_decreasing = false;
                }

                // If everything is false, we can stop
                if !is_increasing && !is_decreasing && !value_check {
                    break;
                }
            }
            // If either side is None, we treat that as end of valid data
            // so we just stop checking further
            _ => {
                break;
            }
        }
    }

    (is_increasing, is_decreasing, value_check)
}

/// Demonstration function that shows how to implement:
///  1) validate the row as-is
///  2) if it fails, remove columns left-to-right until we find a pass or run out of columns
///
/// We consider "valid" if (is_increasing || is_decreasing) && value_check.
pub fn part_two(input: &str) -> Option<u64> {
    // 1) Preprocess input: replace " " with "\t"
    let processed_input: String = {
        let mut lines_vec = vec![];
        for line_result in Cursor::new(input).lines() {
            let line = line_result.unwrap();
            // Replace spaces with tabs
            lines_vec.push(line.replace(' ', "\t"));
        }
        lines_vec.join("\n")
    };

    // 2) Determine how many columns we have at maximum
    let max_cols = processed_input
        .lines()
        .map(|line| line.split('\t').count())
        .max()
        .unwrap_or(0);

    // 3) Build a Polars schema from these columns
    let mut schema = Schema::default();
    for i in 0..max_cols {
        schema.with_column(format!("column_{}", i).into(), DataType::Int64);
    }

    // 4) Build a new cursor from processed_input for Polars to read
    let cursor = Cursor::new(processed_input);

    // 5) Create the CSV reader with dynamic schema
    let mut df = CsvReadOptions::default()
        .with_has_header(false)
        .with_schema(Some(Arc::new(schema)))
        .map_parse_options(|parse_opts| parse_opts.with_separator(b'\t'))
        .into_reader_with_file_handle(cursor)
        .finish()
        .unwrap();

    println!("Original DataFrame:\n{}", df);

    // We'll accumulate our new flags
    let col_names = df.get_column_names();
    let n = df.height();

    let mut inc_flags = Vec::with_capacity(n);
    let mut dec_flags = Vec::with_capacity(n);
    let mut val_flags = Vec::with_capacity(n);
    let mut meets_all_flags = Vec::with_capacity(n);

    for row_idx in 0..n {
        // Collect the row as Option<i64> (some cells might be None if shorter)
        let row_vals: Vec<Option<i64>> = col_names
            .iter()
            .map(|c| {
                df.column(c)
                    .unwrap()
                    .i64()
                    .unwrap()
                    .get(row_idx) // Option<i64>
            })
            .collect();

        // 1) Check the row "as is"
        let (mut is_increasing, mut is_decreasing, mut value_check) = check_sequence_no_skip(&row_vals);
        let mut meets_all = (is_increasing || is_decreasing) && value_check;

        // 2) If not meets_all, try removing each column in turn
        if !meets_all {
            let mut success = false;
            // Try removing columns left-to-right
            for col_to_remove in 0..row_vals.len() {
                let mut temp_row = row_vals.clone();
                temp_row.remove(col_to_remove); // remove the nth column

                let (i2, d2, v2) = check_sequence_no_skip(&temp_row);
                if (i2 || d2) && v2 {
                    // we found a pass
                    is_increasing = i2;
                    is_decreasing = d2;
                    value_check = v2;
                    meets_all = true;
                    success = true;
                    break;
                }
            }
            // if success remains false, we never found a valid removal
            if !success {
                // We'll keep meets_all = false
                // inc/dec/val_check are whatever the last pass was
            }
        }

        inc_flags.push(is_increasing);
        dec_flags.push(is_decreasing);
        val_flags.push(value_check);
        meets_all_flags.push(meets_all);
    }

    // Create the new Series
    let inc_series = Series::new("is_increasing".into(), inc_flags);
    let dec_series = Series::new("is_decreasing".into(), dec_flags);
    let check_series = Series::new("valid_check".into(), val_flags);
    let meets_all_series = Series::new("meets_all_conditions".into(), meets_all_flags);

    // Add columns to the DataFrame
    let df = df
        .with_column(inc_series).unwrap()
        .with_column(dec_series).unwrap()
        .with_column(check_series).unwrap()
        .with_column(meets_all_series).unwrap();

    println!("\nFinal DataFrame:\n{}", df);

    // Count how many rows had meets_all = true
    let value: u64 = df
        .column("meets_all_conditions")
        .unwrap()
        .bool()
        .unwrap()
        .sum() // sums over bool, treating true=1, false=0
        .unwrap_or(0)
        .into();

    println!("Number of rows that meet all conditions = {value}");
    Some(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
