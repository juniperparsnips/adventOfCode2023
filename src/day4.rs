use core::{ops::Add, str::FromStr};
use std::collections::HashSet;

fn main() {
    let input = include_str!("../assets/day4Input.txt");
    // let part1_res = run_part_1(input);
    // println!("part 1: {part1_res}");

    let part2_res = run_part_2(input);
    println!("part 2: {part2_res}");
}

fn run_part_1(input: &str) -> u32 {
    let cards = Card::read_input(input).unwrap();

    cards.iter().map(Card::score).reduce(Add::add).unwrap()
}

fn run_part_2(input: &str) -> u32 {
    let cards = Card::read_input(input).unwrap();

    let mut instances_of_card = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        let num_matches = card.num_matches() as usize;
        // the game asserts that the number of matches will not make a card produce past the end
        for j in (i + 1)..(i + num_matches + 1) {
            instances_of_card[j] += instances_of_card[i];
        }
    }

    instances_of_card.into_iter().reduce(Add::add).unwrap()
}

#[derive(Debug, Clone)]
struct Card {
    id: u32,
    winning: HashSet<u32>,
    ours: HashSet<u32>,
}

impl Card {
    fn read_input(input: &str) -> Result<Vec<Card>, String> {
        input.lines().map(|l| l.parse()).collect()
    }

    fn score(&self) -> u32 {
        let num_matches = self.num_matches();

        if num_matches >= 1 {
            2_usize.pow(num_matches - 1) as u32
        } else {
            0
        }
    }

    fn num_matches(&self) -> u32 {
        self.winning.intersection(&self.ours).count() as u32
    }
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colon_split: Vec<_> = s.split(':').collect();
        if colon_split.len() != 2 {
            return Err("must have exactly 1 colon in card str".to_string());
        }
        let header = colon_split[0];
        let body = colon_split[1];

        let header_split: Vec<_> = header.split_ascii_whitespace().collect();
        if header_split[0] != "Card" {
            return Err("card str must begin with 'Card'".to_string());
        }
        if header_split.len() != 2 {
            return Err("Header must be of format 'Card <id>:'".to_string());
        }
        let id = u32::from_str_radix(header_split[1], 10).or_else(|e| {
            return Err(format!("Card id must be a number: {e}"));
        })?;

        let body_split: Vec<_> = body.split('|').collect();
        if body_split.len() != 2 {
            return Err("Body must have exactly 1 '|'".to_string());
        }
        let winning = body_split[0]
            .split_ascii_whitespace()
            .map(|n| u32::from_str_radix(n, 10).unwrap())
            .collect();
        let ours = body_split[1]
            .split_ascii_whitespace()
            .map(|n| u32::from_str_radix(n, 10).unwrap())
            .collect();

        Ok(Card { id, winning, ours })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_known_input() {
        let input = r##"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"##;

        assert_eq!(run_part_1(input), 13)
    }

    #[test]
    fn part2_known_input() {
        let input = r##"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"##;

        assert_eq!(run_part_2(input), 30)
    }
}
