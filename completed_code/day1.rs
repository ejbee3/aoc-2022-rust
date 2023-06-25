use std::fs;

fn main() {
    let contents = fs::read_to_string("input1.txt").expect("wrong file name!");

    let mut elves: Vec<Vec<u32>> = vec![];
    let mut elf: Vec<u32> = vec![];
    let mut totals: Vec<u32> = vec![];
    let mut first_largest: u32 = 0;
    let mut second_largest: u32 = 0;
    let mut third_largest: u32 = 0;

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
            if first_largest < totals[i + 1] {
                first_largest = totals[i + 1];
            }
        }
    }

    let mut totals_first_filter = totals.clone();
    totals_first_filter.retain(|value| *value != first_largest);

    for (i, _) in &mut totals_first_filter.iter().enumerate() {
        if totals_first_filter.get(i + 1).is_some() {
            if second_largest < totals_first_filter[i + 1] {
                second_largest = totals_first_filter[i + 1];
            }
        }
    }

    let mut totals_second_filter = totals_first_filter.clone();
    totals_second_filter.retain(|value| *value != second_largest);

    for (i, _) in &mut totals_second_filter.iter().enumerate() {
        if totals_second_filter.get(i + 1).is_some() {
            if third_largest < totals_second_filter[i + 1] {
                third_largest = totals_second_filter[i + 1];
            }
        }
    }

    let top_three_total: u32 = first_largest + second_largest + third_largest;

    println!("first most calories: {}", first_largest);
    println!("second most calories: {}", second_largest);
    println!("third most calories: {}", third_largest);

    println!("added together: {}", top_three_total);

    // println!("{:?}", totals_first_filter);
    // println!("{:?}", totals);
}
