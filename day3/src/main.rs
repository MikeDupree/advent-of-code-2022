use std::{fs, collections::HashMap};

fn main() {
    println!("Hello, world!");

    let rankings = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
    ];

    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    
    let mut points = 0;
    for line in contents.lines() {
        println!("line count {}", line.len());
        println!("total items per bag {}", line.len()/2);
        let (s1, s2) = line.split_at(line.len()/2);
        println!("line split {}", s1); 

        for c in s1.chars() {
            if s2.contains(c) {
                println!("found {}", c);
                let points_index = rankings.iter().position(|&x| x == c).unwrap();
                print!("points {}", points);
                points += points_index + 1;
                println!(" + {} = {}", points_index + 1, points);
                break;
            }
        }
    }

    println!("Total Points: {}", points);
}
