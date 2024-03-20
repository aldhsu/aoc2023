use std::{
    array,
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    str::FromStr,
};

use anyhow::{anyhow, Context, Error, Result};

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input)?);
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct Coord(usize, usize);

impl Coord {
    const OFFSETS: [(isize, isize, Dir); 4] = [
        (0, -1, Dir::North),
        (-1, 0, Dir::West),
        (1, 0, Dir::East),
        (0, 1, Dir::South),
    ];

    fn neighbors(&self) -> [Option<(Coord, Dir)>; 4] {
        fn make_coord(coord: &Coord, off_x: isize, off_y: isize, dir: Dir) -> Option<(Coord, Dir)> {
            Some((
                Coord(
                    coord.0.checked_add_signed(off_x)?,
                    coord.1.checked_add_signed(off_y)?,
                ),
                dir,
            ))
        }
        array::from_fn(|i| {
            let (off_x, off_y, dir) = Self::OFFSETS[i];
            make_coord(self, off_x, off_y, dir)
        })
    }
}

#[derive(Eq, PartialEq)]
struct Map {
    inner: HashMap<Coord, usize>,
    width: usize,
    height: usize,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let inner = s
            .lines()
            .enumerate()
            .flat_map(move |(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    (
                        Coord(x, y),
                        c.to_digit(10).expect("couldn't get digit") as usize,
                    )
                })
            })
            .collect();
        let width = s
            .lines()
            .next()
            .map(|line| line.chars().count())
            .context("couldn't get width")?;
        let height = s.lines().count();

        Ok(Self {
            inner,
            width,
            height,
        })
    }
}

impl Map {
    fn get(&self, coord: &Coord) -> Option<&usize> {
        self.inner.get(coord)
    }
}

#[derive(Clone, Copy)]
struct Cursor {
    coord: Coord,
    dir: Dir,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(Eq, PartialEq, Clone)]
struct State<'a> {
    heat_loss: usize,
    head: Coord,
    dir_list: [Option<Dir>; 3],
    map: &'a Map,
}

impl<'a> Ord for State<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heat_loss.cmp(&other.heat_loss)
    }
}

impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.heat_loss.partial_cmp(&other.heat_loss)
    }
}

type Work<'a> = BinaryHeap<Reverse<State<'a>>>;
type DirList = [Option<Dir>; 3];

impl<'a> State<'a> {
    fn new(map: &'a Map) -> Self {
        Self {
            head: Coord(0, 0),
            dir_list: [None; 3],
            heat_loss: 0,
            map,
        }
    }

    fn solve(&'a self) -> Result<usize> {
        // visited hash with lowest values
        // path finding minheap
        let mut work: Work<'a> = BinaryHeap::from([Reverse(self.clone())]);
        let mut visited: HashMap<(Coord, DirList), usize> = HashMap::new();

        fn add_work<'a>(
            state: State<'a>,
            work: &'_ mut Work<'a>,
            visited: &'_ mut HashMap<(Coord, DirList), usize>,
            map: &Map,
        ) {
            for opt in state.head.neighbors().into_iter() {
                match opt {
                    Some((coord, dir)) => {
                        let Some(heat_loss) = state.map.get(&coord) else {
                            continue;
                        };

                        let total_heat_loss = state.heat_loss + heat_loss;
                        // if let Some(exit) = visited.get(&(coord, dir)) {
                        //     if &total_heat_loss > exit {
                        //         continue;
                        //     }
                        // }

                        // if let Some(previous_loss) = visited.get(&(coord, dir)) {
                        //     if *previous_loss < total_heat_loss {
                        //         continue;
                        //     }
                        // }
                        if state.dir_list.iter().all(|prev| prev == &Some(dir)) {
                            continue;
                        }

                        let mut new_work = State {
                            head: coord,
                            heat_loss: total_heat_loss,
                            ..state
                        };

                        new_work.dir_list.rotate_left(1);
                        new_work.dir_list.last_mut().map(|last| *last = Some(dir));

                        visited.insert((coord, state.dir_list), total_heat_loss);
                        work.push(Reverse(new_work));
                    }
                    None => continue,
                }
            }
        }

        // fn print_visited(visited: &HashMap<Coord, usize>, map: &Map) {
        //     for y in 0..map.height {
        //         let mut string = String::new();
        //         for x in 0..map.width {
        //             string.push_str(&format!("{:3}", visited.get(&Coord(x, y)).unwrap()));
        //             string.push(' ');
        //         }
        //
        //         println!("{}\n", string);
        //     }
        // }

        while let Some(Reverse(state)) = work.pop() {
            add_work(state, &mut work, &mut visited, self.map);
        }

        // print_visited(&visited, self.map);

        let exit = Coord(self.map.width - 1, self.map.height - 1);

        visited
            .into_iter()
            .filter(|((coord, _), _)| *coord == exit)
            .map(|(_, val)| val)
            .min()
            .context("couldn't find end")
    }
}

fn part1(s: &str) -> Result<usize> {
    let map: Map = s.parse()?;
    let state = State::new(&map);

    state.solve()
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
