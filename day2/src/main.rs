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
        [('A', "ROCK"), ('B', "PAPER"), ('C', "SCISSORS"), ('X', "LOSE"), ('Y', "DRAW"), ('Z', "WIN")]);       
    let losing_hand: HashMap<&str, &str> = HashMap::from([("ROCK", "SCISSORS"), ("PAPER", "ROCK"), ("SCISSORS", "PAPER")]);       
    let winning_hand: HashMap<&str, &str> = HashMap::from([("SCISSORS", "ROCK"), ("ROCK", "PAPER"), ("PAPER", "SCISSORS")]);       
    let points_per_hand: HashMap<&str, i32> = HashMap::from([("ROCK", 1), ("PAPER", 2), ("SCISSORS", 3)]);
     
    let contents = fs::read_to_string(INPUT_SRC)
        .expect("Should have been able to read the file");
    
    let mut player_points = 0;

    for line in contents.lines() {
        let opponent_hand = hands.get(&line.chars().nth(0).unwrap()).unwrap();
        let player_move = hands.get(&line.chars().nth(2).unwrap()).unwrap();
        
        println!("{} VS {}", opponent_hand, player_move);
        match player_move {
           &"WIN" => player_points += 6 + points_per_hand.get(winning_hand.get(opponent_hand).unwrap()).unwrap(),
           &"DRAW" => player_points += 3 + points_per_hand.get(opponent_hand).unwrap(),
           _ => player_points += 0 + points_per_hand.get(losing_hand.get(opponent_hand).unwrap()).unwrap(),
        }
    }

    println!("Player's Total: {}", player_points)
}
