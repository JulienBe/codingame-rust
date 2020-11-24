use std::{io, fmt};
use std::iter::{FromIterator, Map};
use std::collections::HashMap;
use std::fmt::Formatter;
use std::borrow::Borrow;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w = parse_input!(inputs[0], i32);
    let h = parse_input!(inputs[1], i32);
    let mut results: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut conversion_table = HashMap::new();

    let mut input_first_line = String::new();
    io::stdin().read_line(&mut input_first_line).unwrap();
    let first_line = input_first_line.trim_end().to_string();
    let mut current_number = 1;
    for c in first_line.chars() {
        if c != ' ' {
            eprintln!("added: {} at {}", c, current_number);
            let mut numbers_on_this_column = Vec::new();
            numbers_on_this_column.push(current_number);
            results.insert(current_number, numbers_on_this_column);
            conversion_table.insert(current_number, c);
            current_number = current_number + 1;
        }
    }

    for i in 1..h as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim_end().to_string();
        let mut current_number = 0;
        for c in line.chars() {
            if c == '|' {
                current_number = current_number + 1;
            }
            if c == '-' {
                let opt_numbers_to_move_down = results.get(&(current_number + 1));
                results.insert(current_number + 1, results.get(&current_number).unwrap().into_vec());
                if let Some(ns) = opt_numbers_to_move_down {
                    results.insert(current_number, ns.into_vec());
                }
                eprintln!("current number: {}", current_number);
                eprintln!("numbers currently on column: {:?}", results.get(&current_number).unwrap());
            }
        }
    }

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    println!("A2");
}
