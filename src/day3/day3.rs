use std::collections::HashMap;
use std::fs::read_to_string;
pub fn run_day3_1() {
    const DOT: u8 = 46;

    let res_first = read_to_string("input/day3/in.txt").unwrap();
    let a: Vec<&[u8]> = res_first.lines().map(|l| l.as_bytes()).collect();

    let mut digit = String::new();
    let mut sum = 0;
    let mut is_touch = false;
    for (i, l) in a.clone().into_iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            if c.is_ascii_digit() {
                digit.push(char::from(*c));

                //check symbols around
                if is_touch {
                    continue;
                }

                let mut rows = Vec::new();
                let mut columns = Vec::new();

                if i == 0 {
                    rows.extend_from_slice(&[i, i + 1]);
                } else {
                    rows.extend_from_slice(&[i - 1, i, i + 1]);
                }

                if j == 0 {
                    columns.extend_from_slice(&[j, j + 1]);
                } else {
                    columns.extend_from_slice(&[j - 1, j, j + 1]);
                }

                for r in rows {
                    for col in columns.clone() {
                        if r == a.len() || col == l.len() {
                            break;
                        }
                        if a[r][col] != DOT && (a[r][col] < 48 || a[r][col] > 57) {
                            is_touch = true;
                            continue;
                        }
                    }
                }
            } else if (*c < 48 || *c > 57) && !digit.is_empty() && is_touch {
                sum += digit.parse::<u32>().unwrap();
                digit.clear();
                is_touch = false;
                continue;
            } else if (*c < 48 || *c > 57) && !digit.is_empty() && !is_touch {
                digit.clear();
                continue;
            }
        }
        if !digit.is_empty() && is_touch {
            sum += digit.parse::<u32>().unwrap();
            digit.clear();
            is_touch = false;
        }
    }

    println!("Final Result First Part: {:?}", sum);
}

pub fn run_day3_2() {
    const STAR: u8 = 42;
    
    let res_first = read_to_string("input/day3/in.txt").unwrap();
    let a: Vec<&[u8]> = res_first.lines().map(|l| l.as_bytes()).collect();

    let mut gear: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let mut digit = String::new();
    let mut is_touch = false;
    let mut coord: (usize, usize) = (0, 0);
    for (i, l) in a.clone().into_iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            if c.is_ascii_digit() {
                digit.push(char::from(*c));

                //check symbols around
                if is_touch {
                    continue;
                }

                let mut rows = Vec::new();
                let mut columns = Vec::new();

                if i == 0 {
                    rows.extend_from_slice(&[i, i + 1]);
                } else {
                    rows.extend_from_slice(&[i - 1, i, i + 1]);
                }

                if j == 0 {
                    columns.extend_from_slice(&[j, j + 1]);
                } else {
                    columns.extend_from_slice(&[j - 1, j, j + 1]);
                }

                for r in rows {
                    for col in columns.clone() {
                        if r == a.len() || col == l.len() {
                            break;
                        }
                        if a[r][col] == STAR {
                            is_touch = true;
                            coord = (r, col);
                            continue;
                        }
                    }
                }
            } else if (*c < 48 || *c > 57) && !digit.is_empty() && is_touch {
                let num = digit.parse::<u32>().unwrap();
                if gear.get(&coord).is_none() {
                    gear.insert(coord, vec![num]);
                } else {
                    gear.entry(coord).and_modify(|v| v.push(num));
                }
                digit.clear();
                coord = (0, 0);
                is_touch = false;
                continue;
            } else if (*c < 48 || *c > 57) && !digit.is_empty() && !is_touch {
                digit.clear();
                continue;
            }
        }
        if !digit.is_empty() && is_touch {
            let num = digit.parse::<u32>().unwrap();
            if gear.get(&coord).is_none() {
                gear.insert(coord, vec![num]);
            } else {
                gear.entry(coord).and_modify(|v| v.push(num));
            }
            digit.clear();
            coord = (0, 0);
            is_touch = false;
        }
    }

    let mut sum = 0;
    for (_, v) in gear {
        if v.len() == 2 {
            sum += v[0] * v[1];
        }
    }
    println!("Final Result Second Part: {:?}", sum);
}
