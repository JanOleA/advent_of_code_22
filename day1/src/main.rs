use std::fs;

fn main() {
    let calories = fs::read_to_string("input.txt").expect("Error reading the input file.");

    let mut max_holding_3rd = 0;
    let mut max_holding_2nd = 0;
    let mut max_holding = 0;
    let mut currently_holding = 0;

    for line in calories.lines() {
        if line == "" { // Completed counting one elf
            if currently_holding > max_holding {
                // if the current elf holds the most we've ever seen, shift all the record holders down one spot, and replace the top record holder
                max_holding_3rd = max_holding_2nd;
                max_holding_2nd = max_holding;
                max_holding = currently_holding;
            } else if currently_holding > max_holding_2nd {
                // if the current elf holds the second most we've ever seen, shift the second place into third, and replace the second record holder
                max_holding_3rd = max_holding_2nd;
                max_holding_2nd = currently_holding;
            } else if currently_holding > max_holding_3rd {
                // if the current elf holds the third most we've ever seen, and replace the third record holder
                max_holding_3rd = currently_holding;
            }
            // reset currently holding for the next elf
            currently_holding = 0;
        } else {
            currently_holding += match line.trim().parse() {
                Ok(num) => num,
                Err(_) => 0,
            };
        }
    }
    println!("{} {} {} {}", max_holding, max_holding_2nd, max_holding_3rd, (max_holding+max_holding_2nd+max_holding_3rd));
}
