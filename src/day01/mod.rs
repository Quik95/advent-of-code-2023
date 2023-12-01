use itertools::Itertools;
use crate::AoCProblem;

#[derive(Debug, Default)]
pub struct Day01 {
    input: Vec<String>,
}

static INPUT: &str = include_str!("../../inputs/day01.txt");

impl AoCProblem for Day01 {
    fn parse_input(&mut self) {
        self.input = INPUT.lines().map(|l| l.to_string()).collect();
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
        let res = self.input.iter().map(|l| {
            let mut line_res = vec![];
            let chars = l.chars().collect_vec();

            for i in 0..chars.len() {
                if chars[i].is_ascii_digit() {
                    line_res.push(chars[i].to_digit(10).unwrap());
                }

                if i + 3 <= chars.len() {
                    if chars[i..i + 3].iter().collect::<String>() == "one" {
                        line_res.push(1);
                    }

                    if chars[i..i + 3].iter().collect::<String>() == "two" {
                        line_res.push(2);
                    }

                    if chars[i..i + 3].iter().collect::<String>() == "six" {
                        line_res.push(6);
                    }
                }

                if i + 4 <= chars.len() {
                    if chars[i..i + 4].iter().collect::<String>() == "four" {
                        line_res.push(4);
                    }

                    if chars[i..i + 4].iter().collect::<String>() == "five" {
                        line_res.push(5);
                    }

                    if chars[i..i + 4].iter().collect::<String>() == "nine" {
                        line_res.push(9);
                    }
                }

                if i + 5 <= chars.len() {
                    if chars[i..i + 5].iter().collect::<String>() == "three" {
                        line_res.push(3);
                    }


                    if chars[i..i + 5].iter().collect::<String>() == "seven" {
                        line_res.push(7);
                    }

                    if chars[i..i + 5].iter().collect::<String>() == "eight" {
                        line_res.push(8);
                    }
                }
            }

            line_res
        })
            .map(|l| 10 * l.first().unwrap() + l.last().unwrap())
            .sum::<u32>()
            .to_string();

        Some(res)
    }

    fn get_day_name(&self) -> String {
        "Day 01: Trebuchet?!".into()
    }
}