use std::cmp::Reverse;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let skip = [" ", "[", "]"];
    let mut stack1: Vec<char> = Vec::new();
    let mut stack2: Vec<char> = Vec::new();
    let mut stack3: Vec<char> = Vec::new();
    let mut stack4: Vec<char> = Vec::new();
    let mut stack5: Vec<char> = Vec::new();
    let mut stack6: Vec<char> = Vec::new();
    let mut stack7: Vec<char> = Vec::new();
    let mut stack8: Vec<char> = Vec::new();
    let mut stack9: Vec<char> = Vec::new();

    let mut stack_index = 0;
    let mut line_count = 0;
    let mut stacks_complete = false;
    for line in contents.lines() {

        if line_count == 8 {
            stacks_complete = true;
        }
        if !stacks_complete {
            for c in line.chars() {
                if !skip.contains(&c.to_string().as_str()) {
                    if stack_index == 2 {
                        stack1.push(c);
                    }
                    match stack_index {
                        1 => stack1.push(c),
                        5 => stack2.push(c),
                        9 => stack3.push(c),
                        13 => stack4.push(c),
                        17 => stack5.push(c),
                        21 => stack6.push(c),
                        25 => stack7.push(c),
                        29 => stack8.push(c),
                        33 => stack9.push(c),
                        _ => (),
                    }
                }
                stack_index += 1;
            }
        }

        stack_index = 0;
        if line_count == 9 {
            stack1.reverse();
            stack2.reverse();
            stack3.reverse();
            stack4.reverse();
            stack5.reverse();
            stack6.reverse();
            stack7.reverse();
            stack8.reverse();
            stack9.reverse();
        }

        if line_count > 9 {
            // read the instruction
            let inst: String = line.chars().filter(|c| c.is_digit(10)||c.is_whitespace()).collect();
            let numbers: Result<Vec<i32>, _> = inst.split_whitespace().map(str::parse).collect();

            // [amount to move, from, to]
            println!("{:?}", numbers.ok().unwrap());
        }

        line_count += 1;
    }

    println!("Total: {:?}", stack1);
    println!("Total: {:?}", stack2);
    println!("Total: {:?}", stack3);
    println!("Total: {:?}", stack4);
    println!("Total: {:?}", stack5);
    println!("Total: {:?}", stack6);
    println!("Total: {:?}", stack7);
    println!("Total: {:?}", stack8);
    println!("Total: {:?}", stack9);
}
