use std::fs;
// Day 3 
fn main() {
    let rankings = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
    ];

    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    
    let mut points = 0;
    let mut group = ["", "", ""];
    let mut group_index: i8 = 0;
    for line in contents.lines() {
        group[group_index as usize] = line;
        if group_index == 2 {
            for c in group[0].chars() {
                if group[1].contains(c) && group[2].contains(c) {
                    points += rankings.iter().position(|&x| x == c).unwrap() + 1;
                    break;
                }
            }
            group_index = -1;
        }
        group_index += 1;
    }

    println!("Total Points: {}", points);
}
