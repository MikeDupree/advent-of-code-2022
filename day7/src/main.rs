use std::{fs, str};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    find_dir_totals_v1(contents.as_str());
    find_dir_to_delete(contents.as_str());
}

fn find_dir_totals_v1(c: &str) {
    let mut dirs = vec![(String::from("/"), 0)];

    let mut dir_size_total = 0;
    for line in c.lines() {
        if line.contains("$ cd") && !line.contains("..") && !line.contains("/") {
            dirs.push((line.to_string().replace("$ cd ", ""), 0));
        }

        if line.contains("..") {
            let dir = dirs.pop().unwrap();
            if dir.1 <= 100_000 {
                dir_size_total += dir.1;
            }
            dirs.last_mut().unwrap().1 += dir.1;
        }

        let output: String = line.chars().filter(|c| c.is_numeric()).collect();
        if !output.is_empty() {
            let size = output.parse::<u32>().ok().unwrap();
            dirs.last_mut().unwrap().1 += size;
        }
    }

    println!("Total size of dirs under 100,000 = {}", dir_size_total);
}

fn find_dir_to_delete(c: &str) {
    let total_filesystem_space = 70_000_000;
    let space_req_for_update = 30_000_000;
    let mut total_space_used = 0;

    for line in c.lines() {
        let output: String = line.chars().filter(|c| c.is_numeric()).collect();
        if !output.is_empty() {
            let size = output.parse::<u32>().ok().unwrap();
            total_space_used += size;
        }
    }

    let space_left = total_filesystem_space - total_space_used;
    let space_req = space_req_for_update - space_left;
    println!("total_space_used: {}", total_space_used);
    println!("total space left: {}", space_left);
    println!("total space req: {}", space_req);

    let mut dirs = vec![(String::from("/"), 0)];

    let mut selected = total_filesystem_space;
    let mut selected_dir = String::from("");
    for line in c.lines() {
        if line.contains("$ cd") && !line.contains("..") && !line.contains("/") {
            dirs.push((line.to_string().replace("$ cd ", ""), 0));
        }

        if line.contains("..") {
            let dir = dirs.pop().unwrap();

            if dir.1 >= space_req && dir.1 < selected {
                selected = dir.1;
                selected_dir = dir.0.clone();
            }
            dirs.last_mut().unwrap().1 += dir.1;
        }

        let output: String = line.chars().filter(|c| c.is_numeric()).collect();
        if !output.is_empty() {
            let size = output.parse::<u32>().ok().unwrap();
            dirs.last_mut().unwrap().1 += size;
        }
    }

    for _i in 0..dirs.len() {
        if dirs.len() < 1 {
            break;
        }
        let dspace = dirs.pop().unwrap();
        if dspace.1 >= space_req {
            if dspace.1 < selected {
                selected = dspace.1;
                selected_dir = dspace.0.clone();
            }
        }
        if dirs.len() > 0 {
            dirs.last_mut().unwrap().1 += dspace.1;
        }
    }

    println!("dir to delete {}", selected_dir);
}
