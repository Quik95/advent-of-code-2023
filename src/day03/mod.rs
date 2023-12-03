use std::str::FromStr;

use itertools::Itertools;

use crate::AoCProblem;

#[derive(Default)]
pub struct Day03 {
    data: EngineSchematic,
}

impl AoCProblem for Day03 {
    fn parse_input(&mut self, input: &str) {
        self.data = input.parse().unwrap();
    }

    fn parse_input_default(&mut self) {
        self.parse_input(include_str!("../../inputs/day03.txt"));
    }

    fn part_1(&self) -> Option<String> {
        let symbols = self.data.get_symbols();
        let sum = symbols
            .iter()
            .map(|(x, y)| {
                [
                    (0, 1),
                    (1, 0),
                    (0, -1),
                    (-1, 0),
                    (1, 1),
                    (-1, -1),
                    (-1, 1),
                    (1, -1),
                ]
                .iter()
                .filter(|(dx, dy)| self.data.get_or_default(x + dx, y + dy).is_ascii_digit())
                .map(|(dx, dy)| self.data.find_number_bounds(x + dx, y + dy).unwrap())
                .unique()
                .map(|(st, en)| char_slice_to_int(&self.data.map[st..=en]))
                .sum::<i32>()
            })
            .sum::<i32>();

        Some(sum.to_string())
    }

    fn part_2(&self) -> Option<String> {
        let symbols = self.data.get_gear_symbols();
        let sum = symbols
            .iter()
            .map(|(x, y)| {
                [
                    (0, 1),
                    (1, 0),
                    (0, -1),
                    (-1, 0),
                    (1, 1),
                    (-1, -1),
                    (-1, 1),
                    (1, -1),
                ]
                .iter()
                .filter(|(dx, dy)| self.data.get_or_default(x + dx, y + dy).is_ascii_digit())
                .map(|(dx, dy)| self.data.find_number_bounds(x + dx, y + dy).unwrap())
                .unique()
                .collect_vec()
            })
            .filter(|unique| unique.len() == 2)
            .map(|unique| {
                unique
                    .iter()
                    .map(|(start, end)| char_slice_to_int(&self.data.map[*start..=*end]))
                    .product::<i32>()
            })
            .sum::<i32>();

        Some(sum.to_string())
    }

    fn get_day_name(&self) -> String {
        "Day 03: Gear Ratios".into()
    }
}

fn char_slice_to_int(slice: &[char]) -> i32 {
    slice.iter().rev().enumerate().fold(0, |acc, (i, c)| {
        acc + (*c as i32 - '0' as i32) * 10_i32.pow(i as u32)
    })
}

#[derive(Default)]
struct EngineSchematic {
    map: Vec<char>,
    width: usize,
    height: usize,
}

impl EngineSchematic {
    fn get(&self, x: i32, y: i32) -> Option<char> {
        if x >= self.width as i32 || y >= self.height as i32 || x < 0 || y < 0 {
            None
        } else {
            Some(self.map[(y * self.width as i32 + x) as usize])
        }
    }

    fn get_or_default(&self, x: i32, y: i32) -> char {
        self.get(x, y).unwrap_or('.')
    }

    const fn index(&self, x: i32, y: i32) -> Option<usize> {
        if x >= self.width as i32 || y >= self.height as i32 || x < 0 || y < 0 {
            None
        } else {
            Some((y * self.width as i32 + x) as usize)
        }
    }

    fn get_symbols(&self) -> Vec<(i32, i32)> {
        let mut symbols = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.map[y * self.width + x] != '.'
                    && !self.map[y * self.width + x].is_ascii_digit()
                {
                    symbols.push((x as i32, y as i32));
                }
            }
        }
        symbols
    }

    fn get_gear_symbols(&self) -> Vec<(i32, i32)> {
        self.map
            .iter()
            .enumerate()
            .filter_map(|(i, c)| {
                if *c == '*' {
                    Some(((i % self.width) as i32, (i / self.width) as i32))
                } else {
                    None
                }
            })
            .collect_vec()
    }

    fn find_number_bounds(&self, x: i32, y: i32) -> Option<(usize, usize)> {
        let mut index_start = self.index(x, y)?;
        let mut index_end = self.index(x, y)?;
        if !self.map[index_start].is_ascii_digit() {
            return None;
        }

        while index_start > 0
            && index_start % self.width != 0
            && self.map[index_start - 1].is_ascii_digit()
        {
            index_start -= 1;
        }

        while index_end < self.map.len() - 1
            && index_end % self.width != self.width - 1
            && self.map[index_end + 1].is_ascii_digit()
        {
            index_end += 1;
        }

        Some((index_start, index_end))
    }
}

impl FromStr for EngineSchematic {
    type Err = !;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let lines = value
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect_vec();
        let height = lines.len();
        let width = lines[0].len();
        let mut map = Vec::with_capacity(width * height);
        for line in lines {
            map.extend(line.trim().chars());
        }

        Ok(Self { map, width, height })
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::AoCProblem;

    #[test]
    pub fn part1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
.........1
10+10.....
";

        let mut problem = super::Day03::default();
        problem.parse_input(input);
        assert_eq!(problem.part_1(), Some("4381".into()));
    }

    #[test]
    pub fn part2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
.........1
10+10.....
";

        let mut problem = super::Day03::default();
        problem.parse_input(input);
        assert_eq!(problem.part_2(), Some("467835".into()));
    }
}
