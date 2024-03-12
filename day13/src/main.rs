use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Context, Error, Result};

fn main() -> Result<()> {
    let input = include_str!("../input.txt");

    println!("part1 {}", part1(input)?);
    println!("part2 {}", part2(input)?);
    Ok(())
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Tile {
    Ash,
    Rock,
}

#[derive(Hash, PartialEq, Eq)]
struct Coord(usize, usize);

#[derive(Clone)]
struct Map {
    inner: Vec<Tile>,
    width: usize,
    height: usize,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut inner = Vec::new();
        let width = s
            .lines()
            .next()
            .context("couldn't get width")?
            .chars()
            .count();
        let height = s.lines().count();
        for line in s.lines() {
            for c in line.chars() {
                let tile = match c {
                    '.' => Tile::Ash,
                    '#' => Tile::Rock,
                    _ => return Err(anyhow!("couldn't match char")),
                };
                inner.push(tile);
            }
        }

        Ok(Self {
            inner,
            width,
            height,
        })
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Dir {
    Vertical(usize),
    Horizontal(usize),
}

impl Dir {
    fn score(&self) -> usize {
        match self {
            Dir::Vertical(count) => *count,
            Dir::Horizontal(count) => 100 * *count,
        }
    }
}

fn cmp_iterators<A: Eq + std::fmt::Debug>(
    left: impl Iterator<Item = A>,
    right: impl Iterator<Item = A>,
) -> bool {
    // let left = left.collect::<Vec<_>>();
    //
    // let right = right.collect::<Vec<_>>();
    //
    // let left = left.into_iter();
    // let right = right.into_iter();

    let mut zipper = left.zip(right);
    let mut is_equal = true;

    while let Some((left, right)) = zipper.next() {
        if left != right {
            is_equal = false;
            break;
        }
    }
    is_equal
}

#[test]
fn cmp_iterators_test() {
    let left = [1, 2, 3];
    let right = [1, 2, 3, 4];
    assert!(cmp_iterators(left.iter(), right.iter()));
    let left = [1, 2, 3, 4];
    let right = [1, 2, 3];
    assert!(cmp_iterators(left.iter(), right.iter()))
}

impl Map {
    fn find_reflection(&self, original: Option<Dir>) -> Option<Dir> {
        for x in 0..self.width - 1 {
            // can't be reflected if it falls off the edge
            let min = (x + 1).min(self.width - x - 1);

            let left = (0..self.height).flat_map(|y| {
                (0..=x)
                    .rev()
                    .take(min)
                    .map(move |x| &self.inner[x + y * self.width])
            });

            let right = (0..self.height).flat_map(|y| {
                (x + 1..self.width)
                    .take(min)
                    .map(move |x| &self.inner[x + y * self.width])
            });

            if cmp_iterators(left, right) {
                if let Some(Dir::Vertical(count)) = original {
                    if count - 1 == x {
                        continue;
                    }
                }
                return Some(Dir::Vertical(x + 1));
            }
        }

        for y in 0..self.height - 1 {
            // can't be reflected if it falls off the edge
            let min = (y + 1).min(self.height - y - 1);
            let top = (0..=y)
                .rev()
                .take(min)
                .flat_map(|y| (0..self.width).map(move |x| &self.inner[x + y * self.width]));

            let bottom = (y + 1..self.height)
                .take(min)
                .flat_map(|y| (0..self.width).map(move |x| &self.inner[x + y * self.width]));

            if cmp_iterators(top, bottom) {
                if let Some(Dir::Horizontal(count)) = original {
                    if count - 1 == y {
                        continue;
                    }
                }
                return Some(Dir::Horizontal(y + 1));
            }
        }

        None
    }

    fn smudge_find_reflection(&self, original: Dir) -> Option<Dir> {
        for (x, tile) in self.inner.iter().enumerate() {
            if let Tile::Rock = tile {
                let mut clone = self.clone();
                clone.inner[x] = Tile::Ash;

                if let Some(dir) = clone.find_reflection(Some(original.clone())) {
                    return Some(dir);
                }
            }
        }

        None
    }
}

#[test]
fn reflection_test() {
    let input = r#"##
.."#;
    let map: Map = input.parse().unwrap();
    assert_eq!(map.find_reflection(None).unwrap(), Dir::Vertical(1));

    let input = r#"#.
#."#;
    let map: Map = input.parse().unwrap();
    assert_eq!(map.find_reflection(None).unwrap(), Dir::Horizontal(1));

    let input = r#".##
...
..."#;
    let map: Map = input.parse().unwrap();
    assert_eq!(map.find_reflection(None).unwrap(), Dir::Vertical(2));

    let input = r#"...
.#.
.#."#;
    let map: Map = input.parse().unwrap();
    assert_eq!(map.find_reflection(None).unwrap(), Dir::Horizontal(2));

    let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."#;
    let map: Map = input.parse().unwrap();
    assert_eq!(map.find_reflection(None).unwrap(), Dir::Vertical(5))
}

#[test]
fn smudge_find_reflection_test() {
    let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."#;
    let map: Map = input.parse().unwrap();
    assert_eq!(
        map.smudge_find_reflection(Dir::Vertical(5)).unwrap(),
        Dir::Horizontal(3)
    )
}

fn part1(s: &str) -> Result<usize> {
    s.split("\n\n").try_fold(0, |acc, chunks| {
        chunks
            .parse::<Map>()
            .and_then(|map| Ok(acc + map.find_reflection(None).context("throw it up")?.score()))
    })
}

#[test]
fn part1_test() {
    let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;
    assert_eq!(part1(input).unwrap(), 405)
}

fn part2(s: &str) -> Result<usize> {
    s.split("\n\n").try_fold(0, |acc, chunks| {
        chunks.parse::<Map>().and_then(|map| {
            let original = map.find_reflection(None).context("couldn't get original")?;
            Ok(acc
                + map
                    .smudge_find_reflection(original)
                    .context("throw it up")?
                    .score())
        })
    })
}

#[test]
fn part2_test() {
    let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;
    assert_eq!(part2(input).unwrap(), 400)
}
