use core::{
    ops::{Add, Sub},
    str::{FromStr, Split},
};
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../assets/day5Input.txt");
    let part_1_out = run_part_1(input);
    println!("part_1: {part_1_out}");
}

fn run_part_1(input: &str) -> usize {
    let almanac: Almanac = input.parse().unwrap();

    almanac.smallest_location().into()
}

fn run_part_2(input: &str) -> usize {
    let almanac: Almanac = input.parse().unwrap();

    almanac.smallest_range_location().into()
}

struct SeedRange {
    start: Seed,
    length: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Seed(usize);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Soil(usize);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Fertilizer(usize);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Water(usize);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Light(usize);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Temperature(usize);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Humidity(usize);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Location(usize);

impl From<usize> for Seed {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl From<Seed> for usize {
    fn from(value: Seed) -> Self {
        value.0
    }
}
impl From<usize> for Soil {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl From<Soil> for usize {
    fn from(value: Soil) -> Self {
        value.0
    }
}
impl From<usize> for Fertilizer {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl From<Fertilizer> for usize {
    fn from(value: Fertilizer) -> Self {
        value.0
    }
}
impl From<usize> for Water {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl From<Water> for usize {
    fn from(value: Water) -> Self {
        value.0
    }
}
impl From<usize> for Light {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl From<Light> for usize {
    fn from(value: Light) -> Self {
        value.0
    }
}
impl From<usize> for Temperature {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl From<Temperature> for usize {
    fn from(value: Temperature) -> Self {
        value.0
    }
}
impl From<usize> for Humidity {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl From<Humidity> for usize {
    fn from(value: Humidity) -> Self {
        value.0
    }
}
impl From<usize> for Location {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl From<Location> for usize {
    fn from(value: Location) -> Self {
        value.0
    }
}

struct Map<S, D, L> {
    dest_start: D,
    source_start: S,
    length: L,
}

impl<S, D, L> FromStr for Map<S, D, L>
where
    S: From<L>,
    D: From<L>,
    L: FromStr,
    <L as FromStr>::Err: std::fmt::Display,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = s.split_ascii_whitespace().collect();
        println!("{s}");
        if split.len() != 3 {
            return Err("Expected map line to have exactly 3 values".to_string());
        }
        let dest_start: D = split[0].parse::<L>().map_err(|e| format!("{e}"))?.into();
        let source_start: S = split[1].parse::<L>().map_err(|e| format!("{e}"))?.into();
        let length: L = split[2].parse().map_err(|e| format!("{e}"))?;

        Ok(Self {
            dest_start,
            source_start,
            length,
        })
    }
}

struct Maps<S, D, L>(Vec<Map<S, D, L>>, String);

impl<S, D, L> Maps<S, D, L>
where
    S: From<L> + Into<L> + Copy,
    D: From<L> + Into<L> + Copy,
    L: Add<L, Output = L> + Sub<L, Output = L> + PartialOrd + Copy,
{
    fn convert(&self, source: S) -> D {
        for map in &self.0 {
            if source.into() > map.source_start.into()
                && source.into() < map.source_start.into() + map.length
            {
                let i: L = source.into() - map.source_start.into();
                let dest: L = map.dest_start.into() + i;
                return dest.into();
            }
        }

        Into::<L>::into(source).into()
    }

    fn invert(&self, dest: D) -> S {
        for map in &self.0 {
            if dest.into() > map.dest_start.into()
                && dest.into() < map.dest_start.into() + map.length
            {
                let i: L = dest.into() - map.dest_start.into();
                let source: L = map.source_start.into() + i;
                return source.into();
            }
        }

        Into::<L>::into(dest).into()
    }
}

impl<S, D, L> FromStr for Maps<S, D, L>
where
    S: From<L>,
    D: From<L>,
    L: FromStr,
    <L as FromStr>::Err: std::fmt::Display,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let header = lines
            .next()
            .ok_or_else(|| "expected at least 1 line".to_string())?;
        let maps: Vec<_> = lines
            .map(|l| l.parse::<Map<S, D, L>>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(maps, header.to_string()))
    }
}

struct Almanac {
    seeds: Vec<Seed>,
    seed_ranges: Vec<SeedRange>,
    seed_to_soil: Maps<Seed, Soil, usize>,
    soil_to_fertilizer: Maps<Soil, Fertilizer, usize>,
    fertilizer_to_water: Maps<Fertilizer, Water, usize>,
    water_to_light: Maps<Water, Light, usize>,
    light_to_temperature: Maps<Light, Temperature, usize>,
    temperature_to_humidity: Maps<Temperature, Humidity, usize>,
    humidity_to_location: Maps<Humidity, Location, usize>,
}

impl Almanac {
    fn smallest_location(&self) -> Location {
        self.seeds
            .iter()
            .map(|s| self.seed_to_soil.convert(*s))
            .map(|s| self.soil_to_fertilizer.convert(s))
            .map(|f| self.fertilizer_to_water.convert(f))
            .map(|w| self.water_to_light.convert(w))
            .map(|l| self.light_to_temperature.convert(l))
            .map(|t| self.temperature_to_humidity.convert(t))
            .map(|h| self.humidity_to_location.convert(h))
            .reduce(|smallest, l| smallest.min(l))
            .unwrap()
    }

    fn is_valid_seed(&self, seed: Seed) -> bool {
        self.seed_ranges
            .iter()
            .any(|range| seed >= range.start && seed.0 < range.start.0 + range.length)
    }

    fn smallest_range_location(&self) -> Location {
        let mut map_discontinuities = HashSet::new();
        for range in self.seed_ranges {
            map_discontinuities.insert(range.start)
        }
        // populate seed_discontinuities
        self.humidity_to_location
            .0
            .iter()
            .map(|h| self.temperature_to_humidity.invert(d))
            .

        // check all discontinuities
        seed_discontinuities
            .into_iter()
            .filter(|s| self.is_valid_seed(s))
            .map(|s| self.seed_to_soil.convert(s))
            .map(|s| self.soil_to_fertilizer.convert(s))
            .map(|f| self.fertilizer_to_water.convert(f))
            .map(|w| self.water_to_light.convert(w))
            .map(|l| self.light_to_temperature.convert(l))
            .map(|t| self.temperature_to_humidity.convert(t))
            .map(|h| self.humidity_to_location.convert(h))
            .reduce(|smallest, l| smallest.min(l))
            .unwrap()
    }
}

impl FromStr for Almanac {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stanzas = s.split("\n\n");
        println!("stanzas: {:?}", stanzas);
        let mut seeds_str = stanzas.next().unwrap().split_ascii_whitespace();
        if seeds_str.next() != Some("seeds:") {
            return Err("Expected first stanza to begin with `seeds: `".to_string());
        }
        let seeds: Vec<Seed> = seeds_str
            .map(|s| {
                println!("{s}");
                let n = usize::from_str_radix(s, 10).map_err(|e| format!("{e}"))?;
                Ok(Seed(n))
            })
            .collect::<Result<Vec<_>, String>>()?;

        let mut seed_ranges = Vec::with_capacity(seeds.len() / 2);
        for i in 0..(seed_ranges.len() - 1) / 2 {
            let start = seeds[i * 2];
            let length = seeds[i * 2 + 1].into();
            seed_ranges.push(SeedRange { start, length });
        }

        let seed_to_soil = parse_map_stanza("seed-to-soil map:", &mut stanzas)?;
        let soil_to_fertilizer = parse_map_stanza("soil-to-fertilizer map:", &mut stanzas)?;
        let fertilizer_to_water = parse_map_stanza("fertilizer-to-water map:", &mut stanzas)?;
        let water_to_light = parse_map_stanza("water-to-light map:", &mut stanzas)?;
        let light_to_temperature = parse_map_stanza("light-to-temperature map:", &mut stanzas)?;
        let temperature_to_humidity =
            parse_map_stanza("temperature-to-humidity map:", &mut stanzas)?;
        let humidity_to_location = parse_map_stanza("humidity-to-location map:", &mut stanzas)?;

        Ok(Self {
            seeds,
            seed_ranges,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        })
    }
}

fn parse_map_stanza<S, D, L>(
    header: &str,
    stanzas: &mut Split<'_, &str>,
) -> Result<Maps<S, D, L>, String>
where
    S: From<L>,
    D: From<L>,
    L: FromStr,
    <L as FromStr>::Err: std::fmt::Display,
{
    let maps: Maps<S, D, L> = stanzas
        .next()
        .ok_or_else(|| "Expected at least 1 more stanza".to_string())?
        .parse()?;
    if maps.1 != header {
        return Err(format!("Expected stanza to begin with '{header}'"));
    }
    Ok(maps)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_known_answer() {
        let input = r##"seeds: 79 14 55 13

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
56 93 4
"##;

        assert_eq!(run_part_1(input), 35)
    }

    #[test]
    fn part_2_known_answer() {
        let input = r##"seeds: 79 14 55 13

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
56 93 4
"##;

        assert_eq!(run_part_2(input), 46)
    }
}
