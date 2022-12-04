use std::fs;
use std::collections::HashMap;


fn part1() {
    let strategy = fs::read_to_string("input.txt").expect("Error reading the input file.");

    let mut outcomes: HashMap<&str, i32> = HashMap::new();

    // hardcode the coutcomes (a lil stupid, I know, but a learning opportunity for the hashmap)
    outcomes.insert("A X", 3 + 1); // draw
    outcomes.insert("A Y", 6 + 2); // win
    outcomes.insert("A Z", 0 + 3); // lose

    outcomes.insert("B X", 0 + 1); // lose
    outcomes.insert("B Y", 3 + 2); // draw
    outcomes.insert("B Z", 6 + 3); // win

    outcomes.insert("C X", 6 + 1); // win
    outcomes.insert("C Y", 0 + 2); // lose
    outcomes.insert("C Z", 3 + 3); // draw

    let mut score = 0;

    for line in strategy.lines() {
        score += match outcomes.get(line) {
            Some(num) => num,
            None => {
                println!("Unknown outcome: {line}");
                &0
            },
        };
    }
    println!("{score}");
}


fn get_hand_score(hand: &str, score_for_hand: &HashMap<&str, i32>) -> i32 {
    *match score_for_hand.get(hand) {
        Some(num) => num,
        None => {
            println!("Can't get score for choice because didn't know which hand to choose.");
            &0
        }
    }
}


fn part2() {
    let strategy = fs::read_to_string("input.txt").expect("Error reading the input file.");

    let mut score_for_hand: HashMap<&str, i32> = HashMap::new();

    score_for_hand.insert("A", 1);
    score_for_hand.insert("B", 2);
    score_for_hand.insert("C", 3);

    let mut wins: HashMap<&str, &str> = HashMap::new();
    wins.insert("A", "B");
    wins.insert("B", "C");
    wins.insert("C", "A");

    let mut losses: HashMap<&str, &str> = HashMap::new();
    losses.insert("B", "A");
    losses.insert("C", "B");
    losses.insert("A", "C");

    let mut score = 0;

    for line in strategy.lines() {
        let opponent_play = &line.trim()[0..1];
        let desired_outcome = &line.trim()[2..3];
        score += match desired_outcome {
            "X" => {
                let my_play = match losses.get(opponent_play) {
                    Some(play) => play,
                    None => {
                        println!("Unknown opponent play: {opponent_play}");
                        ""
                    },
                };
                get_hand_score(my_play, &score_for_hand)
            },
            "Y" => {
                get_hand_score(opponent_play, &score_for_hand) + 3
            },
            "Z" => {
                let my_play = match wins.get(opponent_play) {
                    Some(play) => play,
                    None => {
                        println!("Unknown opponent play: {opponent_play}");
                        ""
                    },
                };
                get_hand_score(my_play, &score_for_hand) + 6
            },
            _ => continue,
        }
    }
    println!("{score}");
}


fn main() {
    part1();
    part2();

}
