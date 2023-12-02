use itertools::Itertools;

use crate::AoCProblem;

#[derive(Debug, Default)]
pub struct Day01 {
    input: Vec<String>,
}

static INPUT: &str = include_str!("../../inputs/day01.txt");

impl AoCProblem for Day01 {
    fn parse_input(&mut self, input: &str) {
        self.input = input.lines().map(|l| l.to_string()).collect();
    }

    fn parse_input_default(&mut self) {
        self.parse_input(INPUT)
    }

    fn part_1(&self) -> Option<String> {
        Some(
            self
                .input
                .iter()
                .map(|l|
                    l
                        .chars()
                        .filter(|c| c.is_ascii_digit())
                        .map(|c| c.to_digit(10).unwrap())
                        .collect_vec()
                )
                .map(|l| 10 * l.first().unwrap() + l.last().unwrap())
                .sum::<u32>()
                .to_string()
        )
    }

    fn part_2(&self) -> Option<String> {
        let needle = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].map(|s| s.chars().collect_vec());
        let eldeen = ["eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin"].map(|s| s.chars().collect_vec());

        let res = self.input.iter().map(|l| {
            let mut haystack = l.chars().collect_vec();

            let mut first = 0;
            'outer: for i in 0..l.len() {
                if haystack[i].is_ascii_digit() {
                    first = haystack[i].to_digit(10).unwrap();
                    break;
                }

                for j in 0..needle.len() {
                    // check if the needle is in the haystack
                    if i + needle[j].len() <= l.len() && haystack[i..i + needle[j].len()] == needle[j] {
                        first = (j + 1) as u32;
                        break 'outer;
                    }
                }
            }

            haystack.reverse();
            let mut last = 0;
            'outer: for i in 0..l.len() {
                if haystack[i].is_ascii_digit() {
                    last = haystack[i].to_digit(10).unwrap();
                    break;
                }

                for j in 0..eldeen.len() {
                    // check if the needle is in the haystack
                    if i + eldeen[j].len() <= l.len() && haystack[i..i + eldeen[j].len()] == eldeen[j] {
                        last = (j + 1) as u32;
                        break 'outer;
                    }
                }
            }


            first * 10 + last
        })
            .sum::<u32>()
            .to_string();

        Some(res)
    }

    fn get_day_name(&self) -> String {
        "Day 01: Trebuchet?!".into()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::AoCProblem;

    #[test]
    fn test_part_1() {
        let mut day = super::Day01::default();
        day.parse_input("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet");

        assert_eq!(day.part_1(), Some("142".into()));
    }

    #[test]
    fn test_part_2() {
        let mut day = super::Day01::default();
        day.parse_input("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen");

        assert_eq!(day.part_2(), Some("281".into()));
    }
}