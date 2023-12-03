use anyhow::{Context, Error, Result};
use std::{collections::BTreeMap, ops::Range, rc::Rc, str::FromStr};

#[derive(Copy, Clone)]
struct Coord(usize, usize);

impl Coord {
    const SURROUNDING: [(isize, isize); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    fn surrounding(self) -> impl Iterator<Item = (usize, usize)> {
        Self::SURROUNDING.into_iter().map(move |(x_off, y_off)| {
            (
                self.0.saturating_add_signed(x_off),
                self.1.saturating_add_signed(y_off),
            )
        })
    }
}

#[derive(Debug)]
enum Tile {
    Blank,
    Num(usize),
    Symbol(char),
}

#[derive(Debug, Eq, PartialEq)]
struct PartNumber {
    number: usize,
    y: usize,
    x_start: usize,
    x_end: usize,
}

impl PartNumber {
    fn new(x: usize, y: usize, number: usize) -> Self {
        Self {
            number,
            y,
            x_start: x,
            x_end: x,
        }
    }

    fn add_onto(&mut self, x: usize, num: usize) {
        self.number = self.number * 10 + num as usize;
        self.x_end = x;
    }

    fn symbol_close(&self, map: &Map) -> bool {
        fn find_in_map(map: &Map, x: usize, y: usize) -> bool {
            Coord(x, y)
                .surrounding()
                .any(|coord| map.contains_key(&coord))
        }

        (self.x_start..=self.x_end).any(|x| find_in_map(&map, x, self.y))
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '0'..='9' => Tile::Num(c.to_digit(10).unwrap() as usize),
            '.' => Tile::Blank,
            _ => Tile::Symbol(c),
        }
    }
}

type Map = BTreeMap<(usize, usize), Tile>;

fn part1(input: &str) -> usize {
    let mut numbers = Vec::new();
    let mut symbol_map = Map::new();

    for (y, line) in input.lines().enumerate() {
        let mut current_number: Option<PartNumber> = None;

        for (x, c) in line.chars().enumerate() {
            let tile: Tile = Tile::from(c);

            match tile {
                Tile::Symbol(_) => {
                    if let Some(number) = current_number.take() {
                        numbers.push(number);
                    }
                    symbol_map.insert((x, y), tile);
                }
                Tile::Num(num) => {
                    if let Some(number) = &mut current_number {
                        number.add_onto(x, num);
                    } else {
                        current_number = Some(PartNumber::new(x, y, num as usize));
                    }
                }
                Tile::Blank => {
                    if let Some(number) = current_number.take() {
                        numbers.push(number);
                    }
                }
            };
        }

        if let Some(num) = current_number {
            numbers.push(num);
        }
    }

    numbers
        .iter()
        .filter(|n| n.symbol_close(&symbol_map))
        .map(|n| n.number)
        .sum::<usize>()
}

fn part2(input: &str) -> usize {
    let mut numbers = Vec::new();
    let mut stars = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut current_number: Option<PartNumber> = None;

        for (x, c) in line.chars().enumerate() {
            let tile: Tile = Tile::from(c);

            match tile {
                Tile::Symbol(c) => {
                    if let Some(number) = current_number.take() {
                        numbers.push(number);
                    }
                    if c == '*' {
                        stars.push((x, y));
                    }
                }
                Tile::Num(num) => {
                    if let Some(number) = &mut current_number {
                        number.add_onto(x, num);
                    } else {
                        current_number = Some(PartNumber::new(x, y, num as usize));
                    }
                }
                Tile::Blank => {
                    if let Some(number) = current_number.take() {
                        numbers.push(number);
                    }
                }
            };
        }

        if let Some(num) = current_number {
            numbers.push(num);
        }
    }

    let mut numbers_map: BTreeMap<(usize, usize), &PartNumber> = BTreeMap::new();

    for number in &numbers {
        for x in number.x_start..=number.x_end {
            numbers_map.insert((x, number.y), &number);
        }
    }

    stars
        .into_iter()
        .filter_map(|(x, y)| {
            let mut numbers = Coord(x, y)
                .surrounding()
                .filter_map(|coord| numbers_map.get(&coord))
                .collect::<Vec<_>>();
            numbers.dedup();

            (numbers.len() == 2).then(|| numbers.into_iter().map(|n| n.number).product::<usize>())
        })
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

#[test]
fn example_works() {
    let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
    assert_eq!(part1(input), 4361);
}

#[test]
fn negative_works() {
    let input = r#"-467"#;
    assert_eq!(part1(input), 467);
}
