use std::time::Instant;

use fxhash::FxHashMap as HashMap;

// const INPUT: &[u8] = include_bytes!("../test.txt");
// const INPUT: &[u8] = include_bytes!("../test2.txt");
// const INPUT: &[u8] = include_bytes!("../test3.txt");
const INPUT: &[u8] = include_bytes!("../real.txt");
const AAA: u32 = (b'A' as u32) << 16 | (b'A' as u32) << 8 | b'A' as u32;
const ZZZ: u32 = (b'Z' as u32) << 16 | (b'Z' as u32) << 8 | b'Z' as u32;

fn main() {
    let start = Instant::now();
    let mut lines = INPUT.splitn(2, |c| *c == b'\n');
    let instructions = lines.next().unwrap();
    let remaining = lines.next().unwrap();
    let remaining = &remaining[1..]; // drop \n
    let mut map = HashMap::default();
    let mut p2_start_nodes = vec![];
    // AAA = (BBB, CCC)\n
    for line in remaining.chunks_exact(17) {
        let source = (line[0] as u32) << 16 | (line[1] as u32) << 8 | line[2] as u32;
        let dest_l = (line[7] as u32) << 16 | (line[8] as u32) << 8 | line[9] as u32;
        let dest_r = (line[12] as u32) << 16 | (line[13] as u32) << 8 | line[14] as u32;
        map.insert(source, (dest_l, dest_r));
        if line[2] == b'A' {
            p2_start_nodes.push(source);
        }
    }
    let parse_end = Instant::now();

    let p1_instructions = instructions.into_iter().cycle().enumerate();
    let mut current_source: u32 = AAA;
    let mut p1 = 0;
    for (n, instruction) in p1_instructions {
        let dests = map.get(&current_source).unwrap();
        let next = match instruction {
            b'L' => dests.0,
            b'R' => dests.1,
            _ => unreachable!(),
        };
        if next == ZZZ {
            p1 = n + 1;
            break;
        } else {
            current_source = next;
        }
    }
    let p1_end = Instant::now();

    let mut p2 = 1;
    for start_node in p2_start_nodes {
        let mut len = 0;
        let mut current_source = start_node;
        let instruction_loop = instructions.into_iter().cycle().enumerate();
        for (n, instruction) in instruction_loop {
            let dests = map.get(&current_source).unwrap();
            let next = match instruction {
                b'L' => dests.0,
                b'R' => dests.1,
                _ => unreachable!(),
            };
            if (next & 0xFF) as u8 == b'Z' {
                len = n + 1;
                break;
            } else {
                current_source = next;
            }
        }
        p2 = num::integer::lcm(p2, len);
    }

    let p2_end = Instant::now();
    println!("p1: {p1}");
    println!("p2: {p2}");
    println!("parse: {:?}", parse_end - start);
    println!("p1: {:?}", p1_end - parse_end);
    println!("p2: {:?}", p2_end - p1_end);
}
