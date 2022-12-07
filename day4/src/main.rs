use std::fs;


fn part1(pairs: &str) {
    let mut count = 0;
    for pair in pairs.lines() {
        let items: Vec<&str> = pair.split(",").collect();
        let first_from_to: Vec<&str> = items[0].split("-").collect();
        let second_from_to: Vec<&str> = items[1].split("-").collect();

        let first_from: i32 = first_from_to[0].trim().parse().unwrap();
        let first_to: i32 = first_from_to[1].trim().parse().unwrap();

        let second_from: i32 = second_from_to[0].trim().parse().unwrap();
        let second_to: i32 = second_from_to[1].trim().parse().unwrap();

        if (first_from >= second_from && first_to <= second_to) // first is fully contained in second
            || (second_from >= first_from && second_to <= first_to) { // second is fully contained in first 
            count += 1;
        }
    }
    println!("{count}");
}


fn part2(pairs: &str) {
    let mut count = 0;
    for pair in pairs.lines() {
        let items: Vec<&str> = pair.split(",").collect();
        let first_from_to: Vec<&str> = items[0].split("-").collect();
        let second_from_to: Vec<&str> = items[1].split("-").collect();

        let first_from: i32 = first_from_to[0].trim().parse().unwrap();
        let first_to: i32 = first_from_to[1].trim().parse().unwrap();

        let second_from: i32 = second_from_to[0].trim().parse().unwrap();
        let second_to: i32 = second_from_to[1].trim().parse().unwrap();

        if !(second_from > first_to || second_to < first_from) {
            count += 1;
        }
    }
    println!("{count}");
}


fn main() {
    let pairs = fs::read_to_string("input.txt").expect("Error reading file");

    part1(&pairs);
    part2(&pairs);
}
