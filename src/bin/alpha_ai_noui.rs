extern crate mine_sweeper;
mod alpha_ai;

use time::Duration;
use std::{time::SystemTime};
use mine_sweeper::{*, mine_sweeper::*};
use std::time;
use alpha_ai::*;

fn main() {
    let mut mine_sweeper: MineSweeper;
    if let Ok((cols, rows, chance)) = get_args() {
        mine_sweeper = MineSweeper::new(cols, rows, chance, ApperanceSettings::default());
    }
    else {
        mine_sweeper = MineSweeper::default();
    }
    let mut wins: usize = 0;
    let mut losses: usize = 0;
    let mut times: Vec<(Duration, usize)> = Vec::with_capacity(1000);
    let mut alpha_ai: AlphaAI = AlphaAI::new(mine_sweeper.cols(), mine_sweeper.rows());

    loop {
        match mine_sweeper.game_state {
            GameState::Running => {
                //let time = SystemTime::now();
                alpha_ai.update_ai(&mut mine_sweeper); 
                //println!("step: {:?}, took {:?} us", alpha_ai.step, SystemTime::now().duration_since(time).unwrap().as_micros());
                alpha_ai.step += 1;

            }
            GameState::Won => {
                wins += 1; 
                let time = SystemTime::now().duration_since(mine_sweeper.start_time).unwrap();
                times.push((time , alpha_ai.step));
                restart_no_ui(&mut alpha_ai, &mut mine_sweeper, &wins, &losses, &mut times);
                println!("Time: {:?} ", time.as_micros());
            }
            GameState::Lost => {
                losses += 1; 
                let time = SystemTime::now().duration_since(mine_sweeper.start_time).unwrap();
                restart_no_ui(&mut alpha_ai, &mut mine_sweeper, &wins, &losses, &mut times);
                println!("Time: {:?} ", time.as_micros());
            }
        }
    }
}

fn restart_no_ui(alpha_ai: &mut AlphaAI, mine_sweeper: &mut MineSweeper, wins: &usize, losses: &usize, times: &mut Vec<(Duration, usize)>) {
    print!("Wins: {:?} Losses: {:?} : {:?} Steps: {:?} ", *wins, *losses, (*wins as f64/(*wins+*losses) as f64), alpha_ai.step);
    print!("Time per step: {:?} ", if let Some(v) = average_time_per_step(times) {v.as_micros()} else {0});

    if let Ok((cols, rows, chance)) = get_args() {
        *mine_sweeper = MineSweeper::new(cols, rows, chance, ApperanceSettings::default());
    }
    else {
        *mine_sweeper = MineSweeper::default();
    }
    *alpha_ai = AlphaAI::new(mine_sweeper.cols(), mine_sweeper.rows());
}
