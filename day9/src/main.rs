use std::collections::{HashMap, HashSet};
use std::{fs, iter};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    follow_head(&contents.as_str());
}

fn follow_head(c: &str) {
    let x = 0;
    let y = 1;
    let axis = HashMap::from([('U', 1), ('D', -1), ('L', -1), ('R', 1)]);
    let coord = HashMap::from([('U', y), ('D', y), ('L', x), ('R', x)]);
    let reset_coord = HashMap::from([('U', x), ('D', x), ('L', y), ('R', y)]);

    let mut tail_positions: Vec<_> = iter::repeat_with(|| vec![String::from("0,0")])
        .take(9)
        .collect();
    let mut tail_position: Vec<_> = iter::repeat_with(|| [0, 0]).take(9).collect();
    println!("t positions {:?}", tail_positions);
    let mut head_position = [0, 0];

    for line in c.lines() {
        let mut chars = line.chars().collect::<Vec<char>>();
        let dir = chars[0];
        chars.drain(0..2);

        let n: u32 = chars
            .into_iter()
            .collect::<String>()
            .as_str()
            .parse::<u32>()
            .ok()
            .unwrap();

        // Move Head / Tail one position at a time
        for i in 0..n {
            // Update head position
            head_position[*coord.get(&dir).unwrap()] += axis.get(&dir).unwrap();

            for tail in 0..tail_position.len() {
                // Check for tail movement
                let mut head_in_range = false;

                let mut head_of_tail = head_position.clone();
                if tail != 0 {
                    // If its not the first knot the head of that knot is the previous
                    // knot.
                    head_of_tail = tail_position[tail - 1].clone();
                }

                for xi in -1..2 {
                    //println!(" xi :: {}", xi);
                    for yi in -1..2 {
                        //println!(" yi :: {}", yi);
                        if tail_position[tail][x] + xi == head_of_tail[x]
                            && tail_position[tail][y] + yi == head_of_tail[y]
                        {
                            head_in_range = true;
                        }
                    }
                }

                // Head is in range. No tail movement.
                if head_in_range {
                    continue;
                }

                let mut debug = true;
                if dir == 'U' || dir == 'D' {
                    tail_position[tail][x] = head_of_tail[x];
                    tail_position[tail][y] = tail_position[tail][y] + axis.get(&dir).unwrap();
                    debug = false;
                } else {
                    tail_position[tail][x] = tail_position[tail][x] + axis.get(&dir).unwrap();
                    tail_position[tail][y] = head_of_tail[y];
                    debug = false;
                }
                //println!("Tail before: {:?}", tail_positions.last().unwrap());

                tail_positions[tail].push(format!(
                    "{},{}",
                    tail_position[tail][x], tail_position[tail][y]
                ));
                //println!("Tail After: {:?}", tail_positions.last().unwrap());

                if debug {
                    println!("");
                    println!("-----------------------------");
                    println!(
                        "Error: expected to have updated tail position \n {:?}",
                        tail_positions[tail].last()
                    );
                    println!("-----------------------------");
                    println!("");
                }
            }
        }
    }

    // remove all duplicate elements.
    let tail = tail_positions[8].clone();
    let set: HashSet<String> 
    = tail.into_iter().collect();

    let vec: Vec<String> = set.into_iter().collect();
    println!("positions: {:?}", vec.len());
}
