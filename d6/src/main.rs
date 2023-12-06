use std::time::Instant;

// const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../real.txt");
const INPUTB: &[u8] = include_bytes!("../real.txt");

fn parse_p1_line<'a>(s: &'a str) -> impl Iterator<Item = u32> + 'a {
    s.split_at(9)
        .1
        .split_ascii_whitespace()
        .map(|x| u32::from_str_radix(x, 10).unwrap())
}
fn parse_p2_line(s: &[u8]) -> f64 {
    let mut x = 0.0;
    for c in &s[9..] {
        if *c != b' ' {
            x *= 10.0;
            x += (c - b'0') as f64;
        }
    }
    x
}

fn main() {
    let start = Instant::now();
    // let's just brute force for now
    let problem = {
        let mut lines = INPUT.lines();
        std::iter::zip(
            parse_p1_line(lines.next().unwrap()),
            parse_p1_line(lines.next().unwrap()),
        )
    };
    let p1: usize = problem
        .map(|(time, distance)| (1..time).filter(|n| n * (time - n) > distance).count())
        .product();
    let p1_end = Instant::now();

    let (time, distance) = {
        let mut lines = INPUTB.split(|x| *x == b'\n');
        (
            parse_p2_line(lines.next().unwrap()),
            parse_p2_line(lines.next().unwrap()),
        )
    };

    // solve for n:
    // distance = n*(time-n)
    // 0 = n*time - n^2 - distance
    // (-b +/- root(b^2-4ac)) / 2a
    let a = -1.0;
    let b = time;
    let c = -1.0 * distance;
    let root = b * b - 4.0 * a * c;
    let s1 = (-b + root.sqrt()) / 2.0 * a;
    let s2 = (-b - root.sqrt()) / 2.0 * a;

    let lower = s1.ceil();
    let upper = s2.floor();
    let p2 = upper - lower + 1.0;
    let p2_end = Instant::now();

    println!("p1: {p1}");
    println!("p2: {p2}");
    println!("p1: {:?}", p1_end - start); // 1.7us
    println!("p2: {:?}", p2_end - p1_end); // 0ns lol
}
