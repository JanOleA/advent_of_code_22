use std::fs;


fn part1(crates: &str) {
    let mut stacks: Vec<Vec<&str>> = vec![]; 

    for (i, line) in crates.lines().enumerate() {
        match i {
            0..=7 => {
                for stack in 0..=8 {
                    if i == 0 {
                        stacks.push(vec![]);
                    }
                    let item = &line[(1 + stack*4)..(2 + stack*4)];
                    if item != " " {
                        stacks[stack].push(item);
                    }
                }
            },
            8 => {
                for stack in stacks.iter_mut() {
                    stack.reverse();
                }
            }
            8..=9 => {
                continue;
            }
            _ => {
                let instructions: Vec<&str> = line.split(" ").collect();
                if instructions.len() < 6 {
                    continue;
                }
                let move_num: usize = instructions[1].trim().parse().unwrap();
                let move_from: usize = instructions[3].trim().parse().unwrap();
                let move_to: usize = instructions[5].trim().parse().unwrap();

                for _i in 0..move_num {
                    let move_val = stacks[move_from - 1].pop().unwrap();
                    stacks[move_to - 1].push(move_val);
                }
            },
        }
    }
    for stack in stacks.iter() {
        print!("{}", stack.last().unwrap());
    }
    print!("\n");
}


fn part2(crates: &str) {
    let mut stacks: Vec<Vec<&str>> = vec![]; 

    for (i, line) in crates.lines().enumerate() {
        match i {
            0..=7 => {
                for stack in 0..=8 {
                    if i == 0 {
                        stacks.push(vec![]);
                    }
                    let item = &line[(1 + stack*4)..(2 + stack*4)];
                    if item != " " {
                        stacks[stack].push(item);
                    }
                }
            },
            8 => {
                for stack in stacks.iter_mut() {
                    stack.reverse();
                }
            }
            8..=9 => {
                continue;
            }
            _ => {
                let instructions: Vec<&str> = line.split(" ").collect();
                if instructions.len() < 6 {
                    continue;
                }
                let move_num: usize = instructions[1].trim().parse().unwrap();
                let move_from: usize = instructions[3].trim().parse().unwrap();
                let move_to: usize = instructions[5].trim().parse().unwrap();

                let mut move_stack: Vec<&str> = vec![];
                for _i in 0..move_num {
                    move_stack.push(stacks[move_from - 1].pop().unwrap());
                }
                move_stack.reverse();
                for item in move_stack.iter() {
                    stacks[move_to - 1].push(item);
                }
            },
        }
    }
    for stack in stacks.iter() {
        print!("{}", stack.last().unwrap());
    }
    print!("\n");
}


fn main() {
    let crates = fs::read_to_string("input.txt").expect("Error reading file");

    part1(&crates);
    part2(&crates);
}
