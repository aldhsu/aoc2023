use std::{
    array,
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    str::FromStr,
};

use anyhow::anyhow;
use anyhow::{Context, Error, Result};

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input)?);
    println!("part2: {}", part2(input)?);
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, PartialOrd, Ord, Default)]
struct Coord(usize, usize);

type Neighbors<const MAX_STEP: usize> = [Option<(Coord, usize)>; MAX_STEP];

impl Coord {
    const OFFSETS: [(isize, isize, Dir); 4] = [
        (0, -1, Dir::North),
        (-1, 0, Dir::West),
        (1, 0, Dir::East),
        (0, 1, Dir::South),
    ];

    fn neighbors<const MAX_STEP: usize>(&self) -> [(Dir, Neighbors<MAX_STEP>); 4] {
        fn make_coord(
            coord: &Coord,
            off_x: isize,
            off_y: isize,
            step: usize,
        ) -> Option<(Coord, usize)> {
            Some((
                Coord(
                    coord.0.checked_add_signed(off_x)?,
                    coord.1.checked_add_signed(off_y)?,
                ),
                step,
            ))
        }

        array::from_fn(|i| {
            let (off_x, off_y, dir) = Self::OFFSETS[i];
            (
                dir,
                array::from_fn(|j| {
                    let j = j as isize + 1;
                    let off_x = off_x * j;
                    make_coord(self, off_x, off_y * j, j as usize)
                }),
            )
        })
    }
}

#[derive(Eq, Debug, PartialEq)]
struct Node {
    distance: usize,
    cost: i32,
}

#[derive(Eq, PartialEq)]
struct Map {
    inner: HashMap<Coord, Node>,
    width: usize,
    height: usize,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let width = s
            .lines()
            .next()
            .map(|line| line.chars().count())
            .context("couldn't get width")?;
        let height = s.lines().count();

        let inner = s
            .lines()
            .enumerate()
            .flat_map(move |(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    (
                        Coord(x, y),
                        Node {
                            distance: width - x + height - y,
                            cost: c.to_digit(10).expect("couldn't get digit") as i32,
                        },
                    )
                })
            })
            .collect();

        Ok(Self {
            inner,
            width,
            height,
        })
    }
}

impl Map {
    fn get(&self, coord: &Coord) -> Option<&Node> {
        self.inner.get(coord)
    }

    fn node_cost(&self, coord: &Coord) -> Option<i32> {
        self.get(coord).map(|node| node.cost)
    }

    fn solve<const MIN_STEP: u8, const MAX_STEP: usize>(&self) -> Result<i32> {
        let finish = Coord(self.width - 1, self.height - 1);
        let mut open_set: BinaryHeap<Reverse<State>> =
            BinaryHeap::from([Reverse(State::default())]);
        let mut visited: HashMap<(Coord, Dir), i32> = HashMap::new();

        while let Some(Reverse(state)) = open_set.pop() {
            if state.head == finish {
                return Ok(state.heat_loss);
            }

            for (dir, neighbors) in state.head.neighbors::<MAX_STEP>().into_iter() {
                if state.is_aligned(&Some(dir)) {
                    continue;
                }

                let mut total_cost = state.heat_loss;
                for neighbor in neighbors {
                    let Some((neighbor, step)) = neighbor else {
                        break;
                    };

                    let Some(neighbor_cost) = self.node_cost(&neighbor) else {
                        break;
                    }; // neighbor doesn't exist;
                    total_cost += neighbor_cost;

                    if (step as u8) < MIN_STEP {
                        continue
                    }

                    let visited_key = (neighbor, dir);

                    if &total_cost < visited.get(&visited_key).unwrap_or(&i32::MAX) {
                        visited.insert(visited_key, total_cost);
                        let next_state = State {
                            heat_loss: total_cost,
                            head: neighbor,
                            steps: step as u8,
                            dir: Some(dir),
                        };

                        open_set.push(Reverse(next_state))
                    }
                }
            }
        }

        Err(anyhow!("couldn't get a path"))
    }

    fn print_path(&self, path: &[Coord]) {
        let mut hash: HashMap<&usize, HashSet<usize>> = HashMap::new();
        path.iter().for_each(|Coord(x, y)| {
            hash.entry(y).or_default().insert(*x);
        });

        for y in 0..self.height {
            let mut string = String::new();
            for x in 0..self.width {
                let char = match hash.get(&y) {
                    Some(v) => match v.get(&x) {
                        Some(_) => '*',
                        None => '.',
                    },
                    None => '.',
                };
                string.push(char);
            }

            println!("{}", string);
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(Eq, PartialEq)]
enum Plane {
    Horizontal,
    Vertical,
}

impl Dir {
    fn plane(&self) -> Plane {
        match self {
            Dir::North => Plane::Vertical,
            Dir::South => Plane::Vertical,
            Dir::East => Plane::Horizontal,
            Dir::West => Plane::Horizontal,
        }
    }
    fn is_aligned(&self, other: &Option<Self>) -> bool {
        other.is_some_and(|other| self.plane() == other.plane())
    }
}

#[derive(Eq, PartialEq, Clone, Hash)]
struct State {
    heat_loss: i32,
    head: Coord,
    steps: u8,
    dir: Option<Dir>,
}

impl State {
    fn is_aligned(&self, other: &Option<Dir>) -> bool {
        match self.dir {
            Some(dir) => dir.is_aligned(other),
            None => false,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heat_loss.cmp(&other.heat_loss)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            head: Coord(0, 0),
            heat_loss: 0,
            steps: 0,
            dir: None,
        }
    }
}

fn part1(s: &str) -> Result<i32> {
    let map: Map = s.parse()?;

    map.solve::<0, 3>()
}

fn part2(s: &str) -> Result<i32> {
    let map: Map = s.parse()?;

    map.solve::<4, 10>()
}

#[test]
fn part1_test() {
    let input = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;
    assert_eq!(part1(input).unwrap(), 102);
}

#[test]
fn part2_test() {
    let input = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;
    assert_eq!(part2(input).unwrap(), 94);

    let input = r#"111111111111
999999999991
999999999991
999999999991
999999999991"#;
    assert_eq!(part2(input).unwrap(), 71);
}

// 2413432311323
// 3215453535623
// 3255245654254
// 3446585845452
// 4546657867536
// 1438598798454
// 4457876987766
// 3637877979653
// 4654967986887
// 4564679986453
// 1224686865563
// 2546548887735
// 4322674655533
//
// 2413432311323
// 3215453535623
// 3255245654254
// 3446585845452
// 4546657867536
// 1438598798454
// 4457876987766
// 3637877979653
// 4654967986887
// 4564679986453
// 1224686865563
// 2546548887735
// 4322674655533
//
// 2>>34^>>>1323
// 32v>>>35v5623
// 32552456v>>54
// 3446585845v52
// 4546657867v>6
// 14385987984v4
// 44578769877v6
// 36378779796v>
// 465496798688v
// 456467998645v
// 12246868655<v
// 25465488877v5
// 43226746555v>
//
// 0..*..*..*....
// 1..*..*.......
// 2.............
// 3........*.*..
// 4.............
// 5.............
// 6..........**.
// 7.............
// 8.............
// 9...........**
// 0.............
// 1.............
// 2............*
//
//  5   4   5          23
//
//  3   5   6  11  15  20
//
//
//  5   4   5   8  19  22  24  27  36  37  40  42  50
//
//  3   5   6  11  15  20  27  32  35  40  46  44  47
//
//  6   7  11  16  17  21  26  32  40  44  46  49  51
//
//  9  11  15  21  22  29  31  39  43  48  50  54  53
//
// 20  16  19  25  28  33  38  46  49  55  55  58  59
//
// 21  25  22  30  35  42  46  53  58  63  59  63  67
//
// 25  29  27  34  42  49  55  62  66  70  76  69  73
//
// 28  34  37  41  49  56  62  71  78  87  80  74  76
//
// 44  40  42  45  54  60  67  80  86  92  88  91  83
//
// 48  45  48  49  55  62  71  89  94  98  92  96  99
//
// 48  47  49  53  59  70  76  84  90 102  97 102 102
//
// 50  55  53  59  64  68  76  84  98 103 110 105 107
