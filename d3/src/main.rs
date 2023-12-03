use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};
use std::{ops::RangeInclusive, time::Instant};

// const INPUT: &str = include_str!("../test.txt");
const INPUT: &[u8] = include_bytes!("../real.txt");

// top left is 0,0
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Number {
    value: u32,
    len: usize,
}

#[derive(Debug)]
struct Problem {
    numbers: HashMap<Coord, Number>,
    links: HashMap<Coord, Coord>,
    symbols: HashSet<Coord>,
    gears: Vec<Coord>,
}
impl Problem {
    fn parse(s: &[u8]) -> Self {
        let mut problem = Problem {
            numbers: HashMap::default(),
            symbols: HashSet::default(),
            links: HashMap::default(),
            gears: Vec::new(),
        };
        for (y, line) in s.split(|x| *x == b'\n').enumerate() {
            let mut current_number = None;
            for (x, c) in line.into_iter().enumerate() {
                if (b'0'..=b'9').contains(&c) {
                    let value = c - b'0';
                    match current_number {
                        None => {
                            problem.links.insert(Coord { x, y }, Coord { x, y });
                            current_number = Some((
                                x,
                                Number {
                                    value: value as u32,
                                    len: 1,
                                },
                            ));
                        }
                        Some((old_start, old_number)) => {
                            problem
                                .links
                                .insert(Coord { x, y }, Coord { x: old_start, y });
                            current_number = Some((
                                old_start,
                                Number {
                                    value: old_number.value * 10 + (value as u32),
                                    len: old_number.len + 1,
                                },
                            ));
                        }
                    }
                } else {
                    if let Some((start, number)) = current_number {
                        problem.numbers.insert(Coord { y, x: start }, number);
                        current_number = None;
                    }

                    if *c != b'.' {
                        problem.symbols.insert(Coord { y, x });
                        if *c == b'*' {
                            problem.gears.push(Coord { y, x });
                        }
                    }
                }
            }
            if let Some((start, number)) = current_number {
                problem.numbers.insert(Coord { y, x: start }, number);
            }
        }
        problem
    }
}

fn main() {
    let start = Instant::now();
    let problem = Problem::parse(INPUT);
    let parse_end = Instant::now();

    let mut part_number_sum = 0;

    'numbers: for (coord, number) in &problem.numbers {
        for (y, xs) in neighbours((coord, number)) {
            for x in xs {
                if problem.symbols.contains(&Coord { x, y }) {
                    // println!("part number: {}", number.value);
                    part_number_sum += number.value;
                    continue 'numbers;
                }
            }
        }
    }
    let p1_end = Instant::now();

    let mut p2 = 0;
    for gear in problem.gears {
        let mut adjacent = vec![];

        let dxs: &[isize] = if gear.x == 0 { &[0, 1] } else { &[-1, 0, 1] };
        let dys: &[isize] = if gear.y == 0 { &[0, 1] } else { &[-1, 0, 1] };

        for dy in dys {
            let mut skip = 0;
            for dx in dxs {
                if *dy != 0 || *dx != 0 {
                    if skip != 0 && (*dy == -1 || *dy == 1) {
                        skip -= 1;
                    } else {
                        let target_x = ((gear.x as isize) + dx) as usize;
                        let target_y = ((gear.y as isize) + dy) as usize;
                        if let Some(source) = problem.links.get(&Coord {
                            y: target_y,
                            x: target_x,
                        }) {
                            let number = problem.numbers.get(source).unwrap();
                            adjacent.push(number.value);

                            // how far through the number are we
                            let number_progress = target_x - source.x;
                            skip = number.len - number_progress;
                        }
                    }
                }
            }
        }

        if adjacent.len() == 2 {
            p2 += adjacent[0] * adjacent[1];
        }
    }

    let p2_end = Instant::now();

    println!("p1: {part_number_sum}");
    println!("p2: {p2}");
    println!("parse: {:?}", parse_end - start);
    println!("p1: {:?}", p1_end - parse_end);
    println!("p2: {:?}", p2_end - p1_end);
}

fn neighbours((coord, number): (&Coord, &Number)) -> Vec<(usize, RangeInclusive<usize>)> {
    if coord.y != 0 {
        if coord.x != 0 {
            vec![
                (coord.y - 1, coord.x - 1..=coord.x + number.len),
                (coord.y, coord.x - 1..=coord.x - 1),
                (coord.y, coord.x + number.len..=coord.x + number.len),
                (coord.y + 1, coord.x - 1..=coord.x + number.len),
            ]
        } else {
            vec![
                (coord.y - 1, coord.x..=coord.x + number.len),
                (coord.y, coord.x + number.len..=coord.x + number.len),
                (coord.y + 1, coord.x..=coord.x + number.len),
            ]
        }
    } else {
        if coord.x != 0 {
            vec![
                (coord.y, coord.x - 1..=coord.x - 1),
                (coord.y, coord.x + number.len..=coord.x + number.len),
                (coord.y + 1, coord.x - 1..=coord.x + number.len),
            ]
        } else {
            vec![
                (coord.y, coord.x + number.len..=coord.x + number.len),
                (coord.y + 1, coord.x..=coord.x + number.len),
            ]
        }
    }

    // let mut neighbours = vec![];
    // // above
    // if coord.y != 0 {
    //     if coord.x != 0 {
    //         neighbours.push(Coord {
    //             x: coord.x - 1,
    //             y: coord.y - 1,
    //         });
    //     }
    //     for n in 0..=number.len {
    //         neighbours.push(Coord {
    //             x: coord.x + n,
    //             y: coord.y - 1,
    //         });
    //     }
    // }

    // if coord.x != 0 {
    //     neighbours.push(Coord {
    //         x: coord.x - 1,
    //         y: coord.y,
    //     });
    // }
    // neighbours.push(Coord {
    //     x: coord.x + number.len,
    //     y: coord.y,
    // });

    // // below
    // if coord.x != 0 {
    //     neighbours.push(Coord {
    //         x: coord.x - 1,
    //         y: coord.y + 1,
    //     });
    // }
    // for n in 0..=number.len {
    //     neighbours.push(Coord {
    //         x: coord.x + n,
    //         y: coord.y + 1,
    //     });
    // }
    // neighbours
}
