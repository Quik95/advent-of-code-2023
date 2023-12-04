use crate::AoCProblem;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Default)]
pub struct Day04 {
    data: Vec<ScratchCard>,
}

impl AoCProblem for Day04 {
    fn parse_input(&mut self, input: &str) {
        self.data = input.lines().map(|l| l.parse().unwrap()).collect_vec();
    }

    fn parse_input_default(&mut self) {
        self.parse_input(include_str!("../../inputs/day04.txt"));
    }

    fn part_1(&self) -> Option<String> {
        let winning_cards: u32 = self.data.iter().map(|c| c.get_card_value()).sum();
        Some(winning_cards.to_string())
    }

    fn part_2(&self) -> Option<String> {
        let mut store = (0..self.data.len()).map(|_| 1).collect_vec();

        let rewards = self
            .data
            .iter()
            .map(|sc| sc.scratch.iter().filter(|n| sc.winning.contains(n)).count())
            .collect_vec();

        let mut to_process = (0..self.data.len()).collect_vec();
        while let Some(card) = to_process.pop() {
            let reward = rewards[card];
            for next_card in card + 1..=card + reward {
                store[next_card] += 1;

                to_process.push(next_card);
            }
        }

        let total_cards = store.iter().sum::<u64>();

        Some(total_cards.to_string())
    }

    fn get_day_name(&self) -> String {
        "Day 04: Scratchcards".into()
    }
}

struct ScratchCard {
    winning: Vec<u8>,
    scratch: Vec<u8>,
}

impl ScratchCard {
    fn get_card_value(&self) -> u32 {
        let pow = self
            .scratch
            .iter()
            .filter(|n| self.winning.contains(n))
            .count();
        if pow == 0 {
            return 0;
        }
        2_u32.pow(pow as u32 - 1)
    }
}

impl FromStr for ScratchCard {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let card_number_end = s.find(": ").unwrap();
        let mut parts = s[card_number_end + 2..].split(" | ");

        let scratch = parts
            .next()
            .unwrap()
            .split(' ')
            .filter(|c| !c.is_empty())
            .map(|n| n.parse().unwrap())
            .collect();
        let winning = parts
            .next()
            .unwrap()
            .split(' ')
            .filter(|c| !c.is_empty())
            .map(|n| n.parse().unwrap())
            .collect();

        Ok(ScratchCard { winning, scratch })
    }
}

#[cfg(test)]
mod tests {
    use crate::AoCProblem;
    use pretty_assertions::assert_eq;

    #[test]
    fn part1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let mut day04 = super::Day04::default();
        day04.parse_input(input);
        assert_eq!(day04.part_1(), Some("13".into()));
    }

    #[test]
    fn part2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let mut day04 = super::Day04::default();
        day04.parse_input(input);
        assert_eq!(day04.part_2(), Some("30".into()));
    }
}
