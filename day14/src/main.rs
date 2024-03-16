#![feature(btree_cursors)]

use std::{
    collections::{BTreeMap, HashMap},
    ops::Bound,
    str::FromStr,
};

use anyhow::{Context, Error, Result};

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input)?);
    println!("part2: {}", part2(input)?);
    Ok(())
}

#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Debug, Copy, Clone)]
struct Coord(usize, usize);

enum Tile {
    Empty,
    Block,
    Rock,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'O' => Tile::Rock,
            '.' => Tile::Empty,
            '#' => Tile::Block,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Map {
    blocks: MapIndex,
    rocks: MapIndex,
    width: usize,
    height: usize,
}

#[derive(Default, Clone, PartialEq, Eq, Hash, Debug)]
struct MapIndex {
    x_index: BTreeMap<Coord, bool>,
    y_index: BTreeMap<Coord, bool>,
}

impl MapIndex {
    fn insert(&mut self, Coord(x, y): Coord) -> bool {
        self.x_index.insert(Coord(x, y), true);
        self.y_index.insert(Coord(y, x), true).is_some()
    }

    fn clear(&mut self) {
        self.x_index.clear();
        self.y_index.clear();
    }
}

impl<'a> IntoIterator for &'a MapIndex {
    type Item = &'a Coord;
    type IntoIter = std::collections::btree_map::Keys<'a, Coord, bool>;

    fn into_iter(self) -> Self::IntoIter {
        self.x_index.keys()
    }
}

impl Map {
    fn north(&mut self) {
        let mut rock_count = BTreeMap::new();

        // go through each rock
        // find the next block
        // add the rock to the count above the block
        for rock in &self.rocks {
            let cursor = &self.blocks.x_index.upper_bound(Bound::Excluded(rock));
            if let Some(Coord(x, y)) = cursor.key() {
                if x == &rock.0 {
                    *rock_count.entry(Coord(*x, *y + 1)).or_insert(0) += 1;
                    continue;
                }
            }
            *rock_count.entry(Coord(rock.0, 0)).or_insert(0) += 1;
        }

        self.rocks.clear();
        // go through the counts and spread them across y
        for (Coord(x, y), count) in rock_count {
            for i in 0..count {
                self.rocks.insert(Coord(x, y + i));
            }
        }
    }

    fn south(&mut self) {
        let mut rock_count = BTreeMap::new();

        for rock in &self.rocks {
            let cursor = &self.blocks.x_index.lower_bound(Bound::Excluded(rock));
            if let Some(Coord(x, y)) = cursor.key() {
                if x == &rock.0 {
                    *rock_count.entry(Coord(*x, *y - 1)).or_insert(0) += 1;
                    continue;
                }
            }
            *rock_count
                .entry(Coord(rock.0, self.height - 1))
                .or_insert(0) += 1;
        }

        self.rocks.clear();
        // go through the counts and spread them across y
        for (Coord(x, y), count) in rock_count {
            for i in 0..count {
                self.rocks.insert(Coord(x, y - i));
            }
        }
    }

    fn east(&mut self) {
        let mut rock_count = BTreeMap::new();

        // go through each rock
        // find the next block
        // add the rock to the count above the block
        for rock in &self.rocks {
            let cursor = &self
                .blocks
                .y_index
                .lower_bound(Bound::Excluded(&Coord(rock.1, rock.0)));
            if let Some(Coord(y, x)) = cursor.key() {
                if y == &rock.1 {
                    *rock_count.entry(Coord(*x - 1, *y)).or_insert(0) += 1;
                    continue;
                }
            }
            *rock_count.entry(Coord(self.width - 1, rock.1)).or_insert(0) += 1;
        }

        self.rocks.clear();
        // go through the counts and spread them across y
        for (Coord(x, y), count) in rock_count {
            for i in 0..count {
                self.rocks.insert(Coord(x - i, y));
            }
        }
    }

    fn west(&mut self) {
        let mut rock_count = BTreeMap::new();

        // go through each rock
        // find the next block
        // add the rock to the count above the block
        for rock in &self.rocks {
            let cursor = &self
                .blocks
                .y_index
                .upper_bound(Bound::Excluded(&Coord(rock.1, rock.0)));
            if let Some(Coord(y, x)) = cursor.key() {
                if y == &rock.1 {
                    *rock_count.entry(Coord(*x + 1, *y)).or_insert(0) += 1;
                    continue;
                }
            }
            *rock_count.entry(Coord(0, rock.1)).or_insert(0) += 1;
        }

        self.rocks.clear();
        // go through the counts and spread them across y
        for (Coord(x, y), count) in rock_count {
            for i in 0..count {
                self.rocks.insert(Coord(x + i, y));
            }
        }
    }

    fn cycle(&mut self) {
        self.north();
        self.west();
        self.south();
        self.east();
    }

    fn load(&self) -> usize {
        self.rocks
            .x_index
            .iter()
            .map(|(Coord(_, y), _)| self.height - y)
            .sum()
    }
}

#[test]
fn cycle_test() {
    let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;
    let mut map = input.parse::<Map>().unwrap();
    map.cycle();

    let expected = r#".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."#;
    let expected: Map = expected.parse().unwrap();
    assert_eq!(map.rocks.x_index, expected.rocks.x_index);
}

#[test]
fn south_test() {
    let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;
    let mut map = input.parse::<Map>().unwrap();
    map.south();

    let expected = r#".....#....
....#....#
...O.##...
...#......
O.O....O#O
O.#..O.#.#
O....#....
OO....OO..
#OO..###..
#OO.O#...O"#;
    let expected: Map = expected.parse().unwrap();
    assert_eq!(map.rocks.x_index, expected.rocks.x_index);
}

#[test]
fn west_test() {
    let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;
    let mut map = input.parse::<Map>().unwrap();
    map.west();

    let expected = r#"O....#....
OOO.#....#
.....##...
OO.#OO....
OO......#.
O.#O...#.#
O....#OO..
O.........
#....###..
#OO..#...."#;
    let expected: Map = expected.parse().unwrap();
    assert_eq!(map.rocks.x_index, expected.rocks.x_index);
}

#[test]
fn east_test() {
    let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;
    let mut map = input.parse::<Map>().unwrap();
    map.east();

    let expected = r#"....O#....
.OOO#....#
.....##...
.OO#....OO
......OO#.
.O#...O#.#
....O#..OO
.........O
#....###..
#..OO#...."#;
    let expected: Map = expected.parse().unwrap();
    assert_eq!(map.rocks.x_index, expected.rocks.x_index);
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut blocks = MapIndex::default();
        let mut rocks = MapIndex::default();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match Tile::from(c) {
                    Tile::Empty => {}
                    Tile::Block => {
                        blocks.insert(Coord(x, y));
                    }
                    Tile::Rock => {
                        rocks.insert(Coord(x, y));
                    }
                }
            }
        }

        let width = s
            .lines()
            .next()
            .context("couldn't get width")?
            .chars()
            .count();
        let height = s.lines().count();

        Ok(Self {
            blocks,
            rocks,
            width,
            height,
        })
    }
}

fn part1(s: &str) -> Result<usize> {
    let mut map: Map = s.parse()?;
    map.north();
    Ok(map.load())
}

#[test]
fn part1_test() {
    let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;
    assert_eq!(part1(input).unwrap(), 136);
}

fn part2(s: &str) -> Result<usize> {
    let mut map: Map = s.parse()?;
    let mut cache: HashMap<Map, Map> = HashMap::new();

    let mut vec = Vec::new();
    let mut loop_start = 0;

    loop {
        if let Some(next) = cache.get(&map) {
            map = next.clone();
        } else {
            let current_map = map.clone();
            map.cycle();
            let next_map = map.clone();

            cache.insert(current_map, next_map);
        };

        if cache.len() == loop_start {
            let load = map.load();
            if let Some((first_map, first_load)) = vec.first() {
                if first_load == &load && first_map == &map {
                    break;
                }
            }
            vec.push((map.clone(), load));
        } else {
            loop_start = cache.len()
        }
    }

    let result = &vec[(1_000_000_000 - loop_start) % vec.len() - 1];
    Ok(result.1)
}

#[test]
fn part2_test() {
    let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;
    assert_eq!(part2(input).unwrap(), 64);
}
