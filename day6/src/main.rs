use std::{fs, str};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    println!("{}", find_first_marker(contents.as_str()));
}

fn find_first_marker(buffer: &str) -> i32 {
    // Keep track of the last four characters received
    let mut last_four = [' ', ' ', ' ', ' '];

    for (i, c) in buffer.chars().enumerate() {
        // Shift the last four characters over by one,
        // and add the new character to the end
        last_four.rotate_right(1);
        last_four[3] = c;

        // Check if all characters in the last four are different
        if i > 3
            && last_four[0] != last_four[1]
            && last_four[0] != last_four[2]
            && last_four[0] != last_four[3]
            && last_four[1] != last_four[2]
            && last_four[1] != last_four[3]
            && last_four[2] != last_four[3]
        {
            // If so, return the number of characters processed so far plus four
            return (i + 1) as i32;
        }
    }

    // If no marker was found, return -1
    -1
}
