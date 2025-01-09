use std::fs;
use std::collections::HashMap;
use std::iter::zip;

fn get_file_contents() -> Vec<Vec<i32>> {
    let file_path = "inputs/day1.txt";
    let contents: Vec<Vec<i32>> = fs::read_to_string(file_path)
        .expect("File read failed")
        .lines()
        .map(|s| s.split("   ").map(|n| n.parse::<i32>().unwrap()).collect())
        .collect();
    contents
}

fn get_part_1(left_column: &Vec<i32>, right_column: &Vec<i32>) -> i32 {
    let sum = zip(left_column, right_column).fold(0, |acc, elem| acc + (elem.0 - elem.1).abs());
    sum
}

fn get_part_2(left_column: &Vec<i32>, right_column: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in right_column {
        map.insert(*i, *map.get(i).unwrap_or(&0) + 1);
    }

    let similarity = left_column
        .iter()
        .fold(0, |acc, elem| acc + elem * *map.get(elem).unwrap_or(&0));
    similarity
}

fn main() {
    let contents = get_file_contents();
    let mut left_column: Vec<i32> = contents.iter().map(|row| row[0]).collect();
    left_column.sort();
    let mut right_column: Vec<i32> = contents.iter().map(|row| row[1]).collect();
    right_column.sort();

    let sum = get_part_1(&left_column, &right_column);
    println!("The sum of the differences is {}", sum);

    let similarity = get_part_2(&left_column, &right_column);
    println!("The similarity score is {}", similarity);
}
