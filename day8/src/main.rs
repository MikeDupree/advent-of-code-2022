use std::{fs, str};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    find_tree_house_location(contents.as_str());
}

fn find_tree_house_location(c: &str) {
    let mut trees: Vec<Vec<u32>> = vec![];

    for row in c.lines() {
        trees.push(row.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    let mut visible_trees = 0;
    let mut visibleb_trees = 0;
    for (ri, row) in trees.iter().enumerate() {
        for (ti, tree) in trees[ri].iter().enumerate() {
            //println!("ri {} ti {}", ri, ti);

            if ri == 0 || ri == row.len() - 1 || ti == 0 || ti == trees[ri].len() - 1 {
                //visible_trees += 1;
                visibleb_trees += 1;
                continue;
            }
            let mut vis: Vec<Vec<bool>> = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new()];

            let t = trees[ri].clone();
            //check left
            for l in 0..ti {
                vis[0].push(tree > &t[l]);
            }
            //check right
            for r in ti + 1..trees[ri].len() {
                vis[1].push(tree > &t[r]);
            }
            // check up
            for u in 0..ri {
                vis[2].push(tree > &trees[u][ti]);
            }
            for d in ri + 1..trees.len() {
                vis[3].push(tree > &trees[d][ti]);
            }
            

            if vis[0].clone().into_iter().all(|t| t)
                || vis[1].clone().into_iter().all(|t| t)
                || vis[2].clone().into_iter().all(|t| t)
                || vis[3].clone().into_iter().all(|t| t)
            {
                visible_trees += 1;
            }
        }
    }
    println!("visiblei_trees: {}", visible_trees);
    println!("visibleb_trees: {}", visibleb_trees);
    println!("visible_trees: {}", visible_trees + visibleb_trees);
}
