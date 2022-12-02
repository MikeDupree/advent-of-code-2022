use std::{fs, collections::HashMap};

const INPUT_SRC: &str = "input.txt";
/**
* Rock Paper Scissors
* A, X = Rock
* B, Y = Paper
* C, Z = Scissors
*/
fn main() {
    println!("Day 2 :: Rock Paper Sissors");
    let hands: HashMap<char, &str> = HashMap::from(
        [('A', "ROCK"), ('B', "PAPER"), ('C', "SCISSORS"), ('X', "ROCK"), ('Y', "PAPER"), ('Z', "SCISSORS")]);       
    let hand_rankings: HashMap<&str, &str> = HashMap::from([("ROCK", "SCISSORS"), ("PAPER", "ROCK"), ("SCISSORS", "PAPER")]);       
    let points_per_hand: HashMap<&str, i32> = HashMap::from([("ROCK", 1), ("PAPER", 2), ("SCISSORS", 3)]);
     
    let contents = fs::read_to_string(INPUT_SRC)
        .expect("Should have been able to read the file");
    
    let mut player_points = 0;

    for line in contents.lines() {
        let opponent_hand = hands.get(&line.chars().nth(0).unwrap()).unwrap();
        let player_hand = hands.get(&line.chars().nth(2).unwrap()).unwrap();
        //println!("index : {}", opponent_hands.get(&line.chars().nth(0).unwrap());
        println!("{} VS {}", opponent_hand, player_hand);
        if hand_rankings.get(player_hand).unwrap() == opponent_hand {
            player_points += 6 + points_per_hand.get(player_hand).unwrap();
        } else if player_hand == opponent_hand {
            player_points += 3 + points_per_hand.get(player_hand).unwrap();
        } else {
            player_points += 0 + points_per_hand.get(player_hand).unwrap();
        }
    }

    println!("Player's Total: {}", player_points)
}
