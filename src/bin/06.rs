advent_of_code::solution!(6);

enum Operator {
    Add,
    Mul,
}

impl Operator {
    fn parse(op: &str) -> Self {
        match op {
            "+" => Self::Add,
            "*" => Self::Mul,
            _ => panic!("unknown operator: {op}"),
        }
    }
}

fn parse_input(input: &str) -> (Vec<Operator>, Vec<&str>) {
    let mut lines = input.lines().collect::<Vec<_>>();
    let ops = lines
        .pop()
        .unwrap()
        .split_whitespace()
        .map(Operator::parse)
        .collect();
    (ops, lines)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ops, lines) = parse_input(input);
    let grand_total = lines
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect::<Vec<u64>>()
        })
        .reduce(|mut reg, val| {
            for (i, (op, v)) in ops.iter().zip(val).enumerate() {
                match op {
                    Operator::Add => reg[i] += v,
                    Operator::Mul => reg[i] *= v,
                }
            }
            reg
        })
        .unwrap()
        .into_iter()
        .sum();
    Some(grand_total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ops, lines) = parse_input(input);

    let lines = {
        let mut transposed_lines = vec![String::new(); lines[0].len()];
        for line in lines.iter() {
            for (i, c) in line.chars().enumerate() {
                if !c.is_whitespace() {
                    transposed_lines[i].push(c);
                }
            }
        }
        transposed_lines
    };

    let mut grand_total = 0;
    let mut line_cursor = 0;
    for op in ops {
        let mut reg = lines[line_cursor].parse::<u64>().unwrap();
        line_cursor += 1;
        while let Some(line) = lines.get(line_cursor)
            && !line.is_empty()
        {
            let val = line.parse::<u64>().unwrap();
            match op {
                Operator::Add => {
                    reg += val;
                }
                Operator::Mul => {
                    reg *= val;
                }
            }
            line_cursor += 1;
        }
        grand_total += reg;
        line_cursor += 1;
    }

    Some(grand_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
