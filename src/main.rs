#![feature(never_type)]

use std::time::Instant;

use crate::day01::Day01;
use crate::day02::Day02;

mod day01;
mod day02;

fn main() {
    color_eyre::install().unwrap();

    let days: Vec<Box<dyn AoCProblem>> = vec![
        Box::<Day01>::default(),
        Box::<Day02>::default(),
    ];

    for mut day in days {
        day.print_solution();
    }
}

pub trait AoCProblem {
    fn parse_input(&mut self, input: &str);
    fn parse_input_default(&mut self);
    fn part_1(&self) -> Option<String>;
    fn part_2(&self) -> Option<String>;
    fn get_day_name(&self) -> String;

    fn print_solution(&mut self) {
        println!("{}:", self.get_day_name());

        let start = Instant::now();
        self.parse_input_default();
        println!("\tParsing input took: {:?}\n", start.elapsed());

        let start = Instant::now();
        if let Some(part_1) = self.part_1() {
            println!("\tPart 1: {}", part_1);
            println!("\tPart 1 took: {:?}\n", start.elapsed());
        }

        let start = Instant::now();
        if let Some(part_2) = self.part_2() {
            println!("\tPart 2: {}", part_2);
            println!("\tPart 2 took: {:?}\n", start.elapsed());
        }
    }
}
