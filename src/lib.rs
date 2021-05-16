pub mod mine_sweeper;

use std::env::args;
use std::time::Duration;

pub fn get_args() -> Result<(usize, usize, f64), &'static str> {
    if args().len() != 4 {
        return Err("wrong number of arguments");
    }
    let mut col: usize = 0;
    let mut row: usize = 0;
    let mut chance: f64 = 0.0;
    for (i, arg) in args().enumerate() {
        match i {
            0 => continue,
            1 => col = match arg.parse() {
                Ok(v) => {v},
                Err(_) => {return Err("faild to convert col")},
            },
            2 => row = match arg.parse() {
                Ok(v) => {v},
                Err(_) => {return Err("failed to convert row")},
            },
            3 => chance = match arg.parse() {
                Ok(v) => {v},
                Err(_) => {return Err("Failed to convert chance")},
            },
            _ => return Err("unknown error")
            
        }
    }
    return Ok((col, row, chance))
}


pub fn average_time_per_step(times: &Vec<(Duration, usize)>) -> Option<Duration> {
    if times.len() < 1 {return None;}
    let mut sum: Duration = Duration::from_nanos(0);
    for (time, steps) in times {
        sum += *time / *steps as u32;
    }   
    return Some(sum / times.len() as u32)
}