advent_of_code::solution!(4);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GridItem {
    Empty,
    RollOfPaper,
}
pub struct GridPos(usize);
pub struct GridLine(Vec<GridItem>);
pub struct Grid {
    lines: GridLine,
    width: usize,
    height: usize,
}

impl GridLine {
    fn find(&self, filter: GridItem) -> impl Iterator<Item = GridPos> {
        self.0
            .iter()
            .enumerate()
            .filter(move |(_, item)| **item == filter)
            .map(|(idx, _)| GridPos(idx))
    }
}

impl Grid {
    fn find(&self, filter: GridItem) -> impl Iterator<Item = GridPos> {
        self.lines.find(filter)
    }
    fn ctx(&self, pos: &GridPos, size: usize) -> GridLine {
        let mut line = vec![];
        let pos_x = pos.0 % self.width;
        let pos_y = pos.0 / self.width;
        for y in pos_y.saturating_sub(size)..=pos_y.strict_add(size) {
            if y >= self.height {
                break;
            }
            for x in pos_x.saturating_sub(size)..=pos_x.strict_add(size) {
                if x >= self.width {
                    break;
                }
                let item = self.lines.0[y * self.width + x];
                line.push(item);
            }
        }
        GridLine(line)
    }
}

pub fn parse_input(input: &str) -> Grid {
    let width = input.lines().next().unwrap().len();
    let lines = input
        .trim()
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '@' => GridItem::RollOfPaper,
                _ => GridItem::Empty,
            })
        })
        .collect();
    let lines = GridLine(lines);
    let height = lines.0.len() / width;
    Grid {
        lines,
        width,
        height,
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse_input(input);
    let accessible_rolls_of_paper = grid
        .find(GridItem::RollOfPaper)
        .filter(|pos| grid.ctx(pos, 1).find(GridItem::RollOfPaper).count() < 5)
        .count();
    Some(accessible_rolls_of_paper)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = parse_input(input);
    let mut accessible_rolls_of_paper = 0;
    let mut removed_rolls_of_paper = vec![];
    loop {
        removed_rolls_of_paper.extend(
            grid.find(GridItem::RollOfPaper)
                .filter(|pos| grid.ctx(pos, 1).find(GridItem::RollOfPaper).count() < 5),
        );
        if removed_rolls_of_paper.is_empty() {
            break;
        }
        accessible_rolls_of_paper += removed_rolls_of_paper.len();
        for pos in removed_rolls_of_paper.drain(..) {
            grid.lines.0[pos.0] = GridItem::Empty;
        }
    }
    Some(accessible_rolls_of_paper)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
