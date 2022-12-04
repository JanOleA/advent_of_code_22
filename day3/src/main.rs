use std::fs;


fn part1() {
    let rucksacks = fs::read_to_string("input.txt").expect("Error reading the input file.");

    let mut priority_sum = 0;

    'lineloop: for line in rucksacks.lines() {
        let line_len = line.len();
        let comp1 = &line[..line_len/2];
        let comp2 = &line[line_len/2..];

        for item1 in comp1.chars() {
            for item2 in comp2.chars() {
                if item1 == item2 { // At this point I didn't know about str.contains()
                    let mut priority = item1 as i32 - 38;
                    if item1.is_lowercase() {
                        priority -= 58;
                    }
                    priority_sum += priority;
                    continue 'lineloop;
                }
            }
        }
    }
    println!("{priority_sum}")
}


fn part2() {
    let rucksacks = fs::read_to_string("input.txt").expect("Error reading the input file.");

    let mut priority_sum = 0;

    let mut elf1: &str = "";
    let mut elf2: &str = "";
    let mut elf3: &str;
    'lineloop: for (i, line) in rucksacks.lines().enumerate() {
        match i % 3 {
            0 => {
                elf1 = line;
            },
            1 => {
                elf2 = line;
            },
            2 => {
                elf3 = line;
                for item1 in elf1.chars() {
                    if elf2.contains(item1) && elf3.contains(item1) { // this is where I learned about str.contains().
                        let mut priority = item1 as i32 - 38;
                        if item1.is_lowercase() {
                            priority -= 58;
                        }
                        priority_sum += priority;
                        continue 'lineloop;
                    }
                }
            },
            _ => {},
        }
    }
    println!("{priority_sum}")
}


fn main() {
    part1();
    part2();
}
