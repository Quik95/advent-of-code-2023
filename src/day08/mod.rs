use std::collections::HashMap;
use std::str::FromStr;

use itertools::Itertools;
use nom::{Finish, IResult, InputTake};
use nom::bytes::complete::{tag, take};
use nom::combinator::map;
use nom::sequence::{delimited, separated_pair};

use crate::AoCProblem;

#[derive(Default)]
pub struct Day08 {
    path: Vec<char>,
    network: Network,
}

impl AoCProblem for Day08 {
    fn parse_input(&mut self, input: &str) {
        let mut parts = input.split("\n\n");
        self.path = parts.next().unwrap().chars().collect_vec();
        self.network = parts.next().unwrap().parse().unwrap();
    }

    fn parse_input_default(&mut self) {
        self.parse_input(include_str!("../../inputs/day08.txt"))
    }

    fn part_1(&self) -> Option<String> {
        let mut current = self.network.nodes.get("AAA").unwrap();
        let mut path = self.path.iter().cycle();
        let mut count = 0;
        while current.label != "ZZZ" {
            let dir = path.next().unwrap();
            match dir {
                'L' => current = self.network.nodes.get(&current.left).unwrap(),
                'R' => current = self.network.nodes.get(&current.right).unwrap(),
                _ => unreachable!(),
            }
            count += 1;
        }

        Some(count.to_string())
    }

    fn part_2(&self) -> Option<String> {
        let starts = self
            .network
            .nodes
            .keys()
            .filter(|k| k.ends_with('A'))
            .collect_vec();
        let counts = starts.iter().map(|&s| {
            let mut current = s;
            let mut path = self.path.iter().cycle();
            let mut count = 0;
            while !current.ends_with('Z') {
                let dir = path.next().unwrap();
                let node = self.network.nodes.get(current).unwrap();
                match dir {
                    'L' => current = &node.left,
                    'R' => current = &node.right,
                    _ => unreachable!(),
                };
                count += 1;
            }
            count
        });

        let lcm = counts.reduce(lcm).unwrap();

        Some(lcm.to_string())
    }

    fn get_day_name(&self) -> String {
        "Day 08: Haunted Wasteland".into()
    }
}

type NodeLabel = String;

#[derive(Default)]
struct Network {
    nodes: HashMap<NodeLabel, Node>,
}

impl FromStr for Network {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nodes = s
            .lines()
            .map(|line| {
                let node: Node = line.parse().unwrap();
                (node.label.clone(), node)
            })
            .collect();

        Ok(Self { nodes })
    }
}

struct Node {
    label: NodeLabel,
    left: NodeLabel,
    right: NodeLabel,
}

impl FromStr for Node {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, node) = parse_node(s).finish().unwrap();
        Ok(node)
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn parse_node_label(i: &str) -> IResult<&str, &str> {
    take(3_usize)(i)
}

fn parse_node_exits(i: &str) -> IResult<&str, (&str, &str)> {
    let f = separated_pair(take(3_usize), tag(", "), take(3_usize));

    delimited(tag("("), f, tag(")"))(i)
}

fn parse_node(i: &str) -> IResult<&str, Node> {
    let (i, label) = parse_node_label(i)?;
    let (i, _) = map(tag(" = "), drop)(i)?;
    let (i, (l, r)) = parse_node_exits(i)?;
    Ok((
        i,
        Node {
            label: label.into(),
            left: l.into(),
            right: r.into(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use crate::AoCProblem;
    use pretty_assertions::assert_eq;

    #[test]
    fn part1_test() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let mut problem = super::Day08::default();
        problem.parse_input(input);
        assert_eq!(problem.part_1(), Some("2".into()));
    }

    #[test]
    fn part1_test2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let mut problem = super::Day08::default();
        problem.parse_input(input);
        assert_eq!(problem.part_1(), Some("6".into()));
    }

    #[test]
    fn part2_test() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let mut problem = super::Day08::default();
        problem.parse_input(input);
        assert_eq!(problem.part_2(), Some("6".into()));
    }
}
