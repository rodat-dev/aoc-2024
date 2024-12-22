use std::path::Path;

fn main() -> std::io::Result<()> {
    let file_contents = std::fs::read_to_string(Path::join(&std::env::current_dir()?, "input.txt"))?;
    let (left, right): (Vec<&str>, Vec<&str>) = file_contents.split("\n").into_iter().map(|line| line.split_ascii_whitespace().collect::<Vec<&str>>()).map(|pair| (pair[0], pair[1])).unzip();
    let mut left_ints = left.into_iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut right_ints = right.into_iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    left_ints.sort();
    right_ints.sort();

    let sum = left_ints.into_iter().zip(right_ints.into_iter()).map(|(l, r)| i32::abs(l - r)).sum::<i32>();
    println!("{}", sum);
    Ok(())
}
