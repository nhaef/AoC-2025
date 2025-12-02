advent_of_code::solution!(1);

enum Dir {
    Left,
    Right,
}

impl std::str::FromStr for Dir {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Dir::Left),
            "R" => Ok(Dir::Right),
            _ => Err(()),
        }
    }
}

fn parse(input: &str) -> impl IntoIterator<Item = (Dir, u64)> {
    input
        .trim()
        .lines()
        .map(|l| (l[..1].parse().unwrap(), l[1..].parse().unwrap()))
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial = 50_u64;
    let mut dial_hit_zero = 0_u64;
    for (dir, mut val) in parse(input) {
        val %= 100;
        dial = match dir {
            Dir::Left => match dial.checked_sub(val) {
                Some(dial) => dial,
                None => 100 - (val - dial),
            },
            Dir::Right => (dial + val) % 100,
        };
        if dial == 0 {
            dial_hit_zero += 1;
        }
    }
    Some(dial_hit_zero)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dial = 50_u64;
    let mut dial_hit_zero = 0_u64;
    for (dir, mut val) in parse(input) {
        dial = match dir {
            Dir::Left => match dial.checked_sub(val) {
                Some(dial) => dial,
                None => {
                    if dial != 0 {
                        val -= dial;
                        dial_hit_zero += 1;
                    }
                    dial_hit_zero += (val - 1) / 100;
                    (100 - (val % 100)) % 100
                }
            },
            Dir::Right => {
                dial += val;
                dial_hit_zero += (dial - 1) / 100;
                dial % 100
            }
        };
        if dial == 0 {
            dial_hit_zero += 1;
        }
    }
    Some(dial_hit_zero)
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
        assert_eq!(result, Some(6));
    }
}
