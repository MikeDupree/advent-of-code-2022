use std::fs::File;
use std::io::{self, BufRead};
use std::panic;
use std::path::Path;

const INPUT_SRC: &str = "input.txt";

fn main() {
    // Create a panic hook that does nothing to supress panics in checks below.
    panic::set_hook(Box::new(|_info| {}));
    println!("Day 1 :: Elf Calorie Counter");

    let path = Path::new(INPUT_SRC);
    let display = path.display();

    // Check if file exists
    match File::open(&path) {
        Err(why) => panic!("Could'nt open {} : {} \n", display, why),
        Ok(file) => file,
    };

    // Sum up each line, add total to Vec, new index on empty line
    let mut elf_calories: Vec<u32> = Vec::new();
    if let Ok(lines) = read_lines(INPUT_SRC) {
        let mut index = 0;
        let mut most_calories_carried = 0;
        for line in lines {
            if let Ok(calories) = line {
                if calories.is_empty() {
                    if elf_calories[index] > most_calories_carried {
                        most_calories_carried = elf_calories[index]
                    }
                    index += 1;
                    continue;
                }
                let calories_int = calories.parse().unwrap();
                if panic::catch_unwind(|| elf_calories[index]).is_ok() {
                    elf_calories[index] += calories_int;
                } else {
                    elf_calories.push(calories_int);
                }
            }
        }
        println!("Most calories carried = {}", most_calories_carried)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
