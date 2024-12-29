advent_of_code::solution!(7);

fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    input.lines()
    .filter_map(|line| {
        let (answer, nums) = line.split_once(':')?;
        let answer = answer.parse().ok()?;
        let nums = nums.split_whitespace().filter_map(|num| num.parse().ok()).collect();
        Some((answer, nums))
    })
    .collect()
}

fn solve(test: u64, nums: &[u64]) -> bool {

    nums
        .iter()
        .skip(1)
        .fold(vec![nums[0]], |results, &num| {
            results
                .iter()
                .flat_map(|&res| vec![res+num, res*num])
                .collect()
        })
        .contains(&test)
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse(input)
            .iter()
            .filter(|(answer, nums)| solve(*answer, nums) )
            .map(|(answer, _)| answer)
            .sum(),
    )
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
