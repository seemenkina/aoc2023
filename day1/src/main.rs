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

    println!("--- Line: {line} ---");
    let first_d = line
        .find(|c: char| c.is_ascii_digit())
        .unwrap_or(line.len());
    let last_d = line.rfind(|c: char| c.is_ascii_digit()).unwrap_or(0);
    println!("First position: {first_d}; Last position: {last_d}",);

    let mut res_line: Vec<(usize, &str)> = Vec::new();
    for num in nums {
        let res_char: Vec<(usize, &str)> = line.match_indices(num).collect();
        if !res_char.is_empty() {
            println!("char_num[{num}]:{:?}", res_char);
            res_line.extend(res_char);
        }
    }

    if res_line.is_empty() {
        let res = format!(
            "{}{}",
            line.as_bytes()[first_d] as char,
            line.as_bytes()[last_d] as char,
        );
        println!("Result:{}", res.parse::<u32>().unwrap());
        return res.parse::<u32>().unwrap();
    }
    {
        let min = *res_line.iter().min_by_key(|x| x.0).unwrap();
        let min_char = if min.0 < first_d {
            char::from_digit(match_num(min.1), 10).unwrap()
        } else {
            line.as_bytes()[first_d] as char
        };
        println!(
            "-- Result min: {:?} or position {first_d} = {:?}",
            min, min_char
        );

        let max = *res_line.iter().max_by_key(|x| x.0).unwrap();
        let max_char = if max.0 > last_d {
            char::from_digit(match_num(max.1), 10).unwrap()
        } else {
            line.as_bytes()[last_d] as char
        };
        println!(
            "-- Result max: {:?} or position {last_d} = {:?}",
            max, max_char
        );

        let res = format!("{min_char}{max_char}");
        println!("Result:{}", res.parse::<u32>().unwrap());
        res.parse::<u32>().unwrap()
    }
}

fn find_number_simple(line: &str) -> u32 {
    print!("Line:{line} --- ");
    let first = line.find(|c: char| c.is_ascii_digit()).unwrap();
    let last = line.rfind(|c: char| c.is_ascii_digit()).unwrap();
    println!(
        "First: {first}-{:?}; Last: {last}-{:?}",
        line.as_bytes()[first] as char,
        line.as_bytes()[last] as char
    );

    let res = format!(
        "{}{}",
        line.as_bytes()[first] as char,
        line.as_bytes()[last] as char,
    );
    println!("Result:{}", res.parse::<u32>().unwrap());
    res.parse::<u32>().unwrap()
}

fn main() {
    let res: u32 = read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(find_char_number)
        .sum();

    println!("Final Result: {res}");
}
