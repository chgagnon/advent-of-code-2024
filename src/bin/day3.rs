use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::Chars;

fn add_mul_result_to_sum(sum: &mut u32, line_chars: &mut Chars<'_>, op1_init: u32) {
    let mut op1 = op1_init;
    let mut op2 = 0;
    let mut on_second_op = false;
    while let Some(c) = line_chars.next() {
        println!("Current char in parens is {}", c);
        match c {
            ',' => {
                on_second_op = true;
            },
            ')' => {
                // If first and second op are completed, then multiply them
                if on_second_op {
                    *sum += op1 * op2;
                    println!("Op1 is {}", op1);
                    println!("Op2 is {}", op2);
                    println!("Product is {}", op1 * op2);
                    println!("Sum is now {}", sum);
                }
                // Do not continue iterating after the close paren
                return;
            }
            _ => {
                if let Some(n) = c.to_digit(10) {
                    if on_second_op {
                        op2 = op2 * 10 + n;
                    } else {
                        op1 = op1 * 10 + n;
                    }
                } else {
                    // Not a digit, or a comma, or a close paren -> return
                    return;
                }
            }
        }
    }
}

fn increment_last_valid_or_set_to_none_if_invalid(last_valid: &mut Option<char>, incoming: char) {
    if incoming == 'm'
        || (*last_valid == Some('m') && incoming == 'u')
        || (*last_valid == Some('u') && incoming == 'l')
        || (*last_valid == Some('l') && incoming == '(')
    {
        *last_valid = Some(incoming);
    } else {
        *last_valid = None;
    }
}

fn get_sum_for_line(line: &str) -> u32 {
    let mut sum = 0;
    let mut last_valid_char = None;
    let mut line_chars = line.chars();

    while let Some(curr_char) = line_chars.next() {
        match curr_char {
            incoming @ ('m' | 'u' | 'l' | '(') => {
                increment_last_valid_or_set_to_none_if_invalid(&mut last_valid_char, incoming)
            },
            _ => {
                if last_valid_char == Some('(') {
                    if let Some(n) = curr_char.to_digit(10) {
                        add_mul_result_to_sum(&mut sum, &mut line_chars, n);
                    }
                } else {
                    last_valid_char = None;
                }
            }
        }
    }
    sum
}

fn get_sum_of_mults() -> u32 {
    let file_path = "inputs/day3.txt";
    let file = File::open(file_path).expect("File read failed");
    let reader = BufReader::new(file).lines();

    let sum = reader.fold(0, |acc, line| {
        // Assume (for now) that no valid mul instructions span multiple lines
        let line_sum = get_sum_for_line(&line.unwrap());
        acc + line_sum
    });
    sum
}

fn main() {
    let sum_of_mults = get_sum_of_mults();
    println!("Part 1: The final sum is {}", sum_of_mults);
}
