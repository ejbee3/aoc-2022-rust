use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("wrong file name!");

    let mut big_container: Vec<Vec<&str>> = vec![];

    for _ in 0..9 {
        big_container.push(vec![]);
    }

    for (i, line) in contents.lines().into_iter().enumerate() {
        if i <= 7 {
            let boxes: Vec<_> = line.match_indices(char::is_uppercase).collect();

            // println!("{:?}", v[0].0);

            for b in boxes {
                let mut col_num = b.0 as f32;
                col_num = (col_num / 4.0_f32).ceil();
                let col_i = col_num as usize;
                big_container[col_i - 1].push(b.1);
            }
        } else if i == 8 {
            let number_of_cols: Vec<&str> = line.matches(char::is_numeric).collect();
            for i in 0..number_of_cols.len() {
                big_container[i].reverse();
            }
        } else if i > 9 {
            let moves: Vec<&str> = line.matches(char::is_numeric).collect();
            let mut m = String::from("0");
            if moves.len() == 4 {
                m = String::from(moves[0]);
                m.push_str(moves[1]);
            } else {
                m.push_str(moves[0]);
            }

            for _ in 0..m.parse::<usize>().unwrap() {
                if moves.len() == 4 {
                    let b: &str = big_container[moves[2].parse::<usize>().unwrap() - 1]
                        .pop()
                        .expect("element wasn't removed");
                    big_container[moves[3].parse::<usize>().unwrap() - 1].push(b);
                } else {
                    let b: &str = big_container[moves[1].parse::<usize>().unwrap() - 1]
                        .pop()
                        .expect("element wasn't pushed");
                    big_container[moves[2].parse::<usize>().unwrap() - 1].push(b);
                }
            }
        }
    }

    let top_crates: Vec<&str> = big_container.iter().map(|c| c[c.len() - 1]).collect();
    let joined_crates = top_crates.join("");

    println!("{:?}", joined_crates);
}
