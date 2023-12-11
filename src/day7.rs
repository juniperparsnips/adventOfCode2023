use core::{cmp::Ordering, fmt::Debug, str::FromStr};

fn main() {
    let input = include_str!("../assets/day7Input.txt");
    let part_1 = run_part_1(input);
    println!("part 1: {part_1}");
    let part_2 = run_part_2(input);
    println!("part 2: {part_2}");
}

fn run_part_1(input: &str) -> usize {
    let mut game: Game<Card> = input.parse().unwrap();

    game.0.sort_unstable();

    game.0
        .iter()
        .enumerate()
        .map(|(i, h)| h.bid * (1 + i))
        .sum()
}

fn run_part_2(input: &str) -> usize {
    let mut game: Game<JokerCard> = input.parse().unwrap();

    game.0.sort_unstable();

    game.0
        .iter()
        .enumerate()
        .map(|(i, h)| h.bid * (1 + i))
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(usize)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl FromStr for Card {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(format!("Expected exactly 1 char, found: {}", s.len()));
        }
        match s {
            "2" => Ok(Card::Two),
            "3" => Ok(Card::Three),
            "4" => Ok(Card::Four),
            "5" => Ok(Card::Five),
            "6" => Ok(Card::Six),
            "7" => Ok(Card::Seven),
            "8" => Ok(Card::Eight),
            "9" => Ok(Card::Nine),
            "T" => Ok(Card::Ten),
            "J" => Ok(Card::Jack),
            "Q" => Ok(Card::Queen),
            "K" => Ok(Card::King),
            "A" => Ok(Card::Ace),
            o => Err(format!("'{o}' is not a card")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(usize)]
enum JokerCard {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl FromStr for JokerCard {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(format!("Expected exactly 1 char, found: {}", s.len()));
        }
        match s {
            "J" => Ok(JokerCard::Joker),
            "2" => Ok(JokerCard::Two),
            "3" => Ok(JokerCard::Three),
            "4" => Ok(JokerCard::Four),
            "5" => Ok(JokerCard::Five),
            "6" => Ok(JokerCard::Six),
            "7" => Ok(JokerCard::Seven),
            "8" => Ok(JokerCard::Eight),
            "9" => Ok(JokerCard::Nine),
            "T" => Ok(JokerCard::Ten),
            "Q" => Ok(JokerCard::Queen),
            "K" => Ok(JokerCard::King),
            "A" => Ok(JokerCard::Ace),
            o => Err(format!("'{o}' is not a card")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&[Card; 5]> for HandType {
    fn from(value: &[Card; 5]) -> Self {
        let mut appearances: [usize; 13] = [0; 13];
        for card in value {
            appearances[*card as usize] += 1;
        }
        let max = appearances.iter().max().unwrap();
        match max {
            1 => return HandType::HighCard,
            2 => match appearances.iter().filter(|a| **a == 2).count() {
                1 => return HandType::OnePair,
                2 => return HandType::TwoPair,
                _ => panic!("unreachable"),
            },
            3 => match appearances.iter().filter(|a| **a == 2).count() {
                1 => return HandType::FullHouse,
                0 => return HandType::ThreeOfAKind,
                _ => panic!("unreachable"),
            },
            4 => return HandType::FourOfAKind,
            5 => return HandType::FiveOfAKind,
            _ => panic!("unreachable"),
        }
    }
}

impl From<&[JokerCard; 5]> for HandType {
    fn from(value: &[JokerCard; 5]) -> Self {
        let mut appearances: [usize; 13] = [0; 13];
        for card in value {
            appearances[*card as usize] += 1;
        }
        let num_jokers = appearances[0];
        let max = appearances[1..].iter().max().unwrap();
        match max + num_jokers {
            1 => return HandType::HighCard,
            2 => {
                let jokerless_pairs = appearances.iter().filter(|a| **a == 2).count();
                let pairs = if num_jokers != 0 {
                    jokerless_pairs + 1
                } else {
                    jokerless_pairs
                };
                match pairs {
                    1 => return HandType::OnePair,
                    2 => return HandType::TwoPair,
                    _ => panic!("unreachable"),
                }
            }
            3 => {
                let jokerless_pairs = appearances.iter().filter(|a| **a == 2).count();
                let pairs = if num_jokers != 0 {
                    jokerless_pairs - 1
                } else {
                    jokerless_pairs
                };
                match pairs {
                    1 => return HandType::FullHouse,
                    0 => return HandType::ThreeOfAKind,
                    _ => panic!("unreachable"),
                }
            }
            4 => return HandType::FourOfAKind,
            5 => return HandType::FiveOfAKind,
            _ => panic!("unreachable"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand<C> {
    cards: [C; 5],
    bid: usize,
    hand_type: HandType,
}

impl<C> PartialOrd for Hand<C>
where
    C: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(Ordering::Equal) => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .map(|(s, o)| s.partial_cmp(o))
                .find(|o| *o != Some(Ordering::Equal))
                .flatten(),
            o => o,
        }
    }
}

impl<C> Ord for Hand<C>
where
    C: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<C> FromStr for Hand<C>
where
    C: FromStr<Err = String> + Debug,
    for<'a> HandType: From<&'a [C; 5]>,
{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_ascii_whitespace();

        let cards: Vec<C> = split
            .next()
            .ok_or_else(|| "expected at least 1 word in line".to_string())?
            .chars()
            .map(|c| c.to_string().parse())
            .collect::<Result<_, String>>()?;

        if cards.len() != 5 {
            return Err(format!(
                "Expected exactly 5 cards in a hand, found: {}",
                cards.len()
            ));
        }

        let bid: usize = split
            .next()
            .ok_or_else(|| "expected at least 1 word in line".to_string())?
            .parse()
            .map_err(|e| format!("{e}"))?;
        let cards: [C; 5] = cards.try_into().map_err(|e| format!("{e:?}"))?;
        let hand_type = (&cards).into();

        Ok(Hand {
            cards,
            bid,
            hand_type,
        })
    }
}

struct Game<C>(Vec<Hand<C>>);

impl<C> FromStr for Game<C>
where
    C: FromStr<Err = String> + Debug,
    for<'a> HandType: From<&'a [C; 5]>,
{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hands: Vec<Hand<C>> = s
            .lines()
            .map(|l| l.parse())
            .collect::<Result<_, String>>()?;
        Ok(Game(hands))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_known_answer() {
        let input = r##"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"##;

        assert_eq!(run_part_1(input), 6440);
    }

    #[test]
    fn part_2_known_answer() {
        let input = r##"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"##;

        assert_eq!(run_part_2(input), 5905);
    }
}
