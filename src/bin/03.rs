advent_of_code::solution!(3);

pub fn parse(input: &str) -> impl IntoIterator<Item = Vec<u64>> {
    input.trim().lines().map(|line| {
        line.chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect()
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let total_joltage = parse(input)
        .into_iter()
        .map(|battery_bank| calculate_joltage_output(2, &battery_bank))
        .sum();
    Some(total_joltage)
}

pub fn part_two(input: &str) -> Option<u64> {
    let total_joltage = parse(input)
        .into_iter()
        .map(|battery_bank| calculate_joltage_output(12, &battery_bank))
        .sum();
    Some(total_joltage)
}

fn calculate_joltage_output(num_batteries: usize, battery_bank: &[u64]) -> u64 {
    let mut largest_joltage = vec![0_u64; num_batteries];

    for (i, val) in battery_bank.iter().enumerate() {
        for j in 0..largest_joltage.len() {
            let enough_digits_left = i + largest_joltage.len() - j <= battery_bank.len();
            if enough_digits_left && *val > largest_joltage[j] {
                largest_joltage[j] = *val;
                largest_joltage.iter_mut().skip(j + 1).for_each(|x| *x = 0);
                break;
            }
        }
    }

    largest_joltage
        .into_iter()
        .rev()
        .enumerate()
        .map(|(idx, val)| val * 10u64.pow(idx as u32))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
