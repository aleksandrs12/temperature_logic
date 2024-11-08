use std::io;
use std::time::{Duration, Instant};

static mut DEBUG_COUNTER: i32 = 0;
fn get_current_temperature() -> i32 {
    //let mut input = String::new();
    //io::stdin().read_line(&mut input).expect("Failed to read input");
    //input.trim().parse().expect("Invalid temperature value")
    unsafe{
        DEBUG_COUNTER += 1;
    
    if DEBUG_COUNTER < 1000{
        return DEBUG_COUNTER;
    }
    println!("Temperature value: {}", ((f64::from(DEBUG_COUNTER) - 1000.0).sin() / (f64::from(DEBUG_COUNTER) - 1000.0) * 500.0 + 1000.0).round() as i32);
    return ((f64::from(DEBUG_COUNTER) - 1000.0).sin() / (f64::from(DEBUG_COUNTER) - 1000.0) * 500.0 + 1000.0).round() as i32;
}
}

fn find_average(list: &[f64]) -> f64 {
    list.iter().sum::<f64>() / list.len() as f64
}

fn find_amp(list: &[f64]) -> f64 {
    let max = list.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap();
    let min = list.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap();
    max - min
}


const MAX_AVERAGE_DIVERGENCE: f64 = 0.1;
const MAX_AMP: f64 = 0.2;
const MAX_TIME_REACH_TARGET: Duration = Duration::from_secs(10);
const MAX_TIME_STABILIZE: Duration = Duration::from_secs(10);
const HISTORY_SIZE: usize = 50;

fn set_temp(t: i32) -> i32 {
    let target_temperature: f64 = f64::from(t) / 10.0;
    let time_start = Instant::now();
    let mut target_temperature_reached = false;
    let mut historical_data: [f64; HISTORY_SIZE] = [0.0; HISTORY_SIZE];

    while !target_temperature_reached {
        let current_t: f64 = f64::from(get_current_temperature()) / 10.0;
        if current_t >= target_temperature {
            target_temperature_reached = true;
            break;
        }
        if time_start.elapsed() >= MAX_TIME_REACH_TARGET {
            return 0; // send NOT OK code
        }
    }

    println!("Temperature reached, now stabilizing");

    let time_start = Instant::now();
    for i in 0..HISTORY_SIZE {
        historical_data[i] = f64::from(get_current_temperature()) / 10.0;
    }

    loop {
        let current_t = f64::from(get_current_temperature()) / 10.0;
        historical_data.rotate_left(1);
        historical_data[HISTORY_SIZE - 1] = current_t;
        let amp = find_amp(&historical_data);
        let avg = find_average(&historical_data);

        if amp <= MAX_AMP && (avg - target_temperature).abs() < MAX_AVERAGE_DIVERGENCE {
            return 1; // send OK code
        }

        if time_start.elapsed() >= MAX_TIME_STABILIZE {
            return 0; // send NOT OK code
        }
    }
}

fn main() {
    println!("The return code is: {}", set_temp(1000)); // 100.0 C 
}