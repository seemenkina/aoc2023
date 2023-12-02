use std::fs::read_to_string;
use std::u32;

fn parce_line(nl: usize, line: &str) -> u32 {
    const RED: u32 = 12;
    const GREEN: u32 = 13;
    const BLUE: u32 = 14;

    let i = line.find(": ").unwrap();
    let mut buf_line: String = line[i + 2..].to_string();
    // for checking last set of game
    buf_line.push(';');

    let mut digit = String::new();
    let mut buf = [0u32, 0, 0];
    let mut num = 0;
    for c in buf_line.chars() {
        if c.is_ascii_digit() {
            digit.push(c);
        } else if c.is_whitespace() && !digit.is_empty() {
            num = digit.parse::<u32>().unwrap();
            digit.clear();
            continue;
        } else {
            match c {
                'r' => {
                    buf[0] += num;
                    num = 0
                }
                'g' => {
                    buf[1] += num;
                    num = 0
                }
                'b' => {
                    buf[2] += num;
                    num = 0
                }
                ';' => {
                    if buf[0] > RED || buf[1] > GREEN || buf[2] > BLUE {
                        return 0;
                    } else {
                        buf = [0, 0, 0];
                    }
                }
                _ => continue,
            }
        }
    }

    (nl + 1) as u32
}

fn parce_line_second(line: &str) -> u32 {
    let mut red_max: u32 = 0;
    let mut green_max: u32 = 0;
    let mut blue_max: u32 = 0;

    let i = line.find(": ").unwrap();
    let mut buf_line: String = line[i + 2..].to_string();
    buf_line.push(';');

    let mut digit = String::new();
    let mut buf = [0u32, 0, 0];
    let mut num = 0;
    for c in buf_line.chars() {
        if c.is_ascii_digit() {
            digit.push(c);
        } else if c.is_whitespace() && !digit.is_empty() {
            num = digit.parse::<u32>().unwrap();
            digit.clear();
            continue;
        } else {
            match c {
                'r' => {
                    buf[0] += num;
                    num = 0
                }
                'g' => {
                    buf[1] += num;
                    num = 0
                }
                'b' => {
                    buf[2] += num;
                    num = 0
                }
                ';' => {
                    if buf[0] > red_max {
                        red_max = buf[0];
                    }
                    if buf[1] > green_max {
                        green_max = buf[1];
                    }
                    if buf[2] > blue_max {
                        blue_max = buf[2];
                    }
                    buf = [0, 0, 0];
                }
                _ => continue,
            }
        }
    }

    red_max * green_max * blue_max
}

pub fn run_day2_1() {
    let res_first: u32 = read_to_string("input/day2/in.txt")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(i, l)| parce_line(i, l))
        .sum();

    println!("Final Result First Part: {res_first}");
}

pub fn run_day2_2() {
    let res_second: u32 = read_to_string("input/day2/in.txt")
        .unwrap()
        .lines()
        .map(parce_line_second)
        .sum();

    println!("Final Result Second Part: {res_second}");
}
