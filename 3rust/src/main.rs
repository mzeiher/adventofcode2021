use std::{convert::TryInto, fs::File, io, io::BufRead};

fn main() {
    one()
}

fn one() {
    let file = File::open("./input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut counters = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut counter = 0;

    for line in reader.lines() {
        if let Ok(current_line) = line {
            let mut char_iterator = current_line.chars();
            for i in 0..12 {
                if char_iterator.nth(0) == Some('1') {
                    counters[i] += 1;
                }
            }
            counter += 1;
        }
    }

    let mut epsilon_rate = 0;
    let mut gamma_rate: i32 = 0;
    for i in 0..12 {
        // made in one loop, not using iterator by choice :)
        if counters[i] > (counter / 2) {
            gamma_rate += 2_i32.pow((11 - i).try_into().unwrap());
        } else {
            epsilon_rate += 2_i32.pow((11 - i).try_into().unwrap());
        }
    }
    println!(
        "rows {}, gamma {}, epsilon {}, combined: {}",
        counter,
        gamma_rate,
        epsilon_rate,
        gamma_rate * epsilon_rate
    )
}

fn two() {
    let file = File::open("./input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut counters = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut counter = 0;

    for line in reader.lines() {
        if let Ok(current_line) = line {
            let mut char_iterator = current_line.chars();
            for i in 0..12 {
                if char_iterator.nth(0) == Some('1') {
                    counters[i] += 1;
                }
            }
            counter += 1;
        }
    }

    // todo
}
