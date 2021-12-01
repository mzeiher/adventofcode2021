use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    one();
    two();
}

fn one() {
    let file = File::open("./input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut counter: i32 = 0;
    let mut prev_value: i32 = 9999999;
    for line in reader.lines() {
        if let Ok(current_line) = line {
            let current_number = current_line.parse::<i32>().unwrap();
            if current_number > prev_value {
                counter += 1;
            }
            prev_value = current_number;
        }
    }

    println!("count {}", counter);
}

fn two() {
    let file = File::open("./input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let mut counter: i32 = 0;
    let mut prev_window_sum: i32 = 999999999;
    let mut sliding_window: [i32; 3] = [0, 0, 0];
    let mut loop_counter = 0;

    for line in reader.lines() {
        let current_line = line.unwrap();
        let current_number = current_line.parse::<i32>().unwrap();
        sliding_window[loop_counter % 3] = current_number;
        let current_window_sum = sliding_window.iter().fold(0, |sum, curr| sum + curr);
        if current_window_sum > prev_window_sum {
            counter += 1;
        }

        if loop_counter > 1 {
            prev_window_sum = current_window_sum;
        }
        loop_counter += 1;
    }

    println!("count {}", counter);
}
