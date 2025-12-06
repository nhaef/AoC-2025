use std::{cmp::Ordering, ops::RangeInclusive};

advent_of_code::solution!(5);

type IngredientId = u64;
type IngredientRange = RangeInclusive<IngredientId>;

fn parse_input(input: &str) -> (Vec<IngredientRange>, Vec<IngredientId>) {
    let lines = input.trim().lines();

    let fresh_ingredient_ranges = lines
        .clone()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .collect();

    let available_ingredients = lines
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| line.parse().unwrap())
        .collect();

    (fresh_ingredient_ranges, available_ingredients)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (fresh_ingredient_ranges, available_ingredients) = parse_input(input);

    let merged_ranges = fresh_ingredient_ranges
        .into_iter()
        .fold(vec![], merge_range);

    let count = available_ingredients
        .iter()
        .filter(|id| merged_ranges.iter().any(|range| range.contains(id)))
        .count();

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (fresh_ingredient_ranges, _) = parse_input(input);

    let merged_ranges = fresh_ingredient_ranges
        .into_iter()
        .fold(vec![], merge_range);

    let count = merged_ranges
        .into_iter()
        .map(|range| 1 + range.end() - range.start())
        .sum();

    Some(count)
}

fn merge_range(mut acc: Vec<IngredientRange>, range: IngredientRange) -> Vec<IngredientRange> {
    for i in 0..acc.len() {
        match range.end().cmp(acc[i].start()) {
            Ordering::Less => {
                acc.insert(i, range);
                return acc;
            }
            Ordering::Equal => {
                acc[i] = *range.start()..=*acc[i].end();
                return acc;
            }
            Ordering::Greater => {
                if *range.start() > acc[i].end() + 1 {
                    continue;
                }
                let start = *acc[i].start().min(range.start());
                let mut end = *acc[i].end().max(range.end());
                for j in (i + 1)..acc.len() {
                    if *acc[j].start() > end + 1 {
                        break;
                    }
                    let included_range = acc.remove(j);
                    end = end.max(*included_range.end());
                }
                acc[i] = start..=end;
                return acc;
            }
        }
    }
    acc.push(range);
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
