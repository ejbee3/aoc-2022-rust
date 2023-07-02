use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("wrong file name!");

    let mut count: u32 = 0;

    for line in contents.lines() {
        let split = line.split("-").collect::<Vec<&str>>();
        // println!("{:?}", split);

        let middle = split[1].split(",").collect::<Vec<&str>>();
        // println!("{:?}", middle);

        let first_pair = (num_parser(split[0])..num_parser(middle[0]) + 1).collect::<Vec<u32>>();
        // println!("{:?}", first_pair);

        let second_pair = (num_parser(middle[1])..num_parser(split[2]) + 1).collect::<Vec<u32>>();
        // println!("{:?}", second_pair);

        if first_pair.len() > second_pair.len()
            && second_pair[0] >= first_pair[0]
            && second_pair[second_pair.len() - 1] <= first_pair[first_pair.len() - 1]
        {
            count += 1;
        } else if first_pair[0] >= second_pair[0]
            && first_pair[first_pair.len() - 1] <= second_pair[second_pair.len() - 1]
        {
            count += 1;
        }
    }

    println!("{:?}", count);
}

fn num_parser(s: &str) -> u32 {
    s.parse().expect("this should be a number from the input!")
}
