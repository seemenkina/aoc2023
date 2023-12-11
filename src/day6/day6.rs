use itertools::Itertools;
use std::fs::read_to_string;

pub fn run_day6() {
    let res_first = parse(read_to_string("input/day6/in.txt").unwrap());
    println!("Res first: {:?}", res_first);

    let res_second = parse(read_to_string("input/day6/in2.txt").unwrap());
    println!("Res second: {:?}", res_second)
}

fn parse(lines: String) -> u64 {
    let parse: Vec<Vec<u64>> = lines
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|num| num.parse::<u64>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut res = 1;
    parse[0]
        .clone()
        .into_iter()
        .zip(parse[1].clone())
        .map(|(t, d)| {
            let mut buf_res = 0;
            for i in 1..t {
                if i * (t - i) > d {
                    buf_res += 1;
                }
            }
            buf_res
        })
        .for_each(|e| res *= e);

    res
}
