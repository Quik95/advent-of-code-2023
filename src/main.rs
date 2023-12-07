#![feature(never_type)]
#![feature(iter_map_windows)]

use std::time::{Duration, Instant};

use crate::day01::Day01;
use crate::day02::Day02;
use crate::day03::Day03;
use crate::day04::Day04;
use crate::day05::Day05;
use crate::day06::Day06;
use crate::day07::Day07;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

fn main() {
    color_eyre::install().unwrap();

    let days: Vec<Box<dyn AoCProblem>> = vec![
        Box::<Day01>::default(),
        Box::<Day02>::default(),
        Box::<Day03>::default(),
        Box::<Day04>::default(),
        Box::<Day05>::default(),
        Box::<Day06>::default(),
        Box::<Day07>::default(),
    ];

    let n_days = days.len();
    let total = days
        .into_iter()
        .fold(Duration::from_secs(0), |acc, mut day| {
            acc + day.print_solution()
        });
    println!("Total time for {} days: {:?}", n_days, total);
}

pub trait AoCProblem {
    fn parse_input(&mut self, input: &str);
    fn parse_input_default(&mut self);
    fn part_1(&self) -> Option<String>;
    fn part_2(&self) -> Option<String>;
    fn get_day_name(&self) -> String;

    fn print_solution(&mut self) -> Duration {
        println!("{}:", self.get_day_name());

        let start = Instant::now();
        self.parse_input_default();
        let elapsed_parse = start.elapsed();
        println!("\tParsing input took: {:?}\n", elapsed_parse);

        let start = Instant::now();
        let mut elapsed_part1 = Duration::from_secs(0);
        if let Some(part_1) = self.part_1() {
            println!("\tPart 1: {}", part_1);
            elapsed_part1 = start.elapsed();
            println!("\tPart 1 took: {:?}\n", elapsed_part1);
        }

        let start = Instant::now();
        let mut elapsed_part2 = Duration::from_secs(0);
        if let Some(part_2) = self.part_2() {
            println!("\tPart 2: {}", part_2);
            elapsed_part2 = start.elapsed();
            println!("\tPart 2 took: {:?}\n", elapsed_part2);
        }

        let total_duration = elapsed_parse + elapsed_part1 + elapsed_part2;
        println!("\tTotal time: {:?}\n", total_duration);

        total_duration
    }
}
