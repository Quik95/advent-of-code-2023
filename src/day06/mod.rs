use crate::AoCProblem;
use itertools::Itertools;

#[derive(Default)]
pub struct Day06 {
    data1: Vec<BoatRace>,
    data2: BoatRace,
}

impl AoCProblem for Day06 {
    fn parse_input(&mut self, input: &str) {
        let lines = input.lines().collect_vec();

        let mut l1 = lines.iter();
        let times = l1
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .filter(|x| !x.is_empty())
            .map(|x| x.trim().parse::<u64>().unwrap());
        let distances = l1
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .filter(|x| !x.is_empty())
            .map(|x| x.trim().parse::<u64>().unwrap());

        self.data1 = times
            .zip(distances)
            .map(|(t, d)| BoatRace {
                time: t,
                distance: d,
            })
            .collect_vec();

        let mut l2 = lines.iter();
        let time = l2
            .next()
            .unwrap()
            .strip_prefix("Time: ")
            .unwrap()
            .replace(' ', "")
            .parse::<u64>()
            .unwrap();
        let distance = l2
            .next()
            .unwrap()
            .strip_prefix("Distance: ")
            .unwrap()
            .replace(' ', "")
            .parse::<u64>()
            .unwrap();
        self.data2 = BoatRace { time, distance };
    }

    fn parse_input_default(&mut self) {
        self.parse_input(include_str!("../../inputs/day06.txt"));
    }

    fn part_1(&self) -> Option<String> {
        Some(
            self.data1
                .iter()
                .map(|br| br.get_number_of_winning())
                .product::<u64>()
                .to_string(),
        )
    }

    fn part_2(&self) -> Option<String> {
        Some(self.data2.get_number_of_winning().to_string())
    }

    fn get_day_name(&self) -> String {
        "Day 06: Wait For It".into()
    }
}

#[derive(Default)]
struct BoatRace {
    time: u64,
    distance: u64,
}

impl BoatRace {
    fn get_number_of_winning(&self) -> u64 {
        let determinant = ((self.time * self.time - 4 * self.distance) as f64).sqrt();
        let x1 = ((-(self.time as f64) + determinant) / -2.0 + 0.0005).ceil() as u64;
        let x2 = ((-(self.time as f64) - determinant) / -2.0 - 0.0005).floor() as u64;

        x2 - x1 + 1
    }
}

#[cfg(test)]
mod tests {
    use crate::day06::Day06;
    use crate::AoCProblem;

    #[test]
    pub fn test_part1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let mut d = Day06::default();
        d.parse_input(input);
        assert_eq!(d.part_1(), Some("288".into()))
    }

    #[test]
    pub fn test_part2() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let mut d = Day06::default();
        d.parse_input(input);
        assert_eq!(d.part_2(), Some("71503".into()))
    }
}
