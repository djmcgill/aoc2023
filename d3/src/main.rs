use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};
use std::{cell::RefCell, ops::RangeInclusive, rc::Rc, time::Instant};

// const INPUT: &str = include_str!("../test.txt");
const INPUT: &[u8] = include_bytes!("../real.txt");

// top left is 0,0
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Number {
    value: u32,
    coord: Coord,
    len: usize,
}

#[derive(Debug)]
struct Problem {
    numbers_grid: HashMap<Coord, Rc<RefCell<Number>>>,
    numbers: Vec<Rc<RefCell<Number>>>,
    symbols: HashSet<Coord>,
    gears: Vec<Coord>,
}
impl Problem {
    fn parse(s: &[u8]) -> Self {
        let mut problem = Problem {
            numbers_grid: HashMap::default(),
            numbers: Vec::new(),
            symbols: HashSet::default(),
            gears: Vec::new(),
        };
        for (y, line) in s.split(|x| *x == b'\n').enumerate() {
            let mut current_number = None;
            for (x, c) in line.into_iter().enumerate() {
                if (b'0'..=b'9').contains(&c) {
                    let value = c - b'0';
                    match &current_number {
                        None => {
                            let rc_number = Rc::new(RefCell::new(Number {
                                value: value as u32,
                                len: 1,
                                coord: Coord { x, y },
                            }));

                            current_number = Some(Rc::clone(&rc_number));
                            problem.numbers_grid.insert(Coord { x, y }, rc_number);
                        }
                        Some(old_number) => {
                            let rc_old_number = Rc::clone(&old_number);
                            let mut r = (**old_number).borrow_mut();
                            r.value *= 10;
                            r.value += value as u32;
                            r.len += 1;
                            problem.numbers_grid.insert(Coord { x, y }, rc_old_number);
                        }
                    }
                } else {
                    if let Some(number) = current_number.take() {
                        problem.numbers.push(number);
                    }

                    if *c != b'.' {
                        problem.symbols.insert(Coord { y, x });
                        if *c == b'*' {
                            problem.gears.push(Coord { y, x });
                        }
                    }
                }
            }
            if let Some(number) = current_number.take() {
                problem.numbers.push(number);
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

    'numbers: for number in &problem.numbers {
        for (y, xs) in neighbours(&*number.borrow()) {
            for x in xs {
                if problem.symbols.contains(&Coord { x, y }) {
                    // println!("part number: {}", number.value);
                    part_number_sum += number.borrow().value;
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

                        if let Some(number) = problem.numbers_grid.get(&Coord {
                            y: target_y,
                            x: target_x,
                        }) {
                            let number = (**number).borrow();
                            adjacent.push(number.value);

                            // how far through the number are we
                            let number_progress = target_x - number.coord.x;
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
    println!("parse: {:?}", parse_end - start); // 205.3µs
    println!("p1: {:?}", p1_end - parse_end); // 83.5µs
    println!("p2: {:?}", p2_end - p1_end); // 54.4µs
}

fn neighbours(number: &Number) -> Vec<(usize, RangeInclusive<usize>)> {
    let coord = number.coord;
    match (coord.x, coord.y) {
        (0, 0) => vec![
            (coord.y, coord.x + number.len..=coord.x + number.len),
            (coord.y + 1, coord.x..=coord.x + number.len),
        ],
        (0, _) => vec![
            (coord.y - 1, coord.x..=coord.x + number.len),
            (coord.y, coord.x + number.len..=coord.x + number.len),
            (coord.y + 1, coord.x..=coord.x + number.len),
        ],
        (_, 0) => vec![
            (coord.y, coord.x - 1..=coord.x - 1),
            (coord.y, coord.x + number.len..=coord.x + number.len),
            (coord.y + 1, coord.x - 1..=coord.x + number.len),
        ],
        _ => vec![
            (coord.y - 1, coord.x - 1..=coord.x + number.len),
            (coord.y, coord.x - 1..=coord.x - 1),
            (coord.y, coord.x + number.len..=coord.x + number.len),
            (coord.y + 1, coord.x - 1..=coord.x + number.len),
        ],
    }
}
