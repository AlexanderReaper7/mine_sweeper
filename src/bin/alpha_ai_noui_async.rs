extern crate mine_sweeper;
mod alpha_ai;

use time::Duration;
use std::{time::SystemTime};
use mine_sweeper::{*, mine_sweeper::*};
use std::time;
use alpha_ai::*;
use rayon::prelude::*;

fn main() {
    let mut mine_sweeper: MineSweeper;
    if let Ok((cols, rows, chance)) = get_args() {
        mine_sweeper = MineSweeper::new(cols, rows, chance, ApperanceSettings::default());
    }
    else {
        mine_sweeper = MineSweeper::default();
    }
    let mut times: Vec<(Duration, usize)> = Vec::with_capacity(1000);
    let mut alpha_ai: AlphaAI = AlphaAI::new(mine_sweeper.cols(), mine_sweeper.rows());

    
}