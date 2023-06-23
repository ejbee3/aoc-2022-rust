use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("wrong file name!");

    let mut elves: Vec<Vec<u32>> = vec![];
    let mut elf: Vec<u32> = vec![];
    let mut totals: Vec<u32> = vec![];
    let mut current_largest: u32 = 0;

    for line in contents.lines() {
        if line.is_empty() {
            elves.push(elf.clone());
            elf.clear();
        } else {
            let calories: u32 = line.parse().unwrap();
            elf.push(calories);
        }
    }

    for e in elves {
        let total_cals: u32 = e.iter().sum();
        totals.push(total_cals);
    }

    for (i, _) in &mut totals.iter().enumerate() {
        if totals.get(i + 1).is_some() {
            if current_largest < totals[i + 1] {
                current_largest = totals[i + 1];
            }
        }
    }

    println!("most calories: {current_largest}");
}
