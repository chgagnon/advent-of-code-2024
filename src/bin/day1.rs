use std::fs;
use std::iter::zip;

fn main() {
    let file_path = "inputs/day1.txt";
    let contents: Vec<Vec<i32>> = fs::read_to_string(file_path)
        .expect("File read failed")
        .lines()
        .map(|s| s.split("   ").map(|n| n.parse::<i32>().unwrap()).collect())
        .collect();

    let mut left_column: Vec<i32> = contents.iter().map(|row| row[0]).collect();
    left_column.sort();
    let mut right_column: Vec<i32> = contents.iter().map(|row| row[1]).collect();
    right_column.sort();
    let sum = zip(left_column,right_column).fold(0, |acc, elem| (elem.0 - elem.1).abs() + acc);
    println!("The sum of the differences is {}", sum);

    

}
