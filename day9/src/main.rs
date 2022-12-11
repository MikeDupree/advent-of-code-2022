use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    follow_head(&contents.as_str());
}

fn follow_head(c: &str) {
    let moves = HashMap::from([
        ('U', ["y", "1"]),
        ('D', ["y", "-1"]),
        ('R', ["x", "1"]),
        ('L', ["x", "-1"]),
    ]);
    let mut tail_moves = 0;
    let mut tpos_x = "0";
    let mut tpos_y = "0";

    for line in c.lines() {
        //  print!("{} ", line);
        let mut chars = line.chars().collect::<Vec<char>>();
        let m = chars[0];
        chars.drain(0..2);
        let n: u32 = chars
            .into_iter()
            .collect::<String>()
            .as_str()
            .parse::<u32>()
            .ok()
            .unwrap();
        let mc = moves.get(&m).unwrap();

        //print!("m {} ", m);
        //print!("n {} \n", n);
        //println!("move coords {:?}", mc);

        // moved one space check tail pos to see if it moves
        if n == 1 {
            // they are overlapping
            if tpos_x == "0" && tpos_y == "0" {
                // move tail pos
                if mc[0] == "y" {
                    if mc[1] == "-1" {
                        tpos_y = "1"
                    } else {
                        tpos_y = "-1"
                    }
                }
                if mc[0] == "x" {
                    if mc[1] == "-1" {
                        tpos_x = "1"
                    } else {
                        tpos_x = "-1"
                    }
                }
                continue;
            }

            // handle when movement causes overlap
            // single movement of head that overlaps tail
            // does not trigger movement
            if mc[0] == "y" && mc[1] == tpos_y && tpos_x == "0"
                || mc[0] == "x" && mc[1] == tpos_x && tpos_y == "0"
            {
                tpos_y = "0";
                tpos_x = "0";
                continue;
            }

            // tail is adjacent and move causes tail pos to become diagonal
            if mc[0] == "y" && tpos_y == "0" && tpos_x != "0" {
                if mc[1] == "1" {
                    tpos_y = "-1";
                } else {
                    tpos_y = "1";
                }
            }

            if mc[0] == "x" && tpos_x == "0" && tpos_y != "0" {
                if mc[1] == "1" {
                    tpos_x = "-1";
                } else {
                    tpos_x = "1";
                }
            }

            // tail is diagonal but there is no tail movement
            // update the tail pos for one coord
            if mc[0] == "y" && mc[1] == tpos_y && tpos_x != "0" {
                tpos_y = "0";
                continue;
            } else if mc[0] == "x" && mc[1] == tpos_x && tpos_y != "0" {
                tpos_x = "0";
                continue;
            }
            // single movement that moves the tail
            if mc[0] == "y" && mc[1] == "1" && tpos_y == "-1" {
                tail_moves += 1;
                tpos_x = "0"; // x is centered because of move
                continue;
            } else if mc[0] == "y" && mc[1] == "-1" && tpos_y == "1" {
                tpos_y = "1";
                tpos_x = "0";
                continue;
            }

            if mc[0] == "x" && mc[1] == "1" && tpos_x == "-1" {
                tail_moves += 1;
                tpos_y = "0";
                continue;
            } else if mc[0] == "x" && mc[1] == "-1" {
                tail_moves += 1;
                tpos_y = "0";
                continue;
            }

            println!("");
            println!("");
            println!("Error: single move with no case logic");
            println!("tail ({}, {})  move {:?}", tpos_x, tpos_y, mc);
            println!("");
            println!("");
            continue;
        }

        // movement is greater then one space
        let tmb = tail_moves;
        let mut move_mod = 0;
        println!("\n --- \n");
        if mc[0] == "y" {
            // check tail loc relative to move
            if tpos_y == "0" {
                move_mod = 1;
                tail_moves += n - move_mod;
            } else if tpos_y == mc[1] {
                move_mod = 2;
                tail_moves += n - move_mod;
            } else {
                move_mod = 0;
                tail_moves += n - move_mod;
            }

            if mc[1] == "-1" {
                tpos_y = "1";
                tpos_x = "0";
            } else {
                tpos_y = "-1";
                tpos_x = "0";
            }
        }
        if mc[0] == "x" {
            println!("handle x move");
            // check tail loc relative to move
            if tpos_x == "0" {
                move_mod = 1;
                tail_moves += n - move_mod;
                print!(" x tpos == 0 ");
            } else if tpos_x == mc[1] {
                move_mod = 2;
                tail_moves += n - move_mod;
                print!(" x tpos == move dir ");
            } else {
                print!(" x else ");
                move_mod = 0;
                tail_moves += n - move_mod;
            }

            if mc[1] == "-1" {
                tpos_x = "1";
                tpos_y = "0";
            } else {
                tpos_x = "-1";
                tpos_y = "0";
            }
        }
        println!("tail moves before {} after {}", tmb, tail_moves);
        if tail_moves != tmb {
            // use this to debug,
            // if the print lines at the end are hit
            // then there is a problem
            // TODO this check doesnt work cause some moves might not move tail...
            continue;
        }

        println!("");
        println!("");
        println!("Error: multi move with no case logic");
        println!("tail ({}, {})  move {:?}", tpos_x, tpos_y, mc);
        println!("");
        println!("");
        //println!("");
    }

    println!("Tail Moves: {} times", tail_moves);
}
