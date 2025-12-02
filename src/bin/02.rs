advent_of_code::solution!(2);

fn parse(input: &str) -> impl IntoIterator<Item = (u64, u64)> {
    input
        .trim()
        .split(',')
        .map(|range| range.split_once('-').unwrap())
        .map(|(first, last)| (first.parse().unwrap(), last.parse().unwrap()))
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;
    for (first, last) in parse(input) {
        for id in first..=last {
            let id_str = id.to_string();
            if id_str.len() % 2 == 0 {
                let mid = id_str.len() / 2;
                if id_str[..mid] == id_str[mid..] {
                    sum += id;
                }
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;
    for (first, last) in parse(input) {
        for id in first..=last {
            let id_str = id.to_string();
            'next_pattern: for div_by in 2..=id_str.len() {
                if id_str.len() % div_by == 0 {
                    let pat_len = id_str.len() / div_by;
                    let pattern = &id_str[..pat_len];
                    for i in 1..div_by {
                        let start = i * pat_len;
                        let end = start + pat_len;
                        if &id_str[start..end] != pattern {
                            continue 'next_pattern;
                        }
                    }
                    sum += id;
                    break;
                }
            }
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
