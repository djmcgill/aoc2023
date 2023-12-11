use std::fs::File;
use std::time::Instant;

// const INPUT: &[u8] = include_bytes!("../test.txt");
const INPUT: &[u8] = include_bytes!("../real.txt");

// thanks to manhatten distances, the x_dists and y_dists can be calculated independently
fn main() {
    let start = Instant::now();

    let width = INPUT.splitn(2, |&c| c == b'\n').next().unwrap().len() as isize;
    let height = INPUT.len() as isize / (width + 1);

    let ys_iter = (0..height).map(|y| (y, (0..width).map(move |x| y * (width + 1) + x)));
    let xs_iter = (0..width).map(|x| (x, (0..height).map(move |y| y * (width + 1) + x)));

    let p1_mult = 1;
    let p1 = scan_dimension(ys_iter.clone(), p1_mult) + scan_dimension(xs_iter.clone(), p1_mult);

    let p2_mult = 1000000 - 1;
    let p2 = scan_dimension(ys_iter, p2_mult) + scan_dimension(xs_iter, p2_mult);

    let end = Instant::now();

    println!("p1: {p1}");
    println!("p2: {p2}");
    println!("p1+p2: {:?}", end - start);
}

// okay let's say you have xs = [1,2,3,4]
// then the xs dists are: 4-3 + 4-2 + 4-1 + 3-2 + 3-1 + 2-1
// or: 4*(len-1) + 3*(len-2) + 2*(len-3) + 1*(len-4)
//   - 4*(len-4) - 3*(len-3) + 2*(len-2) - 1(*len-1)
//   = 4*(len-1) + 4*(4-len) + ...
//   = 4*(len -1 + 4 - len) + ...
//   = 4*3 + ...
fn scan_dimension<Min: Iterator<Item = isize>, Maj: Iterator<Item = (isize, Min)>>(
    iter: Maj,
    mult: isize,
) -> isize {
    let mut ys = vec![];
    let mut empties = 0isize;

    // hmm how can I make this a list comprehension
    for (maj, x_iter) in iter {
        let maj_count = x_iter.filter(|&ix| INPUT[ix as usize] == b'#').count();
        if maj_count == 0 {
            empties += 1;
        } else {
            for _ in 0..maj_count {
                ys.push(maj + empties * mult);
            }
        }
    }

    let len = ys.len() as isize;
    ys.iter()
        .enumerate()
        .rev()
        .map(|(n, y)| *y * ((2 * n as isize + 1) - len))
        .sum()
}
