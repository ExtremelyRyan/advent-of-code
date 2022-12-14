// --- Day 8: Treetop Tree House ---
#![allow(dead_code)]
use std::{fs::read_to_string, path::Path};

fn main() {
    let s = read_input("./input/day8_input.txt");
    let _r = parse(s.clone());

    let part_one = part_one(s);
    println!("part one: {part_one}");
}

fn read_input<T: AsRef<Path>>(path: T) -> Vec<Vec<u8>> {
    let s = read_to_string(path).expect("Can't open/read file!");
    let mut vv: Vec<Vec<u8>> = Vec::new();

    for l in s.lines() {
        let mut v: Vec<u8> = Vec::new();
        for c in l.chars() {
            v.push(c.to_digit(10).unwrap() as u8);
        }
        vv.push(v);
    }
    //println!("{:?}", vv);
    vv
}

fn parse(input: Vec<Vec<u8>>) -> Vec<Vec<bool>> {
    let mut visable: Vec<Vec<bool>> = Vec::new();
    let mut index = 0;

    loop {
        let mut v: Vec<bool> = Vec::new();
        //println!("input[{index}]: {:?}, with a len of {}", input[index], input[index].len());

        for (i, num) in input[index].iter().enumerate() {
            //println!("index: {index}, i: {i}, num: {num}");
            //println!("line: {line}");

            // first & last number in column +  first row & last row are always visable
            if i == 0
                || i == input[index].len() - 1
                || index == 0
                || index == input[index].len() - 1
            {
                v.push(true);
            } else if input[index][i - 1] < *num   // left
                || input[index - 1][i] < *num    // down
                || input[index + 1][i] < *num    // up
                || input[index][i + 1] < *num
            // right
            {
                v.push(true);
            } else {
                v.push(false);
                //println!("num: {num}, pushed false!");
            }
        }
        //println!("v: {v:?}\n");
        visable.push(v);

        if index == input[index].len() - 1 {
            // println!("DEBUG: break condition");
            break;
        }

        index += 1;
    }

    //println!("visable: {visable:?}");
    visable
}

fn is_hidden(input: &Vec<Vec<u8>>, current_coordinates: (usize, usize)) -> bool {
    let (row, col) = current_coordinates;
    let max = input[row][col];

    // return value
    let mut visable = false;

    // check left
    for l in 0..col {
        if max <= input[row][l] {
            visable = true;
        }
    }
    // check right
    for r in col + 1..input[row].len() {
        if input[row][r] >= max {
            visable = true;
        }
    }
    // check up
    for u in 0..row {
        if input[u][col] >= max {
            visable = true;
        }
    }
    // check down
    for d in row + 1..input.len() {
        if max <= input[d][col] {
            visable = true;
        }
    }
    visable
}
// returns true if coordinates is hidden, false if visable

// how many trees are visible (true) from outside the grid?
fn part_one(input: Vec<Vec<u8>>) -> usize {
    let mut visable = 0;
    let v = input.clone();

    for (i, r) in input.clone().iter().enumerate() {
        for (j, _c) in r.iter().enumerate() {
            // if !is_hidden_in_column(&v, (i,j)) || !is_hidden_in_row(&v, (i,j)) {
            if !is_hidden(&v, (i, j)) {
                visable += 1;
            }
        }
    }
    visable
}

// how many trees are visible (true) from outside the grid?
fn part_two(input: Vec<Vec<u8>>) -> usize {
    let v = input.clone();
    let mut max = 0;

    for (i, r) in input.clone().iter().enumerate() {
        for (j, _c) in r.iter().enumerate() {
            //println!("dbg: [{i}][{j}]: {c}");
            let temp = check_col_scenery(&v, (i, j)) * check_row_scenery(&v, (i, j));
            //treehouse_view[i][j] = temp;
            if temp > max {
                max = temp;
                //max = treehouse_view[i][j];
            }
            //println!("max: {max}\n\n");
        }
    }
    max
}

fn part2(file: &str) -> usize {
    let trees = std::fs::read_to_string(file)
        .expect("couldn't read file")
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse().unwrap_or_default())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut highest = 0;
    for x in 1..trees.len() - 1 {
        for y in 1..trees[x].len() - 1 {
            let mut score = [0, 0, 0, 0];
            for i in (0..x).rev() {
                score[0] += 1;
                if trees[i][y] >= trees[x][y] {
                    break;
                }
            }
            for i in x + 1..trees.len() {
                score[1] += 1;
                if trees[i][y] >= trees[x][y] {
                    break;
                }
            }
            for i in (0..y).rev() {
                score[2] += 1;
                if trees[x][i] >= trees[x][y] {
                    break;
                }
            }
            for i in y + 1..trees[x].len() {
                score[3] += 1;
                if trees[x][i] >= trees[x][y] {
                    break;
                }
            }
            let score = score[0] * score[1] * score[2] * score[3];
            if score > highest {
                highest = score;
            }
        }
    }
    highest
}

fn check_col_scenery(input: &Vec<Vec<u8>>, current_coordinates: (usize, usize)) -> usize {
    let (row, col) = current_coordinates;
    // comparison target
    let max = input[row][col];
    // return value
    let mut view_up = 0;
    let mut view_down = 0;
    // check up
    for u in 0..row {
        //println!("up[{u}][{col}] = {}", input[u][col]);
        if input[u][col] >= max {
            view_up = u.abs_diff(row);
            break;
        }
    }
    // check down
    for (d, _item) in input.iter().enumerate().skip(row + 1) {
        //println!("dn[{d}][{col}] = {}", input[d][col]);
        if max <= input[d][col] {
            view_down = d.abs_diff(row);
            break;
        }
    }
    //println!("view_up: {view_up}, view_down: {view_down}, result: {}",  view_up * view_down);
    view_up * view_down
}
#[warn(dead_code)]
fn check_row_scenery(input: &[Vec<u8>], current_coordinates: (usize, usize)) -> usize {
    let (row, col) = current_coordinates;
    // comparison target
    let max = input[row][col];
    // return value
    let mut view_left = 0;
    let mut view_right = 0;

    // check left
    for l in 0..col {
        // println!("left: [{row}][{l}] = {}", input[row][l]);
        if max <= input[row][l] {
            view_left = l.abs_diff(col);
            break;
        }
    }
    // check right
    for r in col + 1..input[row].len() {
        // println!("right: [{row}][{r}] = {}", input[row][r]);
        if input[row][r] >= max {
            view_right = r.abs_diff(col);
            break;
        }
    }
    //println!("view_left: {view_left}, view_right: {view_right}, result: {}",  view_left * view_right);
    view_left * view_right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let s = read_input("./input/day8_sample.txt");

        let test_col_1 = is_hidden(&s.clone(), (3, 3));
        assert_eq!(test_col_1, true);
        let test_col_2 = is_hidden(&s.clone(), (3, 2));
        assert_eq!(test_col_2, true);
        let test_col_3 = is_hidden(&s.clone(), (0, 0));
        assert_eq!(test_col_3, false);
        let test_col_4 = is_hidden(&s.clone(), (4, 4));
        assert_eq!(test_col_4, false);

        let test_row_1 = is_hidden(&s.clone(), (3, 3));
        assert_eq!(test_row_1, true);
        let test_row_2 = is_hidden(&s.clone(), (3, 2));
        assert_eq!(test_row_2, false);
        let test_row_3 = is_hidden(&s.clone(), (4, 4));
        assert_eq!(test_row_3, false);
    }

    #[test]
    fn test_part_one_samples() {
        let s = read_input("./input/day8_sample.txt");
        let result = part_one(s);
        println!("sample result: {result}");
    }

    #[test]
    fn test_part_one() {
        let s = read_input("./input/day8_input.txt");
        let result = part_one(s);
        println!("sample result: {result}");
    }

    #[test]
    fn test_part_two_sample() {
        let s = read_input("./input/day8_sample.txt");
        let result = part_two(s);
        println!("sample result: {result}");

        // let s = read_input("input.txt");
        // let result = part_two(s);
        // println!("input result: {result}");
    }

    #[test]
    fn tst() {
        let num: i32 = 10;
        assert_eq!(5, num.abs_diff(5));
    }
}
