use std::{fs::File, io, io::BufRead};

fn main() {
    one();
    two();
}

fn one() {
    let file = File::open("./input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut x_pos = 0;
    let mut depth = 0;

    for line in reader.lines() {
        if let Ok(current_line) = line {
            let strings: Vec<&str> = current_line.split(" ").collect();
            let command = strings[0];
            let value = strings[1].parse::<i32>().unwrap();
            match command {
                "forward" => x_pos += value,
                "down" => depth += value,
                "up" => depth -= value,
                _ => {}
            }
        }
    }

    println!("xpos {}, depth {}, xpos*depth {}", x_pos, depth, x_pos*depth);
}

fn two() {
    let file = File::open("./input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut x_pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in reader.lines() {
        if let Ok(current_line) = line {
            let strings: Vec<&str> = current_line.split(" ").collect();
            let command = strings[0];
            let value = strings[1].parse::<i32>().unwrap();
            match command {
                "forward" => {
                    x_pos += value;
                    depth += value * aim;
                },
                "down" => aim += value,
                "up" => aim -= value,
                _ => {}
            }
        }
    }

    println!("xpos {}, depth {}, xpos*depth {}", x_pos, depth, x_pos*depth);
}
