use fxhash::FxHashSet as HashSet;
use std::cmp::{max, min};

// const INPUT: &[u8] = include_bytes!("../test.txt");
const INPUT: &[u8] = include_bytes!("../real.txt");

fn main() {
    let width = INPUT.splitn(2, |&c| c == b'\n').next().unwrap().len();
    let mut empty_ys: HashSet<usize> = HashSet::default();
    let mut empty_xs: HashSet<usize> = (0..width).collect();
    let mut nodes = HashSet::default();

    for (y, line) in INPUT.chunks_exact(width + 1).enumerate() {
        let mut empty_y = true;
        for (x, c) in line.into_iter().enumerate() {
            if *c == b'#' {
                empty_xs.remove(&x);
                empty_y = false;
                nodes.insert((x, y));
            }
        }
        if empty_y {
            empty_ys.insert(y);
        }
    }

    // try shortcut: find the shortest edge in the graph, remove nodes, repeat
    // this works by triangle inequality, probably

    let mut edges = vec![];
    for node_start in nodes.iter() {
        for node_end in nodes.iter() {
            if node_start < node_end {
                let min_x = min(node_start.0, node_end.0);
                let max_x = max(node_start.0, node_end.0);
                let min_y = min(node_start.1, node_end.1);
                let max_y = max(node_start.1, node_end.1);
                let empty_x_count =
                    empty_xs.iter().filter(|&&x| x > min_x && x < max_x).count() * (1000000 - 1);
                let empty_y_count =
                    empty_ys.iter().filter(|&&y| y > min_y && y < max_y).count() * (1000000 - 1);
                let space_manhatten = max_x - min_x + empty_x_count + max_y - min_y + empty_y_count;
                edges.push((space_manhatten, node_start, node_end));
            }
        }
    }
    edges.sort();

    let mut chosen_edges: HashSet<((usize, usize), (usize, usize))> = HashSet::default();
    let mut p1 = 0;
    for (dist, start, end) in edges {
        if chosen_edges.insert((*start, *end)) {
            p1 += dist;
        }
    }
    println!("p1: {p1}");
}
