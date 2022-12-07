use std::{fs, str};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    println!("{}", find_first_marker(contents.as_str()));
    println!("{}", find_start_marker(contents.as_str()));
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
        if i > 3 && is_unique(&last_four) {
            // If so, return the number of characters processed so far plus four
            return (i + 1) as i32;
        }
    }

    // If no marker was found, return -1
    -1
}

fn find_start_marker(buffer: &str) -> i32 {
    // Keep track of the last four characters received
    let mut m = [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    ];

    for (i, c) in buffer.chars().enumerate() {
        // Shift the last fourteen characters over by one,
        // and add the new character to the end
        m.rotate_right(1);
        m[3] = c;

        // check if all characters are unique
        if i > 13 && is_unique(&m) {
            // If so, return the number of characters processed so far plus four
            return (i + 1) as i32;
        }
    }

    // If no marker was found, return -1
    -1
}

fn is_unique(arr: &[char]) -> bool {
    // create a mutable hash set
    let mut set = std::collections::HashSet::new();
    // iterate over the elements in the array and return false if any element is not unique
    arr.iter().all(|elem| set.insert(elem))
}
