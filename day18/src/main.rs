use std::str::FromStr;

use anyhow::{anyhow, Context, Error, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input)?);
    println!("part2: {}", part2(input)?);
    Ok(())
}

struct Map {
    perimeter: Perimeter,
}

impl Map {
    fn volume(&self) -> Option<f64> {
        Some(
            self.perimeter
                .iter()
                .tuple_windows()
                .map(|(a, b)| {
                    (a.1 + b.1) * (a.0 - b.0) + isize::abs(a.0 - b.0) + isize::abs(a.1 - b.1)
                })
                .sum::<isize>() as f64
                / 2.0
                + 1.0,
        )
    }

    fn parse_from_string(s: &str, parser: fn(&str) -> Result<Op>) -> Result<Self> {
        let ops: Vec<Op> = s.lines().map(parser).collect::<Result<_>>()?;
        let mut perimeter = Perimeter::new();
        perimeter.push(Coord(0, 0));
        let mut head = Coord(0, 0);

        for op in ops {
            op.apply(&mut perimeter, &mut head);
        }

        Ok(Self { perimeter })
    }

    fn parse_from_parts(s: &str) -> Result<Self> {
        Self::parse_from_string(s, Op::from_str)
    }

    fn parse_from_hex(s: &str) -> Result<Self> {
        Self::parse_from_string(s, Op::parse_from_hex)
    }
}

enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl FromStr for Dir {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "U" => Dir::Up,
            "D" => Dir::Down,
            "R" => Dir::Right,
            "L" => Dir::Left,
            _ => return Err(anyhow!("unknown direction: {}", s)),
        })
    }
}

struct Op {
    direction: Dir,
    count: isize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, PartialOrd, Ord, Default)]
struct Coord(isize, isize);

type Perimeter = Vec<Coord>;

impl Op {
    fn apply(&self, map: &mut Perimeter, head: &mut Coord) {
        *head = self.direction.apply(head, self.count);
        map.push(*head);
    }

    fn parse_from_hex(s: &str) -> Result<Self> {
        let (_, hex) = s.rsplit_once(" ").context("couldn't get op")?;
        let hex = &hex[2..8];
        let (meters, dir) = hex.split_at(5);

        dbg!(meters, dir);
        let direction = match dir {
            "0" => Dir::Right,
            "1" => Dir::Down,
            "2" => Dir::Left,
            "3" => Dir::Up,
            _ => return Err(anyhow!("unknown direction in hex: {}", dir)),
        };

        Ok(Self {
            count: isize::from_str_radix(meters, 16)?,
            direction,
        })
    }
}

impl Dir {
    fn apply(&self, Coord(x, y): &Coord, count: isize) -> Coord {
        match self {
            Dir::Up => Coord(*x, y - count),
            Dir::Right => Coord(x + count, *y),
            Dir::Down => Coord(*x, y + count),
            Dir::Left => Coord(x - count, *y),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct Color(String);

impl FromStr for Color {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Color(s.into()))
    }
}

impl FromStr for Op {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let direction = parts.next().context("can't get dir")?.parse::<Dir>()?;
        let count = parts.next().context("can't get count")?.parse::<isize>()?;

        Ok(Op { direction, count })
    }
}

fn part1(input: &str) -> Result<f64> {
    let map = Map::parse_from_parts(input)?;
    map.volume().context("couldn't get volume")
}

fn part2(input: &str) -> Result<f64> {
    let map = Map::parse_from_hex(input)?;
    map.volume().context("couldn't get volume")
}

#[test]
fn part1_test() {
    let input = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;
    assert_eq!(part1(input).unwrap(), 62.0);
}
