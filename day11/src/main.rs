use std::{collections::HashSet, str::FromStr};

fn main() -> Result<(), Error> {
    let input = include_str!("../input.txt");
    println!("part 1: {}", part1(input)?);
    println!("part 2: {}", part2::<1_000_000>(input)?);
    Ok(())
}

#[derive(Debug)]
struct Error;

#[derive(Hash, Ord, PartialEq, PartialOrd, Eq, Clone, Debug)]
struct Coord(usize, usize);

impl Coord {
    fn dist(&self, other: &Self) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Map<const N: usize = 2> {
    inner: HashSet<Coord>,
    height: usize,
    width: usize,
}

impl<const N: usize> FromStr for Map<N> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inner = HashSet::new();
        let width = s
            .lines()
            .next()
            .expect("couldn't get first line")
            .chars()
            .count();
        let height = s.lines().count();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    inner.insert(Coord(x, y));
                }
            }
        }

        Ok(Self {
            inner,
            width,
            height,
        })
    }
}

impl<const N: usize> Map<N> {
    const EXPANSION_COUNT: usize = N;

    fn expand_map(self) -> Map<N> {
        let mut x_hash = HashSet::new();
        let mut y_hash = HashSet::new();

        self.inner.iter().for_each(|Coord(x, y)| {
            x_hash.insert(*x);
            y_hash.insert(*y);
        });

        let mut sorted_x = self.inner.into_iter().collect::<Vec<_>>();
        sorted_x.sort();
        let mut iter = sorted_x.into_iter().peekable();

        let mut new_map = HashSet::new();
        let mut x_expansion_count = 0;

        for x in 0..self.width {
            if !x_hash.contains(&x) {
                while let Some(Coord(x_star, _)) = iter.peek() {
                    if x_star > &x {
                        break;
                    }
                    if let Some(Coord(x_star, y)) = iter.next() {
                        new_map.insert(Coord(x_star + x_expansion_count, y));
                    };
                }

                x_expansion_count += Self::EXPANSION_COUNT - 1;
            }
        }
        iter.for_each(|Coord(x, y)| {
            new_map.insert(Coord(x + x_expansion_count, y));
        });

        let mut sorted_y = new_map.into_iter().collect::<Vec<_>>();
        sorted_y.sort_by_key(|coord| coord.1);
        let mut iter = sorted_y.into_iter().peekable();

        let mut new_map = HashSet::new();
        let mut y_expansion_count = 0;

        for y in 0..self.height {
            if !y_hash.contains(&y) {
                while let Some(Coord(_, y_star)) = iter.peek() {
                    if y_star > &y {
                        break;
                    }
                    if let Some(Coord(x, y_star)) = iter.next() {
                        new_map.insert(Coord(x, y_star + y_expansion_count));
                    };
                }

                y_expansion_count += Self::EXPANSION_COUNT - 1;
            }
        }

        iter.for_each(|Coord(x, y)| {
            new_map.insert(Coord(x, y + y_expansion_count));
        });

        Self {
            inner: new_map,
            height: self.height + y_expansion_count,
            width: self.width + x_expansion_count,
        }
    }

    fn min_dist_between_points(&self) -> usize {
        let points: Vec<_> = self.inner.iter().collect();

        points
            .iter()
            .enumerate()
            .flat_map(|(i, origin)| points.iter().skip(i).map(|comp| origin.dist(comp)))
            .sum()
    }
}

fn part1(input: &str) -> Result<usize, Error> {
    let map: Map = input.parse()?;
    let new_map = map.expand_map();
    let result = new_map.min_dist_between_points();

    Ok(result)
}

fn part2<const EXPANSION_COUNT: usize>(input: &str) -> Result<usize, Error> {
    let map: Map<EXPANSION_COUNT> = input.parse()?;
    let new_map = map.expand_map();
    let result = new_map.min_dist_between_points();

    Ok(result)
}

#[test]
fn expansion_test() {
    let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#...."#;
    let expanded: Map = input.parse::<Map>().unwrap().expand_map();
    let result = r#"....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#......."#;
    let expected: Map = result.parse().unwrap();
    assert_eq!(expanded, expected);
}

#[test]
fn part1_test() {
    let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#...."#;
    assert_eq!(part1(input).unwrap(), 374)
}

#[test]
fn part2_test() {
    let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#...."#;
    assert_eq!(part2::<10>(input).unwrap(), 1030);
    assert_eq!(part2::<100>(input).unwrap(), 8410)
}
