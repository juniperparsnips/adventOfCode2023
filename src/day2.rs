use core::str::FromStr;

fn main() {
    let input = include_str!("../assets/day2Input.txt");

    let limit = GameLimits::new(12, 13, 14);
    let illegal_games = run_part_1(input, limit);
    println!("illegal_games: {illegal_games}");
    let sum_of_games_failed = run_part_2(input);
    println!("games failed: {sum_of_games_failed}");
}

fn run_part_1(input: &str, limit: GameLimits) -> u32 {
    let games: Vec<Game> = input.lines().map(|line| line.parse().unwrap()).collect();

    games
        .iter()
        .filter_map(|g| {
            if !g.is_illegal(&limit) {
                Some(g.id)
            } else {
                None
            }
        })
        .sum()
}

fn run_part_2(input: &str) -> u32 {
    let games: Vec<Game> = input.lines().map(|line| line.parse().unwrap()).collect();

    games.iter().map(|g| g.power()).sum()
}

#[derive(Debug, Clone, Copy)]
struct GameLimits {
    num_of_colors: Hand,
}

impl GameLimits {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        GameLimits {
            num_of_colors: Hand::new(red, green, blue),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Hand {
    num_red: u32,
    num_green: u32,
    num_blue: u32,
}

impl Hand {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Hand {
            num_red: red,
            num_green: green,
            num_blue: blue,
        }
    }

    fn is_greater(&self, other: &Self) -> bool {
        self.num_red > other.num_red
            || self.num_green > other.num_green
            || self.num_blue > other.num_blue
    }
}

impl FromStr for Hand {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colors = s.split(',');

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for color_str in colors {
            let split: Vec<_> = color_str.split_ascii_whitespace().collect();
            if split.len() != 2 {
                return Err(format!("Expected 2 elements in a color amount: {split:?}"));
            } else {
                let num = u32::from_str_radix(split[0], 10)
                    .map_err(|e| format!("number of each color must be a decimal number: {e}"))?;

                match split[1] {
                    "red" => red += num,
                    "green" => green += num,
                    "blue" => blue += num,
                    other => return Err(format!("Unexpected color '{other}'")),
                }
            }
        }

        Ok(Hand::new(red, green, blue))
    }
}

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    hands: Vec<Hand>,
}

impl Game {
    fn is_illegal(&self, limits: &GameLimits) -> bool {
        for hand in &self.hands {
            if hand.is_greater(&limits.num_of_colors) {
                return true;
            }
        }
        return false;
    }

    fn min_game_size(&self) -> Hand {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for hand in &self.hands {
            if hand.num_red > max_red {
                max_red = hand.num_red;
            }
            if hand.num_green > max_green {
                max_green = hand.num_green;
            }
            if hand.num_blue > max_blue {
                max_blue = hand.num_blue;
            }
        }

        Hand {
            num_red: max_red,
            num_green: max_green,
            num_blue: max_blue,
        }
    }

    fn power(&self) -> u32 {
        let Hand {
            num_red,
            num_green,
            num_blue,
        } = self.min_game_size();
        num_red * num_green * num_blue
    }
}

impl FromStr for Game {
    type Err = String;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let colon_split: Vec<_> = line.split(':').collect();

        if colon_split.len() != 2 {
            return Err("there must be exactly 1 ':' in a game line".to_string());
        }

        let header = colon_split[0];
        let hands_str = colon_split[1];

        let magic_match = header.match_indices("Game").next();
        if let Some((0, _)) = magic_match {
            // continue (we can't invert an `if let Some(_) pattern`)
        } else {
            return Err("Lines must begin with 'Game'".to_string());
        }
        let id_str = header.split_ascii_whitespace().last();
        if id_str.is_none() {
            return Err("Lines must have an id 'Game <id>'".to_string());
        }
        let id = u32::from_str_radix(id_str.unwrap(), 10)
            .map_err(|e| format!("game id must be a decimal number: {e}"))?;

        let hands_res: Vec<_> = hands_str.split(";").map(Hand::from_str).collect();
        let mut hands = Vec::with_capacity(hands_res.len());
        for res in hands_res {
            if let Ok(hand) = res {
                hands.push(hand);
            } else {
                return Err(format!("failed parsing hand: {res:?}"));
            }
        }

        Ok(Game { id, hands })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_known_input() {
        let input = r##"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"##;
        let limit = GameLimits::new(12, 13, 14);

        assert_eq!(run_part_1(input, limit), 8)
    }

    #[test]
    fn part2_known_input() {
        let input = r##"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"##;

        assert_eq!(run_part_2(input), 2286)
    }

    #[test]
    fn positive_1_color_over() {
        let game = Game {
            id: 1,
            hands: vec![Hand::new(14, 2, 10)],
        };
        let limits = GameLimits::new(10, 100, 100);
        assert!(game.is_illegal(&limits))
    }

    #[test]
    fn negative_all_colors_under() {
        let game = Game {
            id: 1,
            hands: vec![Hand::new(0, 2, 10)],
        };
        let limits = GameLimits::new(10, 100, 100);
        assert!(!game.is_illegal(&limits))
    }

    #[test]
    fn negative_all_colors_equal() {
        let game = Game {
            id: 1,
            hands: vec![Hand::new(1, 2, 3)],
        };
        let limits = GameLimits::new(1, 2, 3);
        assert!(!game.is_illegal(&limits))
    }

    #[test]
    fn positive_only_1_bad_hand() {
        let mut game = Game {
            id: 1,
            hands: vec![Hand::new(0, 0, 0); 100],
        };
        game.hands[99] = Hand::new(0, 0, 1000);
        let limits = GameLimits::new(100, 100, 100);
        assert!(game.is_illegal(&limits))
    }
}
