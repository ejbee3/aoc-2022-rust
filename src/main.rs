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
        }
    }
    println!("{:?}", big_container);
}
