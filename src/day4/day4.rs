use std::collections::HashSet;
use std::fs::read_to_string;
use std::u32;

fn parce_line(line: &str) -> u32 {
    let res: Vec<HashSet<u32>> = line
        .split(": ")
        .skip(1)
        .flat_map(|x| x.split(" | "))
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let num: usize = res
        .iter()
        .zip(&res[1..])
        .map(|(prev, next)| next.intersection(&prev).count())
        .sum();
    if num == 0 {
        0
    } else {
        1 << (num - 1)
    }
}

fn parce_line_second(line: &str) -> usize {
    let res: Vec<HashSet<u32>> = line
        .split(": ")
        .skip(1)
        .flat_map(|x| x.split(" | "))
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    res.iter()
        .zip(&res[1..])
        .map(|(prev, next)| next.intersection(&prev).count())
        .sum()
}

pub fn run_day4_1() {
    let res_first: u32 = read_to_string("input/day4/in.txt")
        .unwrap()
        .lines()
        .map(parce_line)
        .sum();

    println!("Final Result First Part: {res_first}");
}

pub fn run_day4_2() {
    let read = read_to_string("input/day4/in.txt").unwrap();
    let lines: Vec<&str> = read.lines().collect();

    let mut scratch_map: Vec<u32> = vec![1; lines.len()];

    for (i, line) in lines.iter().enumerate() {
        let maps = parce_line_second(line);
        for j in 1..maps + 1 {
            scratch_map[i + j] += scratch_map[i];
        }
    }

    println!(
        "Final Result Second Part: {:?}",
        scratch_map.into_iter().sum::<u32>()
    );
}
