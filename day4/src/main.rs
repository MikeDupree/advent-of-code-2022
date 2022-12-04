use std::fs;
// Day 3 
fn main() {

    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mut matches = 0;
    for line in contents.lines() {
        let pairs: Vec<&str> = line.split(",").collect();
        let pair1: Vec<&str> = pairs[0].split("-").collect();
        let pair2: Vec<&str> = pairs[1].split("-").collect();
        let p11 = pair1[0].parse::<u32>().unwrap();
        let p12 = pair1[1].parse::<u32>().unwrap();
        let p21 = pair2[0].parse::<u32>().unwrap();
        let p22 = pair2[1].parse::<u32>().unwrap();
        if p11 >= p21 && p12 <= p22 || p11 <= p21 && p12 >= p22 || p11 >= p22 && p12 <= p21 || p11 <= p22 && p12 >= p21 {
            matches += 1
        }
    }

    println!("Total: {}", matches);
}

