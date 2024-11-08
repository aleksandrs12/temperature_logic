use std::io;
use std::time::{Duration, Instant};

fn get_current_temperature() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Invalid temperature value")
}

fn find_average(list: &[i32]) -> f64 {
    list.iter().sum::<i32>() as f64 / list.len() as f64
}

fn find_amp(list: &[i32]) -> f64 {
    let max = list.iter().max().unwrap();
    let min = list.iter().min().unwrap();
    *max as f64 / *min as f64
}

fn find_freq(list: &[i32], base: i32) -> i32 {
    let mut output = 0;
    for i in 1..list.len() {
        if list[i] == base && list[i - 1] != base {
            output += 1;
        }
    }
    output
}

const MAX_AVERAGE_DIVERGENCE: f64 = 0.1;
const MAX_AMP: f64 = 1.0;
const MAX_TIME_REACH_TARGET: Duration = Duration::from_secs(10);
const MAX_TIME_STABILIZE: Duration = Duration::from_secs(10);

fn set_temp(t: i32) -> i32 {
    let time_start = Instant::now();
    let mut target_temperature_reached = false;
    let history_size = 10;
    let mut historical_data = [0; history_size];

    while !target_temperature_reached {
        let current_t = get_current_temperature();
        if current_t >= t {
            target_temperature_reached = true;
            break;
        }
        if time_start.elapsed() >= MAX_TIME_REACH_TARGET {
            return 0; // send NOT OK code
        }
    }

    println!("Temperature reached, now stabilizing");

    let time_start = Instant::now();
    for i in 0..history_size {
        historical_data[i] = get_current_temperature();
    }

    loop {
        let current_t = get_current_temperature();
        historical_data.rotate_left(1);
        historical_data[history_size - 1] = current_t;
        let amp = find_amp(&historical_data);
        let avg = find_average(&historical_data);

        if amp <= MAX_AMP && (avg - t).abs() < MAX_AVERAGE_DIVERGENCE {
            return 1; // send OK code
        }

        if time_start.elapsed() >= MAX_TIME_STABILIZE {
            return 0; // send NOT OK code
        }
    }
}

fn main() {
    set_temp(100);
}