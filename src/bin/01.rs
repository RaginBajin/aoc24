advent_of_code::solution!(1);
use polars::prelude::*;
use std::io::{BufRead, Cursor};

pub fn part_one(input: &str) -> Option<u64> {
    // 1) Preprocess input: replace " " with "\t"
    let processed_input: String = {
        let mut lines_vec = vec![];
        for line_result in Cursor::new(input).lines() {
            let line = line_result.unwrap();
            // Replace spaces with tabs
            lines_vec.push(line.replace("   ", "\t"));
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
    let df = CsvReadOptions::default()
        .with_has_header(false)
        .with_schema(Some(Arc::new(schema)))
        .map_parse_options(|parse_opts| parse_opts.with_separator(b'\t'))
        .into_reader_with_file_handle(cursor)
        .finish()
        .unwrap();

    println!("Original DataFrame:\n{}", df);
    // Define column names
    let col1_name = "column_0";
    let col2_name = "column_1";

    // Get column names dynamically
    let col_names = df.get_column_names();
    // Collect sorted columns
    let mut sorted_columns = Vec::new();

    for col_name in &col_names {
        if let Ok(column) = df.column(col_name) {
            let sorted_col = column
                .sort(SortOptions {
                    descending: false,
                    ..Default::default()
                })
                .unwrap();
            sorted_columns.push(sorted_col);
        }
    }
    let df = DataFrame::new(sorted_columns).unwrap();

    println!("Sorted DataFrame:\n{}", df);
    // Compute the row-wise absolute differences
    let col1 = df.column(col1_name).unwrap().i64().unwrap();
    let col2 = df.column(col2_name).unwrap().i64().unwrap();
    let difference = (col1 - col2).apply(|v| v.map(|x| x.abs()));

    // Sum up the differences
    let sum_difference: i64 = difference.sum().unwrap_or(0);

    // Print results
    println!("Sorted DataFrame:\n{}", df);
    println!("Sum of differences: {}", sum_difference);

    Some(sum_difference as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    // 1) Preprocess input: replace " " with "\t"
    let processed_input: String = {
        let mut lines_vec = vec![];
        for line_result in Cursor::new(input).lines() {
            let line = line_result.unwrap();
            // Replace spaces with tabs
            lines_vec.push(line.replace("   ", "\t"));
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
    let df = CsvReadOptions::default()
        .with_has_header(false)
        .with_schema(Some(Arc::new(schema)))
        .map_parse_options(|parse_opts| parse_opts.with_separator(b'\t'))
        .into_reader_with_file_handle(cursor)
        .finish()
        .unwrap();

    println!("Original DataFrame:\n{}", df);

    let grouped = df
        .clone()
        .lazy()
        .group_by([col("column_1")])
        .agg([col("column_1").count().cast(DataType::Int64).alias("count")])
        .collect()
        .unwrap();

    let joined = df
        .clone()
        .lazy()
        .join(
            grouped.lazy(),
            [col("column_0")],   // from the left DF
            [col("column_1")], // from the right DF
            JoinType::Left.into(),
        )
        // fill missing counts with 0
        .with_column(col("count").fill_null(lit(0)))
        .collect()
        .unwrap();

    println!("Joined DF (with `count`):\n{joined}");

    // Compute the row-wise absolute differences
    let col1 = joined.column("column_0").unwrap().i64().unwrap();
    let col2 = joined.column("count").unwrap().i64().unwrap();
    let difference = (col1 * col2).apply(|v| v.map(|x| x.abs()));

    // Sum up the differences
    let sum_difference: i64 = difference.sum().unwrap_or(0);

    // Print results
    println!("Sorted DataFrame:\n{}", df);
    println!("Sum of differences: {}", sum_difference);

    Some(sum_difference as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
