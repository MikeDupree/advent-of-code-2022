use std::collections::{HashMap,HashSet};
use std::fs;

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
    let mut tail_positions = vec![String::from("0,0")];
    let mut tail_position = [0, 0];
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

            // Check for tail movement
            // xi :: -1
            // yi :: -1
            // yi :: 0
            // yi :: 1
            // xi :: 0
            // yi :: -1
            // yi :: 0
            // yi :: 1
            // xi :: 1
            // yi :: -1
            let mut head_in_range = false;
            for xi in -1..2 {
                //println!(" xi :: {}", xi);
                for yi in -1..2 {
                    //println!(" yi :: {}", yi);
                    if tail_position[x] + xi == head_position[x]
                        && tail_position[y] + yi == head_position[y]
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
                tail_position[x] = head_position[x];
                tail_position[y] = tail_position[y] + axis.get(&dir).unwrap();
                debug = false;
            } else {
                tail_position[x] = tail_position[x] + axis.get(&dir).unwrap();
                tail_position[y] = head_position[y];
                debug = false;
            }
            //println!("Tail before: {:?}", tail_positions.last().unwrap());

            tail_positions.push(format!("{},{}", tail_position[x], tail_position[y]));
            //println!("Tail After: {:?}", tail_positions.last().unwrap());

            if debug {
                println!("");
                println!("-----------------------------");
                println!(
                    "Error: expected to have updated tail position \n {:?}",
                    tail_positions.last()
                );
                println!("-----------------------------");
                println!("");
            }
        }
    }

    // remove all duplicate elements.
    let set: HashSet<String> = tail_positions
        .into_iter().collect();

    let vec: Vec<String> = set.into_iter().collect();
    println!("positions: {:?}", vec.len());
}
