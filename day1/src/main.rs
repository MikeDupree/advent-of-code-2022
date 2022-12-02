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
    if let Ok(lines) = read_lines(INPUT_SRC) {
        let mut current_elfs_calories = 0;
        let mut most_calories_carried = 0;
        let mut second_most_calories_carried = 0;
        let mut third_most_calories_carried = 0;
        for line in lines {

            if let Ok(calories) = line {
                if calories.is_empty() {
                    if current_elfs_calories > most_calories_carried {
                        third_most_calories_carried = second_most_calories_carried;
                        second_most_calories_carried = most_calories_carried;
                        most_calories_carried = current_elfs_calories;
                    } else if current_elfs_calories > second_most_calories_carried {
                        third_most_calories_carried = second_most_calories_carried;
                        second_most_calories_carried = current_elfs_calories;
                    } else if current_elfs_calories > third_most_calories_carried {
                        third_most_calories_carried = current_elfs_calories;
                    }
                    current_elfs_calories = 0;
                    continue;
                }
                current_elfs_calories += calories.parse::<i32>().unwrap();
            }
        }
        println!("1st Most calories carried = {}", most_calories_carried);
        println!("2nd Most calories carried = {}", second_most_calories_carried);
        println!("3rd Most calories carried = {}", third_most_calories_carried);
        println!("Total: {}", most_calories_carried + second_most_calories_carried + third_most_calories_carried)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
