use std::cmp::Reverse;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let skip = [" ", "[", "]"];
    let mut stack = vec![
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];

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
                    match stack_index {
                        1 => stack[0].push(c),
                        5 => stack[1].push(c),
                        9 => stack[2].push(c),
                        13 => stack[3].push(c),
                        17 => stack[4].push(c),
                        21 => stack[5].push(c),
                        25 => stack[6].push(c),
                        29 => stack[7].push(c),
                        33 => stack[8].push(c),
                        _ => (),
                    }
                }
                stack_index += 1;
            }
        }

        stack_index = 0;
        if line_count == 9 {
            for n in 0..9 {
                stack[n].reverse();
                println!("stack {} {:?}", n, stack[n]);
            }
        }

        if line_count > 9 {
            // read the instruction
            let inst: String = line
                .chars()
                .filter(|c| c.is_digit(10) || c.is_whitespace())
                .collect();
            let result: Result<Vec<i32>, _> = inst.split_whitespace().map(str::parse).collect();
            let instructions = result.ok().unwrap();
            // [amount to move, from, to]
            let amount: i32 = instructions[0];
            let from: i32 = instructions[1] - 1;
            let to: i32 = instructions[2] - 1;
            let mut holder: Vec<char> = Vec::new();

            for _i in 0..amount {
                let item = stack[from as usize].pop();
                holder.push(item.unwrap());
            }

            for _i in 0..amount {
                let item = holder.pop();
                println!("{}", item.unwrap());
                stack[to as usize].push(item.unwrap());
            }
        }

        line_count += 1;
    }
    for n in 0..9 {
        println!("stack {} {:?}", n, stack[n]);
    }
}
