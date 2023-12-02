use std::str::FromStr;

use itertools::Itertools;

use crate::AoCProblem;

#[derive(Default)]
pub struct Day02 {
    data: Vec<Vec<CubeSet>>,
}

static INPUT: &str = include_str!("../../inputs/day02.txt");

impl AoCProblem for Day02 {
    fn parse_input(&mut self, input: &str) {
        self.data =
            input
                .lines()
                .map(|l| l[l.find(": ").unwrap() + 2..].split(';').map(|b| b.parse().unwrap()).collect_vec())
                .collect_vec()
    }

    fn parse_input_default(&mut self) {
        self.parse_input(INPUT)
    }

    fn part_1(&self) -> Option<String> {
        Some(
            self.data
                .iter()
                .enumerate()
                .filter(|(_, v)| v.iter().all(|c| c.is_valid()))
                .map(|(i, _)| i + 1)
                .sum::<usize>()
                .to_string()
        )
    }

    fn part_2(&self) -> Option<String> {
        Some(
            self.data
                .iter()
                .map(|d| CubeSet::power(d))
                .sum::<u32>()
                .to_string()
        )
    }

    fn get_day_name(&self) -> String {
        "Day 2: Cube Conundrum".into()
    }
}

struct CubeSet {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl CubeSet {
    pub const fn is_valid(&self) -> bool {
        self.red <= 12
            && self.green <= 13
            && self.blue <= 14
    }

    pub fn power(days: &[Self]) -> u32 {
        let mut red_max = 0;
        let mut green_max = 0;
        let mut blue_max = 0;

        for d in days {
            red_max = red_max.max(d.red);
            green_max = green_max.max(d.green);
            blue_max = blue_max.max(d.blue);
        }

        red_max * green_max * blue_max
    }
}

impl FromStr for CubeSet {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(", ");

        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;

        for s in split {
            let s = s.trim();
            let (value, key) = (&s[..s.find(' ').unwrap()], &s[(s.find(' ').unwrap() + 1)..]);
            match key {
                "red" => red += value.parse::<u32>().unwrap(),
                "green" => green += value.parse::<u32>().unwrap(),
                "blue" => blue += value.parse::<u32>().unwrap(),
                _ => unreachable!()
            }
        }

        Ok(Self {
            red,
            green,
            blue,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::AoCProblem;

    #[test]
    fn part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let mut day = super::Day02::default();
        day.parse_input(input);

        assert_eq!(day.part_1(), Some("8".into()));
    }

    #[test]
    fn part2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let mut day = super::Day02::default();
        day.parse_input(input);

        assert_eq!(day.part_2(), Some("2286".into()));
    }
}