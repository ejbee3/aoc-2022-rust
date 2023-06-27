use itertools::Itertools;
use std::fs;

fn main() {
    // let d = 'D';
    // let d_i = d as u32;

    // println!("{:?}", d_i);
    // 68

    let contents = fs::read_to_string("input.txt").expect("wrong file name!");
    let mut priorities: Vec<u8> = vec![];

    for line in contents.lines() {
        let halfway = line.len() / 2;
        let first_half = &line[0..halfway];
        let second_half = &line[halfway..line.len()];

        let vec_first = first_half.as_bytes();
        let vec_second = second_half.as_bytes();
        let mut matching_byte: u8 = 0;

        for a in vec_first.into_iter().cartesian_product(vec_second) {
            if a.0 == a.1 {
                matching_byte = *a.0;
                break;
            }
        }

        if matching_byte >= 65 && matching_byte <= 90 {
            let upper_prio = matching_byte - 38;
            priorities.push(upper_prio);
        } else {
            let lower_prio = matching_byte - 96;
            priorities.push(lower_prio);
        }
    }

    let priorities_sum = priorities.iter().fold(0u32, |sum, i| sum + (*i as u32));

    // let matching_char = matching_byte as char;

    println!("{:?}", priorities_sum);
    // println!("{:?}", vec_b);
}
