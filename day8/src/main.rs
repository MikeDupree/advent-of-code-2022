use std::{fs, str};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    find_the_tall_tall_trees_location(contents.as_str());
    find_the_tree_view_score(contents.as_str());
}

fn find_the_tall_tall_trees_location(c: &str) {
    let mut trees: Vec<Vec<u32>> = vec![];

    for row in c.lines() {
        trees.push(row.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    let mut visible_trees = 0;
    let mut visibleb_trees = 0;
    for (ri, row) in trees.iter().enumerate() {
        for (ti, tree) in trees[ri].iter().enumerate() {
            if ri == 0 || ri == row.len() - 1 || ti == 0 || ti == trees[ri].len() - 1 {
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
            // check down
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

fn find_the_tree_view_score(c: &str) {
    let mut trees: Vec<Vec<u32>> = vec![];

    for row in c.lines() {
        trees.push(row.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    let mut best_tree_score = 0;
    for (ri, row) in trees.iter().enumerate() {
        for (ti, tree) in trees[ri].iter().enumerate() {
            if ri == 0 || ri == row.len() - 1 || ti == 0 || ti == trees[ri].len() - 1 {
                continue;
            }

            let t = trees[ri].clone();
            let mut t_count = 0;
            let mut v_score;

            //println!("Tree: {} ({}:{}) count={}", tree, ri, ti, t_count);
            // check up
            for u in (0..ri).rev() {
                t_count += 1;
                //println!("  Up: {} ({}:{}) count={} ", &trees[u][ti], u, ti, t_count);
                if tree <= &trees[u][ti] {
                    break;
                }
            }
            v_score = t_count;
            t_count = 0;

            // check down
            for d in ri + 1..trees.len() {
                t_count += 1;
                //println!("  Down: {} ({}:{}) count={}", &trees[d][ti], d, ti, t_count);
                if tree <= &trees[d][ti] {
                    break;
                }
            }
            v_score *= t_count;
            t_count = 0;

            //check left
            for l in (0..ti).rev() {

                t_count += 1;
                //println!("  Left: {} ({}:{}) count={}", &t[l], ri, l, t_count);
                if tree <= &t[l] {
                    break;
                }
            }
            v_score *= t_count;
            t_count = 0;

            //check right
            for r in ti + 1..trees[ri].len() {
                t_count += 1;
                //println!("  Right: {} ({}:{}) count={}", &t[r], ri, r, t_count);
                if tree <= &t[r] {
                    break;
                }
            }

            v_score *= t_count;
            println!("Score: {}", v_score);
            if v_score > best_tree_score {
                best_tree_score = v_score;
            }
        }
    }

    println!("Best Tree View Score: {}", best_tree_score);
}
