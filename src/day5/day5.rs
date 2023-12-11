use itertools::Itertools;
use std::fs::read_to_string;

pub fn run_day5_1() {
    let res_first = read_to_string("input/day5/in.txt").unwrap();

    let buf = res_first.split("\n\n").collect_vec();

    let seeds = buf[0]
        .split_ascii_whitespace()
        .skip(1)
        .map(|num| num.parse::<u64>().unwrap())
        .collect_vec();

    let maps = buf
        .iter()
        .skip(1)
        .map(|line| {
            line.split('\n')
                .filter(|x| !x.ends_with(':'))
                .map(|nums| {
                    nums.split_whitespace()
                        .map(|num| num.parse::<u64>().unwrap())
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect_vec();

    let res_first = seeds
        .iter()
        .map(|seed| {
            let mut x = *seed;
            for ms in &maps {
                for m in ms {
                    if x >= m[1] && x < m[1] + m[2] {
                        x = x - m[1] + m[0];
                        break;
                    }
                }
            }
            x
        })
        .min()
        .unwrap();
    println!("Res first: {:?}", res_first);
}

// fn run_day5_2() {
//     let mut seed_in = Vec::new();
//     let res = seeds
//         .iter()
//         .tuples()
//         .map(|(ss, se)| {
//             seed_in.push((*ss, ss + se - 1));
//             println!("Seed: {:?}", seed_in);

//             let cur = seed_in.pop().unwrap();

//             for ms in &maps {
//                 for m in ms {
//                     let is = m[1];
//                     let ie = m[1] + m[2];
//                     let i = (is, ie - 1);
//                     let x_less_a = cur.0 < is;
//                     let y_less_a = cur.1 < is;
//                     let x_great_b = cur.0 > ie;
//                     let y_great_b = cur.1 > ie;

//                     println!("Interval: {:?}", i);

//                     if !x_less_a & !y_great_b {
//                         println!("In interval: {:?}", !x_less_a & !y_great_b);
//                         seed_in.push((cur.0 - m[1] + m[0], cur.1 - m[1] + m[0]));
//                         println!("New: {:?}", seed_in);
//                         break;
//                     } else if x_less_a & !y_less_a {
//                         println!("Low than interval: {:?}", x_less_a & !y_less_a);
//                         seed_in.push((cur.0, cur.1 - m[1] + m[0]));
//                         seed_in.push((cur.0, cur.1 - m[1] + m[0]));
//                         println!("New: {:?}", seed_in);
//                         break;
//                     }
//                 }
//             }

//         })
//         .collect_vec();

//     println!("----");

//     println!("{:?}", res);

//           // println!(
//         //     "Not in interval: {:?}",
//         //     x_less_a & y_less_a || x_great_b & y_great_b
//         // );
//         // println!("Low than interval: {:?}", x_less_a & !y_less_a);
//         // println!("High than interval: {:?}", !x_great_b & y_great_b);
//         // let low = x.0 >= m[1] && x.1 > m[1];
//         // let high = x.0 < m[1] && x.1 < m[1];
//         // println!("Low:{low}, High:{high}");
//         // if low & high {
//         //     x = (x.0 - m[1] + m[0], x.1 - m[1] + m[0]);
//         //     break;
//         // } else if low & !high {
//         //     let x_0 = (x.0 - m[1] + m[0], m[1] - m[1] + m[0]);
//         //     let x_1 = (m[1] - m[1] + m[0] + 1, x.1 - m[1] + m[0]);
//         //     println!("{:?} and {:?}", x_0, x_1);
//         // } else if !low & high {
//         //     let x_0 = (x.0 - m[1] + m[0], m[1] - m[1] + m[0]);
//         //     let x_1 = (m[1] - m[1] + m[0] + 1, x.1 - m[1] + m[0]);
//         //     println!("{:?} and {:?}", x_0, x_1);
//         // }
// }
