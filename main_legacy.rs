use std::io;
use std::time::{Duration, Instant};

static mut DEBUG_COUNTER: u16 = 0;
fn get_current_temperature() -> u16 {
    /*
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Invalid temperature value")
    */
    
    unsafe{
        DEBUG_COUNTER += 1;
        
        if DEBUG_COUNTER < 950{
            return DEBUG_COUNTER;
        }
        return ((f32::from(DEBUG_COUNTER) - 950.0).sin() * 2.5 + 950.0).round() as u16;
    }
    

}

fn find_average(list: &[f32]) -> f32 {
    list.iter().sum::<f32>() / list.len() as f32
}

fn find_amp(list: &[f32]) -> f32 {
    let max = list.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap();
    let min = list.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap();
    max - min
}


const MAX_AVERAGE_DIVERGENCE: f32 = 1.0;
const MAX_AMP: f32 = 0.5;
const MAX_TIME_REACH_TARGET: Duration = Duration::from_secs(10);
const MAX_TIME_STABILIZE: Duration = Duration::from_secs(10);
const HISTORY_SIZE: usize = 50;

fn set_temp(t: u16) -> u16 {
    let target_temperature: f32 = f32::from(t) / 10.0;
    let time_start = Instant::now();
    let mut target_temperature_reached = false;
    let mut historical_data: [f32; HISTORY_SIZE] = [0.0; HISTORY_SIZE];

    while !target_temperature_reached {
        let current_t: f32 = f32::from(get_current_temperature()) / 10.0;
        println!("Temperature value: {} C", current_t);
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
        historical_data[i] = f32::from(get_current_temperature()) / 10.0;
        println!("Temperature value: {} C", historical_data[i]);
    }

    loop {
        let current_t = f32::from(get_current_temperature()) / 10.0;
        historical_data.rotate_left(1);
        historical_data[HISTORY_SIZE - 1] = current_t;
        let amp = find_amp(&historical_data);
        let avg = find_average(&historical_data);
        println!("Temperature value: {} C,  Amp: {},  Avg: {}", current_t, amp, avg);

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