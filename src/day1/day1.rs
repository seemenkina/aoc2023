use std::{fs::read_to_string, u32};

fn match_num(n: &str) -> u32 {
    match n {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0,
    }
}

fn find_char_number(line: &str) -> u32 {
    let nums = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let first_d = line
        .find(|c: char| c.is_ascii_digit())
        .unwrap_or(line.len());
    let last_d = line.rfind(|c: char| c.is_ascii_digit()).unwrap_or(0);

    let mut res_line: Vec<(usize, &str)> = Vec::new();
    for num in nums {
        let res_char: Vec<(usize, &str)> = line.match_indices(num).collect();
        if !res_char.is_empty() {
            res_line.extend(res_char);
        }
    }

    if res_line.is_empty() {
        let res = format!(
            "{}{}",
            line.as_bytes()[first_d] as char,
            line.as_bytes()[last_d] as char,
        );
        res.parse::<u32>().unwrap()
    } else {
        let min = *res_line.iter().min_by_key(|x| x.0).unwrap();
        let min_char = if min.0 < first_d {
            char::from_digit(match_num(min.1), 10).unwrap()
        } else {
            line.as_bytes()[first_d] as char
        };

        let max = *res_line.iter().max_by_key(|x| x.0).unwrap();
        let max_char = if max.0 > last_d {
            char::from_digit(match_num(max.1), 10).unwrap()
        } else {
            line.as_bytes()[last_d] as char
        };

        let res = format!("{min_char}{max_char}");
        res.parse::<u32>().unwrap()
    }
}

fn find_number_simple(line: &str) -> u32 {
    let first = line.find(|c: char| c.is_ascii_digit()).unwrap();
    let last = line.rfind(|c: char| c.is_ascii_digit()).unwrap();

    let res = format!(
        "{}{}",
        line.as_bytes()[first] as char,
        line.as_bytes()[last] as char,
    );
    res.parse::<u32>().unwrap()
}

pub fn run_day1_1() {
    let res_first: u32 = read_to_string("input/day1/in.txt")
        .unwrap()
        .lines()
        .map(find_number_simple)
        .sum();

    println!("Final Result First Part: {res_first}");
}

pub fn run_day1_2() {
    let res_second: u32 = read_to_string("input/day1/in.txt")
        .unwrap()
        .lines()
        .map(find_char_number)
        .sum();

    println!("Final Result Second Part: {res_second}");
}
