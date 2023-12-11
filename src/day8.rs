use core::str::FromStr;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../assets/day8Input.txt");
    let part_1 = run_part_1(input);
    println!("part 1: {part_1}");

    let part_2 = run_part_2(input);
    println!("part 2: {part_2}");
}

fn gcd(a: usize, b: usize) -> usize {
    let (mut r_n_1, mut r_n) = if a > b { (a, b) } else { (b, a) };

    while r_n != 0 {
        let r = r_n_1 % r_n;
        r_n_1 = r_n;
        r_n = r;
    }

    r_n_1
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn run_part_1(input: &str) -> usize {
    let map: Map = input.parse().unwrap();

    map.follow_instructions()
}

fn run_part_2(input: &str) -> usize {
    let map: Map = input.parse().unwrap();

    map.follow_ghost_instructions()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Left,
    Right,
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(format!(
                "Expected exactly 1 character for an instruction, found {}",
                s.len()
            ));
        }
        match s {
            "L" => Ok(Instruction::Left),
            "R" => Ok(Instruction::Right),
            o => Err(format!("Instruction must be one of 'L' or 'R', found {o}")),
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn next(&self, instruction: Instruction) -> &str {
        match instruction {
            Instruction::Left => &self.left,
            Instruction::Right => &self.right,
        }
    }
}

impl FromStr for Node {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('=');
        let name = split
            .next()
            .ok_or_else(|| "expected one '=' in node line".to_string())?
            .trim()
            .to_string();

        let mut edges = split
            .next()
            .ok_or_else(|| "expected one '=' in node line".to_string())?
            .trim()
            .trim_matches(|c| c == '(' || c == ')')
            .split(", ");
        let left = edges
            .next()
            .ok_or_else(|| "expected 1 ', ' in the node line")?
            .to_string();
        let right = edges
            .next()
            .ok_or_else(|| "expected 1 ', ' in the node line")?
            .to_string();
        Ok(Node { name, left, right })
    }
}

#[derive(Debug, Clone)]
struct Map {
    instructions: Vec<Instruction>,
    nodes: HashMap<String, Node>,
}

impl Map {
    fn follow_instructions(&self) -> usize {
        let mut n = 0;
        let mut curr_name = "AAA";
        while curr_name != "ZZZ" {
            let instr = self.instructions[n % self.instructions.len()];

            curr_name = self.nodes[curr_name].next(instr);
            n += 1;
        }
        n
    }

    fn follow_node_until_ghost_stops(&self, start: &str) -> usize {
        let mut n = 0;
        let mut current = start;
        while !current.ends_with('Z') {
            let instr = self.instructions[n % self.instructions.len()];

            current = self.nodes[current].next(instr);

            n += 1;
        }
        n
    }

    fn follow_ghost_instructions(&self) -> usize {
        // First ending points for each start
        // [17287, 17873, 13771, 19631, 20803, 23147]

        // then get the LCM?
        // not exactly, because I don't know if they actually cycle there regularly or not

        // The first 10 ending points for each start
        // [
        //     [20803, 41606, 62409, 83212, 104015, 124818, 145621, 166424, 187227, 208030],
        //          20803   20803  ...
        //     [19631, 39262, 58893, 78524, 98155, 117786, 137417, 157048, 176679, 196310],
        //          19631   19631  ...
        //     [17873, 35746, 53619, 71492, 89365, 107238, 125111, 142984, 160857, 178730],
        //          17873   17873  ...
        //     [13771, 27542, 41313, 55084, 68855, 82626, 96397, 110168, 123939, 137710],
        //          13771   13771  ...
        //     [23147, 46294, 69441, 92588, 115735, 138882, 162029, 185176, 208323, 231470],
        //          23147   ...
        //     [17287, 34574, 51861, 69148, 86435, 103722, 121009, 138296, 155583, 172870]
        //          17287   ...
        // ]

        // so I could probably get away with doing the LCM but I don't like its lack of rigor

        // n / instructions.len():
        // [79.0, 67.0, 61.0, 71.0, 59.0, 47.0]

        self.nodes
            .keys()
            .filter(|k| k.ends_with('A'))
            .map(|s| self.follow_node_until_ghost_stops(s))
            .reduce(|acc, n| lcm(acc, n))
            .unwrap()
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stanzas = s.split("\n\n");

        let instruction_str = stanzas
            .next()
            .ok_or_else(|| format!("Expected at least 1 stanza"))?;
        let instructions: Vec<Instruction> = instruction_str
            .chars()
            .map(|c| c.to_string().parse())
            .collect::<Result<_, _>>()?;

        let nodes: HashMap<String, Node> = stanzas
            .next()
            .ok_or_else(|| format!("Expected 2 stanzas"))?
            .lines()
            .map(|l| {
                let node: Node = l.parse()?;
                Ok((node.name.clone(), node))
            })
            .collect::<Result<HashMap<_, _>, String>>()?;

        Ok(Map {
            instructions,
            nodes,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_known_input() {
        let input = r##"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"##;

        assert_eq!(run_part_1(input), 2);
    }

    #[test]
    fn part_1_knonw_input_2() {
        let input = r##"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"##;

        assert_eq!(run_part_1(input), 6);
    }

    #[test]
    fn part_2_known_input() {
        let input = r##"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"##;

        assert_eq!(run_part_2(input), 6);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(1, 2), 1);
        assert_eq!(gcd(2, 1), 1);
        assert_eq!(gcd(2, 4), 2);
        assert_eq!(gcd(4, 2), 2);
        assert_eq!(gcd(15, 3), 3);
        assert_eq!(gcd(3, 7), 1);
    }
}
