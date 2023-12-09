use std::str::FromStr;

use itertools::Itertools;

use crate::AoCProblem;

#[derive(Default)]
pub struct Day09 {
    data: Vec<NumberSequence>,
}

impl AoCProblem for Day09 {
    fn parse_input(&mut self, input: &str) {
        self.data = input.lines().map(|l| l.parse().unwrap()).collect()
    }

    fn parse_input_default(&mut self) {
        self.parse_input(include_str!("../../inputs/day09.txt"));
    }

    fn part_1(&self) -> Option<String> {
        Some(
            self.data
                .iter()
                .map(|s| s.get_next_value())
                .sum::<i32>()
                .to_string(),
        )
    }

    fn part_2(&self) -> Option<String> {
        Some(
            self.data
                .iter()
                .map(|s| NumberSequence {
                    numbers: s.numbers.iter().rev().copied().collect_vec(),
                })
                .map(|s| s.get_next_value())
                .sum::<i32>()
                .to_string(),
        )
    }

    fn get_day_name(&self) -> String {
        "Day 09: Mirage Maintenance".into()
    }
}

#[derive(Debug, Default)]
struct NumberSequence {
    numbers: Vec<i32>,
}

impl NumberSequence {
    fn get_next_value(&self) -> i32 {
        let mut seq = self.numbers.clone();
        let mut helper = Vec::with_capacity(seq.len());
        let mut cum_sum = 0;

        while !seq.iter().all(|n| *n == 0) {
            cum_sum += seq.last().unwrap_or(&0);

            helper.clear();
            helper.extend(seq.windows(2).map(|w| w[1] - w[0]));
            seq.clear();
            seq.extend(helper.iter());
        }

        cum_sum
    }
}

impl FromStr for NumberSequence {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            numbers: s.split_whitespace().map(|l| l.parse().unwrap()).collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::AoCProblem;

    #[test]
    fn part1_test() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let mut day09 = super::Day09::default();
        day09.parse_input(input);
        assert_eq!(day09.part_1(), Some("114".into()))
    }

    #[test]
    fn part2_test() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let mut day09 = super::Day09::default();
        day09.parse_input(input);
        assert_eq!(day09.part_2(), Some("2".into()))
    }
}
