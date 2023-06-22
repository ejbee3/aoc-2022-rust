use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("wrong file name!");

    for line in contents.lines() {
        println!("{line}")
    }
}
