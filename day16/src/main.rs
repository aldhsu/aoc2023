use std::{collections::HashMap, fmt::Display, str::FromStr};

use anyhow::{anyhow, Error, Result};

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input)?);
    println!("part2: {}", part2(input)?);

    Ok(())
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Coord(isize, isize);

#[derive(Debug, Eq, PartialEq, Hash)]
enum Tile {
    HorizontalSplitter,
    VerticalSplitter,
    BackSlash,
    ForwardSlash,
    Empty,
}

impl Tile {
    fn next(&self, head: &Head) -> HeadRes {
        match (self, head.dir) {
            (Tile::HorizontalSplitter, Dir::North) | (Tile::HorizontalSplitter, Dir::South) => {
                HeadRes::Two(head.clonedir(Dir::East), head.clonedir(Dir::West))
            }
            (Tile::HorizontalSplitter, Dir::East) | (Tile::HorizontalSplitter, Dir::West) => {
                HeadRes::One(*head)
            }

            (Tile::VerticalSplitter, Dir::North) | (Tile::VerticalSplitter, Dir::South) => {
                HeadRes::One(*head)
            }
            (Tile::VerticalSplitter, Dir::East) | (Tile::VerticalSplitter, Dir::West) => {
                HeadRes::Two(head.clonedir(Dir::North), head.clonedir(Dir::South))
            }
            (Tile::BackSlash, Dir::North) => HeadRes::One(head.clonedir(Dir::West)),
            (Tile::BackSlash, Dir::South) => HeadRes::One(head.clonedir(Dir::East)),
            (Tile::BackSlash, Dir::East) => HeadRes::One(head.clonedir(Dir::South)),
            (Tile::BackSlash, Dir::West) => HeadRes::One(head.clonedir(Dir::North)),
            (Tile::ForwardSlash, Dir::North) => HeadRes::One(head.clonedir(Dir::East)),
            (Tile::ForwardSlash, Dir::South) => HeadRes::One(head.clonedir(Dir::West)),
            (Tile::ForwardSlash, Dir::East) => HeadRes::One(head.clonedir(Dir::North)),
            (Tile::ForwardSlash, Dir::West) => HeadRes::One(head.clonedir(Dir::South)),
            _ => unreachable!(),
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            '-' => Tile::HorizontalSplitter,
            '|' => Tile::VerticalSplitter,
            '\\' => Tile::BackSlash,
            '/' => Tile::ForwardSlash,
            '.' => Tile::Empty,
            _ => return Err(anyhow!("don't know tile")),
        })
    }
}

#[derive(Debug, Default)]
struct Map {
    inner: HashMap<Coord, Tile>,
    height: usize,
    width: usize,
}
impl Map {
    fn coord_possible(&self, coord: Coord) -> Option<()> {
        (coord.0 >= 0
            && coord.1 >= 0
            && coord.0 < self.width as isize
            && coord.1 < self.height as isize)
            .then_some(())
    }

    fn get(&self, coord: &Coord) -> Option<&Tile> {
        self.inner.get(coord)
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut inner = HashMap::new();
        let mut width = 0;
        let mut height = 0;

        for (y, line) in s.lines().enumerate() {
            height += 1;
            width = line.chars().count();

            for (x, c) in line.chars().enumerate() {
                if let Ok(tile) = Tile::try_from(c) {
                    if tile != Tile::Empty {
                        inner.insert(Coord(x as isize, y as isize), tile);
                    }
                }
            }
        }

        Ok(Self {
            inner,
            height,
            width,
        })
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
struct Head {
    dir: Dir,
    coord: Coord,
}

enum HeadRes {
    One(Head),
    Two(Head, Head),
}

type DirSet = [bool; 4];

impl Head {
    fn tick(mut self, active: &mut HashMap<Coord, DirSet>, map: &Map) -> Option<HeadRes> {
        //move
        let coord = self.next_move(map)?;
        self.coord = coord;

        let index = self.dir as usize;
        match active.entry(self.coord) {
            std::collections::hash_map::Entry::Occupied(mut val) => {
                if val.get()[index] {
                    return None;
                } // same direction has been here before
                val.get_mut()[index] = true;
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                let mut val = [false, false, false, false];
                val[index] = true;
                entry.insert(val);
            }
        }

        if let Some(tile) = map.get(&self.coord) {
            Some(tile.next(&self))
        } else {
            Some(HeadRes::One(self))
        }
    }

    fn next_move(&self, map: &Map) -> Option<Coord> {
        let coord = match self.dir {
            Dir::North => Coord(self.coord.0, self.coord.1 - 1),
            Dir::South => Coord(self.coord.0, self.coord.1 + 1),
            Dir::East => Coord(self.coord.0 + 1, self.coord.1),
            Dir::West => Coord(self.coord.0 - 1, self.coord.1),
        };

        map.coord_possible(coord)?;
        Some(coord)
    }

    fn clonedir(&self, dir: Dir) -> Self {
        Self { dir, ..*self }
    }
}

struct State {
    active: HashMap<Coord, DirSet>,
    heads: Vec<Head>,
}

impl State {
    fn tick(&mut self, map: &Map) -> Option<()> {
        let mut new_heads = Vec::new();

        for head in self.heads.drain(..) {
            if let Some(res) = head.tick(&mut self.active, map) {
                match res {
                    HeadRes::One(a) => new_heads.push(a),
                    HeadRes::Two(a, b) => new_heads.extend([a, b].into_iter()),
                }
            }
        }

        self.heads = new_heads;
        (self.heads.is_empty()).then_some(())
    }

    fn count(&self) -> usize {
        self.active.len()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            active: Default::default(),
            heads: vec![Head {
                dir: Dir::East,
                coord: Coord(-1, 0),
            }],
        }
    }
}

struct Sim<'a> {
    map: &'a Map,
    state: State,
}

impl<'a> Display for Sim<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = Vec::new();

        for y in 0..self.map.height {
            let mut string = String::new();
            for x in 0..self.map.width {
                let coord = Coord(x as isize, y as isize);

                if let Some(tile) = self.map.get(&Coord(1000000000000, 100000000000000)) {
                    let c = match tile {
                        Tile::HorizontalSplitter => '-',
                        Tile::VerticalSplitter => '|',
                        Tile::BackSlash => '\\',
                        Tile::ForwardSlash => '/',
                        Tile::Empty => unreachable!(),
                    };

                    string.push(c);
                } else if let Some(_set) = self.state.active.get(&coord) {
                    string.push('#');
                } else {
                    string.push('.');
                }
            }
            result.push(string);
        }

        write!(f, "{}", result.join("\n"))
    }
}

impl<'a> Sim<'a> {
    fn tick(&mut self) -> Option<()> {
        self.state.tick(self.map)
    }

    fn count(&self) -> usize {
        self.state.count()
    }
}

fn part1(s: &str) -> Result<usize> {
    let map = s.parse::<Map>()?;
    let mut sim = Sim {
        map: &map,
        state: Default::default(),
    };

    while sim.tick().is_some() {}
    Ok(sim.count())
}

#[test]
fn part1_test() {
    let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
    assert_eq!(part1(input).unwrap(), 46);
}

fn part2(s: &str) -> Result<usize> {
    let map = s.parse::<Map>()?;
    let width = map.width;
    let height = map.height;
    let mut most = 0;
    //left
    for y in 0..height {
        let mut sim = Sim {
            map: &map,
            state: State {
                heads: vec![Head {
                    dir: Dir::East,
                    coord: Coord(-1, y as isize),
                }],
                ..Default::default()
            },
        };

        while sim.tick().is_some() {}
        most = most.max(sim.count());
    }

    //right
    for y in 0..height {
        let mut sim = Sim {
            map: &map,
            state: State {
                heads: vec![Head {
                    dir: Dir::West,
                    coord: Coord((width) as isize, y as isize),
                }],
                ..Default::default()
            },
        };

        while sim.tick().is_some() {}
        most = most.max(sim.count());
    }

    //top
    for x in 0..width {
        let mut sim = Sim {
            map: &map,
            state: State {
                heads: vec![Head {
                    dir: Dir::South,
                    coord: Coord(x as isize, -1_isize),
                }],
                ..Default::default()
            },
        };

        while sim.tick().is_some() {}
        most = most.max(sim.count());
    }

    //bottom
    for x in 0..width {
        let mut sim = Sim {
            map: &map,
            state: State {
                heads: vec![Head {
                    dir: Dir::North,
                    coord: Coord(x as isize, height as isize),
                }],
                ..Default::default()
            },
        };

        while sim.tick().is_some() {}
        most = most.max(sim.count());
    }

    Ok(most)
}

#[test]
fn part2_test() {
    let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
    assert_eq!(part2(input).unwrap(), 51);
}
