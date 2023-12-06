use std::str::FromStr;

use itertools::Itertools;

use crate::AoCProblem;

#[derive(Default)]
pub struct Day05 {
    seeds: Vec<u64>,
    maps: Vec<SeedMap>,
}

impl AoCProblem for Day05 {
    fn parse_input(&mut self, input: &str) {
        let mut parts = input.split("\n\n");

        let seeds = parts
            .next()
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split(' ')
            .map(|s| s.parse::<u64>().unwrap())
            .collect_vec();
        let maps = parts.map(|p| p.parse::<SeedMap>().unwrap()).collect_vec();

        self.seeds = seeds;
        self.maps = maps;
    }

    fn parse_input_default(&mut self) {
        self.parse_input(include_str!("../../inputs/day05.txt"));
    }

    fn part_1(&self) -> Option<String> {
        let mut seeds = self.seeds.clone();
        for map in &self.maps {
            seeds = map.map_seeds(&seeds);
        }

        Some(seeds.iter().min().unwrap().to_string())
    }

    fn part_2(&self) -> Option<String> {
        None
    }

    fn get_day_name(&self) -> String {
        "Day 05: If You Give a Seed a Fertilizer".into()
    }
}

struct SeedMap {
    name: String,
    mappings: Vec<Mapping>,
}

impl SeedMap {
    fn map_seeds(&self, seeds: &[u64]) -> Vec<u64> {
        seeds.iter().map(|s| self.map_seed(*s)).collect_vec()
    }

    fn map_seed(&self, seed: u64) -> u64 {
        for mapping in &self.mappings {
            if let Some(new_seed) = mapping.apply_mapping_to_seed(seed) {
                return new_seed;
            }
        }
        seed
    }
}

impl FromStr for SeedMap {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let name = lines.next().unwrap().strip_suffix(':').unwrap().into();
        let mappings = lines.map(|l| l.parse().unwrap()).collect_vec();

        Ok(Self { name, mappings })
    }
}

struct Mapping {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl Mapping {
    const fn apply_mapping_to_seed(&self, seed: u64) -> Option<u64> {
        if seed >= self.source_range_start && seed < self.source_range_start + self.range_length {
            Some(self.destination_range_start + (seed - self.source_range_start))
        } else {
            None
        }
    }
}

impl FromStr for Mapping {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');

        Ok(Self {
            destination_range_start: parts.next().unwrap().parse().unwrap(),
            source_range_start: parts.next().unwrap().parse().unwrap(),
            range_length: parts.next().unwrap().parse().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::AoCProblem;

    #[test]
    pub fn test_part1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let mut day05 = crate::day05::Day05::default();
        day05.parse_input(input);
        assert_eq!(day05.part_1(), Some("35".into()));
    }

    #[test]
    pub fn test_part2() {
        let input = "seeds: 79 14

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let mut day05 = crate::day05::Day05::default();
        day05.parse_input(input);
        // assert_eq!(day05.part_2(), Some("46".into()));
    }
}
