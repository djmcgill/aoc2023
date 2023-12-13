use std::time::Instant;

// const INPUT: &[u8] = include_bytes!("../test.txt");
const INPUT: &[u8] = include_bytes!("../real.txt");

fn main() {
    let start = Instant::now();
    let grids: Vec<Vec<&'static [u8]>> = INPUT
        .split(|&c| c == b'\n')
        .scan(vec![], |grids, line| {
            if line.is_empty() {
                let mut ret = vec![];
                std::mem::swap(&mut ret, grids);
                Some(Some(ret))
            } else {
                grids.push(line);
                Some(None)
            }
        })
        .filter_map(|x| x)
        .collect();

    let mut p1 = 0;
    let mut p2 = 0;
    for grid in grids.into_iter() {
        let mut horizontal_line = None;
        let mut vertical_line = None;

        if let Some(y) = check_for_y_p1(&grid[..]) {
            let y = y + 1; // 1-indexed
            p1 += 100 * y;
            horizontal_line = Some(y);
        }
        if let Some(x) = check_for_x_p1(&grid[..]) {
            p1 += x;
            vertical_line = Some(x);
        }

        // okay we found the old line with smudge. now,
        if let Some(y) = check_for_y_p2(&grid[..], horizontal_line) {
            p2 += 100 * y;
        }
        if let Some(x) = check_for_x_p2(&grid[..], vertical_line) {
            p2 += x;
        }
    }
    let end = Instant::now();
    println!("p1: {p1}");
    println!("p2: {p2}");
    println!("p1+p2: {:?}", end - start);
}

fn check_for_y_p1(grid: &[&'static [u8]]) -> Option<usize> {
    (0..grid.len() - 1).find(|y| {
        // okay so for each y, is this line a horizontal mirror.
        // where y is the start of the reflection

        let mut dist_from_center = 0;
        loop {
            if grid[y - dist_from_center] == grid[y + dist_from_center + 1] {
                // this row is fine check the next one
                if y - dist_from_center == 0 || y + dist_from_center + 1 >= grid.len() - 1 {
                    // index from 1
                    return true;
                } else {
                    dist_from_center += 1;
                }
            } else {
                // we know this is wrong
                break;
            }
        }
        false
    })
}

fn check_for_y_p2(grid: &[&'static [u8]], line_found: Option<usize>) -> Option<usize> {
    for y in 1..grid.len() {
        let mut dist_from_center = 0;
        let mut mistakes = 0;

        if let Some(found_y) = line_found {
            if y == found_y {
                continue;
            }
        }

        loop {
            let row_1 = grid[y - dist_from_center - 1].iter();
            let row_2 = grid[y + dist_from_center].iter();
            let mistakes_row = row_1.zip(row_2).filter(|(a, b)| a != b).count();
            mistakes += mistakes_row;

            if mistakes < 2 {
                // this row is fine check the next one
                if y - dist_from_center - 1 == 0 || y + dist_from_center >= grid.len() - 1 {
                    // we're done
                    // println!("grid {n}: match found at y {y}, mistakes: {mistakes}");
                    return Some(y);
                } else {
                    // continue
                    dist_from_center += 1;
                }
            } else {
                // we know this is wrong

                break;
            }
        }
    }
    None
}

fn check_for_x_p1(grid: &[&'static [u8]]) -> Option<usize> {
    for x in 0..grid[0].len() - 1 {
        let mut dist_from_center = 0;
        loop {
            // dbg!(x, dist_from_center, grid.len(), grid[0].len());
            let col_1 = (0..grid.len()).map(|y| grid[y][x - dist_from_center]);
            let col_2 = (0..grid.len()).map(|y| grid[y][x + dist_from_center + 1]);
            let col_okay = col_1.zip(col_2).all(|(a, b)| a == b);

            if col_okay {
                // this row is fine check the next one
                if x - dist_from_center == 0 || x + dist_from_center >= grid[0].len() - 2 {
                    return Some(x + 1);
                } else {
                    dist_from_center += 1;
                }
            } else {
                // we know this is wrong
                break;
            }
        }
    }
    None
}

fn check_for_x_p2(grid: &[&'static [u8]], vertical_line: Option<usize>) -> Option<usize> {
    for x in 1..grid[0].len() {
        let mut dist_from_center = 0;
        let mut mistakes = 0;
        if let Some(vertical_line) = vertical_line {
            if x == vertical_line {
                continue;
            }
        }

        loop {
            // dbg!(x, dist_from_center, grid.len(), grid[0].len());
            let col_1 = (0..grid.len()).map(|y| grid[y][x - dist_from_center - 1]);
            let col_2 = (0..grid.len()).map(|y| grid[y][x + dist_from_center]);
            let mistakes_col = col_1.zip(col_2).filter(|(a, b)| a != b).count();
            mistakes += mistakes_col;

            if mistakes < 2 {
                // this row is fine check the next one
                if x - dist_from_center - 1 == 0 || x + dist_from_center >= grid[0].len() - 1 {
                    // we're done
                    // println!("grid {n}: match found at x {x}, mistakes: {mistakes}");
                    return Some(x);
                } else {
                    dist_from_center += 1;
                }
            } else {
                break;
            }
        }
    }
    None
}
