use std::cmp::Ordering;
use std::str::FromStr;

use itertools::Itertools;

use crate::AoCProblem;

#[derive(Default, Debug)]
pub struct Day07 {
    data: Vec<GameHand>,
    data2: Vec<GameHand>,
}

impl AoCProblem for Day07 {
    fn parse_input(&mut self, input: &str) {
        self.data = input.lines().map(|l| l.parse().unwrap()).collect_vec();

        self.data2 = input
            .lines()
            .map(|l| l.replace('J', "$").parse().unwrap())
            .collect_vec();
    }

    fn parse_input_default(&mut self) {
        self.parse_input(include_str!("../../inputs/day07.txt"));
    }

    fn part_1(&self) -> Option<String> {
        let res = self
            .data
            .iter()
            .sorted_unstable()
            .enumerate()
            .map(|(i, h)| (i + 1) * h.bid as usize)
            .sum::<usize>();

        Some(res.to_string())
    }

    fn part_2(&self) -> Option<String> {
        let res = self
            .data2
            .iter()
            .sorted_unstable()
            .enumerate()
            .map(|(i, h)| (i + 1) * h.bid as usize)
            .sum::<usize>();

        Some(res.to_string())
    }

    fn get_day_name(&self) -> String {
        "Day 07: Camel Cards".into()
    }
}

#[derive(Debug, PartialEq, Ord, PartialOrd, Eq, Copy, Clone)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<Vec<CardKind>> for HandKind {
    fn from(value: Vec<CardKind>) -> Self {
        use CardKind::*;

        let mut counts = value.iter().counts();
        let top_card = **counts
            .iter()
            .filter(|(&v, _)| *v != Joker)
            .max_by_key(|(_, &v)| v)
            .unwrap_or((&&Joker, &5))
            .0;
        counts.remove(&Joker);

        let value = value
            .iter()
            .map(|&v| if v == Joker { top_card } else { v })
            .collect_vec();
        match counts.len() {
            0 => Self::FiveOfAKind,
            1 => Self::FiveOfAKind,
            2 => {
                let count = value.iter().filter(|&&v| v == value[0]).count();
                if count == 4 || count == 1 {
                    Self::FourOfAKind
                } else {
                    Self::FullHouse
                }
            }
            3 => {
                let count = value.iter().filter(|&&v| v == value[0]).count();
                match count {
                    3 => Self::ThreeOfAKind,
                    2 => Self::TwoPair,
                    1 => {
                        let count = value.iter().filter(|&&v| v == value[1]).count();
                        if count == 3 || count == 1 {
                            Self::ThreeOfAKind
                        } else {
                            Self::TwoPair
                        }
                    }
                    _ => unreachable!(),
                }
            }
            4 => Self::OnePair,
            5 => Self::HighCard,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Ord, PartialOrd, Eq, Copy, Clone, Hash)]
enum CardKind {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug)]
struct GameHand {
    cards: Vec<CardKind>,
    bid: u32,
    kind: HandKind,
}

impl Eq for GameHand {}
impl PartialEq for GameHand {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.cards.eq(&other.cards)
    }
}

impl Ord for GameHand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.kind == other.kind {
            self.cards.cmp(&other.cards)
        } else {
            self.kind.cmp(&other.kind)
        }
    }
}

impl PartialOrd for GameHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for GameHand {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CardKind::*;

        let (cards, bid) = s.split_once(' ').unwrap();
        let cards = cards
            .chars()
            .map(|c| match c {
                'A' => Ace,
                'K' => King,
                'Q' => Queen,
                'J' => Jack,
                'T' => Ten,
                '9' => Nine,
                '8' => Eight,
                '7' => Seven,
                '6' => Six,
                '5' => Five,
                '4' => Four,
                '3' => Three,
                '2' => Two,
                '$' => Joker,
                _ => unreachable!(),
            })
            .collect_vec();
        let hand_kind = cards.clone().into();

        Ok(Self {
            cards,
            bid: bid.parse().unwrap(),
            kind: hand_kind,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::day07::HandKind;
    use crate::AoCProblem;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn test_part1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let mut day = super::Day07::default();
        day.parse_input(input);
        assert_eq!(day.part_1(), Some("6440".into()));
        assert!(HandKind::FiveOfAKind > HandKind::FourOfAKind);
    }

    #[test]
    pub fn test_part2() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let mut day = super::Day07::default();
        day.parse_input(input);
        assert_eq!(day.part_2(), Some("5905".into()));
    }
}
