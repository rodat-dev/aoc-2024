use std::{collections::HashMap, io, path::Path};

#[allow(dead_code)]
fn get_location_ids(left: &[i32], right: &[i32]) -> io::Result<i32> {
    let mut left_ints = left.to_vec();
    let mut right_ints = right.to_vec();
    left_ints.sort();
    right_ints.sort();

    let sum = left_ints.into_iter().zip(right_ints.into_iter()).map(|(l, r)| i32::abs(l - r)).sum::<i32>();
    Ok(sum)
}

fn calculate_similarity(left: &[i32], right: &[i32]) -> i32 {
    let mut left_ints = left.to_vec();
    let right_ints = right.to_vec();
    let mut map_ids = HashMap::<i32, usize>::new();
    for n in left_ints.clone() {
        for m in &right_ints {
            if n == *m {
                map_ids.entry(n).and_modify(|v| *v += 1).or_insert(1);
            }
        }

        // remove from left to minimize the number of iterations
        left_ints.remove(left_ints.iter().position(|x| *x == n).unwrap());
    }
    
    let sum = map_ids.into_iter().map(|(k, v)| k * v as i32).sum::<i32>();
    sum
}

fn split_left_and_right() -> io::Result<(Vec<i32>, Vec<i32>)> {
    let file_contents = std::fs::read_to_string(Path::join(&std::env::current_dir()?, "input.txt"))?;
    let (left, right): (Vec<&str>, Vec<&str>) = file_contents.split("\n").into_iter().map(|line| line.split_ascii_whitespace().collect::<Vec<&str>>()).map(|pair| (pair[0], pair[1])).unzip();
    let left_ints = left.into_iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let right_ints = right.into_iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    Ok((left_ints, right_ints))
}

fn main() -> std::io::Result<()> {
    let (left, right) = split_left_and_right()?;
    let similarity = calculate_similarity(&left, &right);

    println!("{}", similarity);
    Ok(())
}
