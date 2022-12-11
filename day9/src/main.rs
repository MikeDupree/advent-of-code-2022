use std::collections::HashMap;
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
    let mut tail_positions = vec![[0, 0]];
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
                    if tail_positions.last().unwrap()[x] + xi == head_position[x]
                        && tail_positions.last().unwrap()[y] + yi == head_position[y]
                    {
                        head_in_range = true;
                    }
                }
            }

            // Head is in range. No tail movement.
            if head_in_range {
                continue;
            }

            let mut xx = 0;
            let mut yy = 0;
            let mut debug = true;
            if dir == 'U' || dir == 'D' {
                xx = head_position[x];
                yy = tail_positions.last().unwrap()[y] + axis.get(&dir).unwrap();
                debug = false;
            } else {
                xx = tail_positions.last().unwrap()[x] + axis.get(&dir).unwrap();
                yy = head_position[y]; 
                debug = false;
            }
            //println!("Tail before: {:?}", tail_positions.last().unwrap());
            let new_tail_pos = [xx, yy];

            tail_positions.push(new_tail_pos);
            //println!("Tail After: {:?}", tail_positions.last().unwrap());

            if debug {
                println!("");
                println!("-----------------------------");
                println!("Error: expected to have updated tail position \n {:?}", tail_positions.last());
                println!("-----------------------------");
                println!("");
            }
        }
    }
    println!("positions: {:?}", tail_positions.iter().unique());

}
