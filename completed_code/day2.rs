use std::fs;

fn main() {
    let contents = fs::read_to_string("input2.txt").expect("wrong file name!");
    let mut totals: Vec<u8> = vec![];

    for line in contents.lines() {
        let round_total = get_round_total_two(line);
        totals.push(round_total);
    }

    // println!("{:?}", totals);
    let all_rounds_total = totals.iter().fold(0u32, |sum, i| sum + (*i as u32));

    println!("{}", all_rounds_total);
}

// fn get_round_total_one(ln: &str) -> u8 {
//     let total = match ln {
//         "B Z" => 9,
//         "A Y" => 8,
//         "C X" => 7,
//         "C Z" => 6,
//         "B Y" => 5,
//         "A X" => 4,
//         "A Z" => 3,
//         "C Y" => 2,
//         "B X" => 1,
//         _ => 0,
//     };

//     total
// }

fn get_round_total_two(ln: &str) -> u8 {
    let total = match ln {
        "B Z" => 9,
        "A Y" => 4,
        "C X" => 2,
        "C Z" => 7,
        "B Y" => 5,
        "A X" => 3,
        "A Z" => 8,
        "C Y" => 6,
        "B X" => 1,
        _ => 0,
    };

    total
}
