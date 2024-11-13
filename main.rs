use std::io;
use std::thread;
use std::time::{Duration, Instant};
use std::error::Error;
use std::result::Result;

static mut DEBUG_COUNTER: u16 = 0;
fn get_current_temperature() -> Result<u16, Box<dyn Error>> {
    // This function should not be in production and is only here to simulate readings from sensors
    unsafe{
        DEBUG_COUNTER += 1;
        if DEBUG_COUNTER == 1010{
            return Err("Something went wrong in the get_current_temperature function".into());
        }
        if DEBUG_COUNTER < 1000{
            return Ok(DEBUG_COUNTER);
        }
        return Ok(((f32::from(DEBUG_COUNTER) - 1000.0).sin() * 2.5 + 1000.0).round() as u16);
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
const HISTORY_SIZE: usize = 100;
const HISTORY_TIME: Duration = Duration::from_millis(1000);

fn set_temp(t: u16) -> Result<u16, Box<dyn Error>> {
    let target_temperature: f32 = f32::from(t) / 10.0;
    let time_start = Instant::now();
    let time_between_reads: Duration = HISTORY_TIME / (HISTORY_SIZE as u32);
    let mut target_temperature_reached = false;
    let mut historical_data: [f32; HISTORY_SIZE] = [0.0; HISTORY_SIZE];

    while !target_temperature_reached {
        let current_t: f32 = f32::from(get_current_temperature()?) / 10.0;
        println!("Temperature value: {} C", current_t);
        if current_t >= target_temperature {
            target_temperature_reached = true;
            break;
        }
        if time_start.elapsed() >= MAX_TIME_REACH_TARGET {
            return Err("The temperature never reached the target value".into()); // send NOT OK code
        }
    }

    println!("Temperature reached, now stabilizing");

    let time_start = Instant::now();
    for i in 0..HISTORY_SIZE {
        historical_data[i] = f32::from(get_current_temperature()?) / 10.0;
        println!("Temperature value: {} C", historical_data[i]);
        thread::sleep(time_between_reads);
    }

    loop {
        let current_t = f32::from(get_current_temperature()?) / 10.0;
        historical_data.rotate_left(1);
        historical_data[HISTORY_SIZE - 1] = current_t;
        let amp = find_amp(&historical_data);
        let avg = find_average(&historical_data);
        println!("Temperature value: {} C,  Amp: {},  Avg: {}", current_t, amp, avg);
        

        if amp <= MAX_AMP && (avg - target_temperature).abs() < MAX_AVERAGE_DIVERGENCE {
            return Ok(0); // send OK code
        }

        if time_start.elapsed() >= MAX_TIME_STABILIZE {
            return Err("The temperature vailed to stabilize".into()); // send NOT OK code
        }
        thread::sleep(time_between_reads);
    }
}

fn main() {
    let result: Result<u16, Box<dyn Error>> = set_temp(1000);

    if let Ok(value) = &result {
        println!("Success: {}", value);
    }

    if let Err(error) = &result {
        println!("Error: {}", error);
    }
    //println!("The return code is: {}", set_temp(1000).unwrap_or_default()); // 100.0 C 
}