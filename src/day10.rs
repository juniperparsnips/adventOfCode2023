use core::str::FromStr;
use std::fmt::Display;

fn main() {
    let input = include_str!("../assets/day10Input.txt");
    let part_1 = run_part_1(input);
    println!("part 1: {part_1}");

    let part_2 = run_part_2(input);
    println!("part 2: {part_2}");
}

fn run_part_1(input: &str) -> usize {
    let mut map: Map = input.parse().unwrap();

    map.traverse_loop() / 2
}

fn run_part_2(input: &str) -> usize {
    let mut map: Map = input.parse().unwrap();

    map.partition();
    map.make_all_unclassified_inside();
    map.count_inside()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Vertical,
    Horizontal,
    NorthToEast,
    NorthToWest,
    SouthToWest,
    SouthToEast,
    Ground,
    Start,
}

impl Tile {
    /// Returns the indices of the next tile, and the next "from" direction
    fn next(&self, from: Direction, row: usize, col: usize) -> (usize, usize, Direction) {
        match self {
            Tile::Vertical => match from {
                Direction::North => (row + 1, col, Direction::North),
                Direction::South => (row - 1, col, Direction::South),
                _ => panic!("invalid!"),
            },
            Tile::Horizontal => match from {
                Direction::East => (row, col - 1, Direction::East),
                Direction::West => (row, col + 1, Direction::West),
                _ => panic!("invalid!"),
            },
            Tile::NorthToEast => match from {
                Direction::North => (row, col + 1, Direction::West),
                Direction::East => (row - 1, col, Direction::South),
                _ => panic!("invalid!"),
            },
            Tile::NorthToWest => match from {
                Direction::North => (row, col - 1, Direction::East),
                Direction::West => (row - 1, col, Direction::South),
                _ => panic!("invalid!"),
            },
            Tile::SouthToWest => match from {
                Direction::South => (row, col - 1, Direction::East),
                Direction::West => (row + 1, col, Direction::North),
                _ => panic!("invalid!"),
            },
            Tile::SouthToEast => match from {
                Direction::South => (row, col + 1, Direction::West),
                Direction::East => (row + 1, col, Direction::North),
                _ => panic!("invalid!"),
            },
            _ => panic!("invalid!"),
        }
    }

    fn connects(&self, towards: Direction) -> bool {
        match self {
            Tile::Vertical => match towards {
                Direction::North | Direction::South => true,
                _ => false,
            },
            Tile::Horizontal => match towards {
                Direction::East | Direction::West => true,
                _ => false,
            },
            Tile::NorthToEast => match towards {
                Direction::East | Direction::North => true,
                _ => false,
            },
            Tile::NorthToWest => match towards {
                Direction::West | Direction::North => true,
                _ => false,
            },
            Tile::SouthToEast => match towards {
                Direction::East | Direction::South => true,
                _ => false,
            },
            Tile::SouthToWest => match towards {
                Direction::West | Direction::South => true,
                _ => false,
            },
            Tile::Ground => false,
            Tile::Start => true,
        }
    }

    // &self should be the left when towards is North/South or the upper when towards is East/West
    fn can_sneak(&self, other: &Self, towards: Direction) -> bool {
        let (this_dir, other_dir) = match towards {
            Direction::East | Direction::West => (Direction::South, Direction::North),
            Direction::North | Direction::South => (Direction::East, Direction::West),
        };

        !(self.connects(this_dir) && other.connects(other_dir))
    }
}

impl TryFrom<Vec<Direction>> for Tile {
    type Error = String;
    fn try_from(mut value: Vec<Direction>) -> Result<Self, Self::Error> {
        if value.len() != 2 {
            return Err(format!(
                "Expected exactly 2 directions to create a tile, found '{}'",
                value.len()
            ));
        }

        value.sort();

        let d1 = value[0];
        let d2 = value[1];

        match (d1, d2) {
            (Direction::North, Direction::South) => Ok(Tile::Vertical),
            (Direction::North, Direction::East) => Ok(Tile::NorthToEast),
            (Direction::North, Direction::West) => Ok(Tile::NorthToWest),
            (Direction::South, Direction::East) => Ok(Tile::SouthToEast),
            (Direction::South, Direction::West) => Ok(Tile::SouthToWest),
            (Direction::East, Direction::West) => Ok(Tile::Horizontal),
            o => Err(format!("Invalid directions to make tile '{:?}'", o)),
        }
    }
}

impl TryFrom<Tile> for Direction {
    type Error = String;
    fn try_from(value: Tile) -> Result<Self, Self::Error> {
        match value {
            Tile::Vertical | Tile::NorthToEast | Tile::NorthToWest => Ok(Direction::North),
            Tile::SouthToWest | Tile::SouthToEast => Ok(Direction::South),
            Tile::Horizontal => Ok(Direction::East),
            o => Err(format!("Can not convert {o:?} into default direction")),
        }
    }
}

impl FromStr for Tile {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(Tile::Vertical),
            "-" => Ok(Tile::Horizontal),
            "L" => Ok(Tile::NorthToEast),
            "J" => Ok(Tile::NorthToWest),
            "7" => Ok(Tile::SouthToWest),
            "F" => Ok(Tile::SouthToEast),
            "." => Ok(Tile::Ground),
            "S" => Ok(Tile::Start),
            o => Err(format!("Unknown tile '{o}'")),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Tile::Vertical => "|",
            Tile::Horizontal => "-",
            Tile::NorthToEast => "L",
            Tile::NorthToWest => "J",
            Tile::SouthToWest => "7",
            Tile::SouthToEast => "F",
            Tile::Ground => ".",
            Tile::Start => "S",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Classification {
    Loop,
    Inside,
    Outside,
}

// each 'up', 'down' etc. represent whether this tween connects to the adjacent tween in that direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Tween {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

struct TweenMap(Vec<Vec<(Tween, bool)>>);

impl TweenMap {
    fn visited(&self, row: usize, col: usize) -> bool {
        self.0[row][col].1
    }
}

impl Grid for TweenMap {
    type Cell = (Tween, bool);
    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn adjacent_cells(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut out = Vec::new();
        if row != 0 && self.0[row][col].0.up {
            out.push((row - 1, col));
        }
        if row < self.height() && self.0[row][col].0.down {
            out.push((row + 1, col));
        }
        if col != 0 && self.0[row][col].0.left {
            out.push((row, col - 1));
        }
        if col < self.width() && self.0[row][col].0.right {
            out.push((row, col + 1));
        }

        out
    }
}

impl Display for TweenMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for row in &self.0 {
            for (_, visited) in row {
                match visited {
                    true => s += "+",
                    false => s += ".",
                }
            }
            s += "\n"
        }
        f.write_str(&s)
    }
}

trait Grid {
    type Cell;

    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn all_edge_cells(&self) -> Vec<(usize, usize)> {
        // (0, 0) .... (0, len-1)
        // (1, 0) .... (len-1, 0)
        // (1, len-1) .. (len-1, len-1)
        // (2, len-1) .. (len-2, len-1)
        (0..self.width())
            .map(|c| (0, c))
            .chain((1..self.height()).map(|r| (r, 0)))
            .chain((1..self.height()).map(|r| (r, self.width() - 1)))
            .chain((2..self.width() - 1).map(|c| (self.height() - 1, c)))
            .collect()
    }

    fn adjacent_cells(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut out = Vec::new();

        if row != 0 {
            out.push((row - 1, col));
        }
        if row < self.height() {
            out.push((row + 1, col));
        }
        if col != 0 {
            out.push((row, col - 1))
        }
        if col < self.width() {
            out.push((row, col + 1))
        }
        if row != 0 && col != 0 {
            out.push((row - 1, col - 1))
        }
        if row != 0 && col < self.width() {
            out.push((row - 1, col + 1))
        }
        if row < self.height() && col != 0 {
            out.push((row + 1, col - 1))
        }
        if row < self.height() && col < self.width() {
            out.push((row + 1, col + 1))
        }

        out
    }
}

#[derive(Debug, Clone)]
struct Map(Vec<Vec<(Tile, Option<Classification>)>>);

impl Map {
    fn find_start(&self) -> Option<(usize, usize)> {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(i, r)| {
                let c = r
                    .iter()
                    .enumerate()
                    .find(|(_, (t, _))| *t == Tile::Start)
                    .unzip()
                    .0;
                Some(i).zip(c)
            })
            .next()
    }

    fn translate_start(&self, row: usize, col: usize) -> Tile {
        let up = if row == 0 {
            None
        } else {
            Some(self.0[row - 1][col])
        };
        let down = self.0.get(row + 1).map(|r| r.get(col)).flatten();
        let left = if col == 0 {
            None
        } else {
            Some(self.0[row][col - 1])
        };
        let right = self.0.get(row).map(|r| r.get(col + 1)).flatten();

        let mut dirs = Vec::with_capacity(2);
        if up.is_some_and(|(t, _)| t.connects(Direction::South)) {
            dirs.push(Direction::North)
        }
        if down.is_some_and(|(t, _)| t.connects(Direction::North)) {
            dirs.push(Direction::South)
        }
        if left.is_some_and(|(t, _)| t.connects(Direction::East)) {
            dirs.push(Direction::West)
        }
        if right.is_some_and(|(t, _)| t.connects(Direction::West)) {
            dirs.push(Direction::East)
        }

        dirs.try_into().unwrap()
    }

    /// Go across the loop setting the tiles on the loop that they're on the loop
    /// Returns the length of the loop
    fn traverse_loop(&mut self) -> usize {
        let (start_row, start_col) = self.find_start().unwrap();
        let mut current = self.translate_start(start_row, start_col);
        self.set_tile_class(start_row, start_col, Classification::Loop);
        let mut direction: Direction = current.try_into().unwrap();

        let (mut row, mut col) = (start_row, start_col);
        let mut steps = 0;

        loop {
            (row, col, direction) = current.next(direction, row, col);
            self.set_tile_class(row, col, Classification::Loop);

            current = self.0[row][col].0;
            steps += 1;
            if row == start_row && col == start_col {
                return steps;
            }
        }
    }

    /// row and column are tween addresses
    fn tween_at(&self, row: usize, col: usize) -> Tween {
        let up = if row > 0 && col > 0 && col < self.width() {
            let (left, left_class) = self.0[row - 1][col - 1];
            let (right, right_class) = self.0[row - 1][col];

            if left_class.is_some_and(|c| c == Classification::Loop)
                && right_class.is_some_and(|c| c == Classification::Loop)
            {
                left.can_sneak(&right, Direction::North)
            } else {
                true
            }
        } else {
            false
        };
        let down = if row < self.height() && col > 0 && col < self.width() {
            let (left, left_class) = self.0[row][col - 1];
            let (right, right_class) = self.0[row][col];

            if left_class.is_some_and(|c| c == Classification::Loop)
                && right_class.is_some_and(|c| c == Classification::Loop)
            {
                left.can_sneak(&right, Direction::South)
            } else {
                true
            }
        } else {
            false
        };
        let left = if col > 0 && row > 0 && row < self.height() {
            let (up, up_class) = self.0[row - 1][col - 1];
            let (down, down_class) = self.0[row][col - 1];

            if up_class.is_some_and(|c| c == Classification::Loop)
                && down_class.is_some_and(|c| c == Classification::Loop)
            {
                up.can_sneak(&down, Direction::West)
            } else {
                true
            }
        } else {
            false
        };
        let right = if col < self.width() && row > 0 && row < self.height() {
            let (up, up_class) = self.0[row - 1][col];
            let (down, down_class) = self.0[row][col];

            if up_class.is_some_and(|c| c == Classification::Loop)
                && down_class.is_some_and(|c| c == Classification::Loop)
            {
                up.can_sneak(&down, Direction::East)
            } else {
                true
            }
        } else {
            false
        };
        Tween {
            up,
            down,
            left,
            right,
        }
    }

    fn make_tweens(&self) -> TweenMap {
        let mut map = vec![Vec::with_capacity(self.0[0].len() + 1); self.0.len() + 1];
        // Tweens go around the outside edge of the grid, so len + 1
        for row in 0..self.0.len() + 1 {
            for col in 0..self.0[0].len() + 1 {
                map[row].push((self.tween_at(row, col), false))
            }
        }
        TweenMap(map)
    }

    fn display_interspersed(&self, tweens: &TweenMap) {
        let mut s = String::new();
        for i in 0..tweens.0.len() {
            // display tweens
            for (_, visited) in &tweens.0[i] {
                match visited {
                    true => s += "+ ",
                    false => s += ". ",
                }
            }
            s += "\n ";
            if i >= self.0.len() {
                s += "\n";
                continue;
            }
            for (tile, class) in &self.0[i] {
                match class {
                    Some(Classification::Outside) => s += "O ",
                    Some(Classification::Inside) => s += "I ",
                    _ => s += &format!("{tile} "),
                }
            }
            s += "\n";
        }
        println!("{s}")
    }

    pub fn partition(&mut self) {
        let loop_size = self.traverse_loop();
        println!("Loop size: {loop_size}");
        println!("farthest point: {}", loop_size / 2);

        let mut tweens = self.make_tweens();

        let mut stack = tweens.all_edge_cells();

        while let Some((row, col)) = stack.pop() {
            if tweens.0[row][col].1 {
                continue;
            }
            tweens.0[row][col].1 = true;

            stack.append(&mut tweens.adjacent_cells(row, col));
        }

        // any cell surrounded by 4 visited tweens is Outside
        for row in 0..self.0.len() {
            for col in 0..self.0[row].len() {
                if tweens.visited(row, col)
                    && tweens.visited(row, col + 1)
                    && tweens.visited(row + 1, col)
                    && tweens.visited(row + 1, col + 1)
                {
                    self.0[row][col].1 = Some(Classification::Outside)
                }
            }
        }

        self.display_interspersed(&tweens);
    }

    pub fn make_all_unclassified_inside(&mut self) {
        self.0
            .iter_mut()
            .map(|r| r.iter_mut())
            .flatten()
            .map(|(_t, c)| c.get_or_insert(Classification::Inside))
            .count();
    }

    pub fn count_inside(&self) -> usize {
        self.0
            .iter()
            .map(|r| r.iter())
            .flatten()
            .filter(|(_t, c)| c.is_some_and(|c| c == Classification::Inside))
            .count()
    }

    #[inline(always)]
    fn set_tile_class(&mut self, row: usize, col: usize, class: Classification) {
        self.0[row][col].1.get_or_insert(class);
    }
}

impl Grid for Map {
    type Cell = (Tile, Option<Classification>);
    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }
}

impl FromStr for Map {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| {
                        let t = c.to_string().parse()?;
                        Ok((t, None))
                    })
                    .collect::<Result<Vec<_>, String>>()
            })
            .collect::<Result<Vec<_>, String>>()?;

        Ok(Map(map))
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for row in &self.0 {
            for (tile, class) in row {
                match *class {
                    Some(Classification::Outside) => s += "O",
                    Some(Classification::Inside) => s += "I",
                    _ => s += &format!("{tile}"),
                }
            }
            s += "\n"
        }
        f.write_str(&s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_known_input() {
        let input = r##".....
.S-7.
.|.|.
.L-J.
.....
"##;

        assert_eq!(run_part_1(input), 4);
    }

    #[test]
    fn part_1_known_input_2() {
        let input = r##"..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"##;

        assert_eq!(run_part_1(input), 8);
    }

    #[test]
    fn part_2_known_input() {
        let input = r##"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
"##;

        assert_eq!(run_part_2(input), 4);
    }

    #[test]
    fn part_2_known_input_2() {
        let input = r##"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
"##;

        assert_eq!(run_part_2(input), 4);
    }

    #[test]
    fn part_2_known_input_3() {
        let input = r##".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"##;

        assert_eq!(run_part_2(input), 8);
    }

    #[test]
    fn part_2_known_input_4() {
        let input = r##"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"##;

        assert_eq!(run_part_2(input), 10);
    }
}
