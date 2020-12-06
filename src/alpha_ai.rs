mod mine_field;
mod mine_sweeper;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;

use piston_window::*;
use time::Duration;
use std::{time::SystemTime, env};
use env::args;
use mine_field::*;
use mine_sweeper::*;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{ResizeEvent, event_loop::{EventSettings, Events}};
use piston::input::{RenderEvent,  UpdateEvent};
use piston::window::WindowSettings;
use std::collections::VecDeque;
use rand::prelude::*;
use std::{thread, time};

fn get_args() -> Result<(usize, usize, f64), &'static str> {
    if env::args_os().len() != 4 {
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
    print!("Wins: {:?} Losses: {:?} : {:?} ", *wins, *losses, (*wins as f64/(*wins+*losses) as f64));
    print!("Time per step: {:?} ", if let Some(v) = average_time_per_step(times) {v.as_micros()} else {0});

    if let Ok((cols, rows, chance)) = get_args() {
        *mine_sweeper = MineSweeper::new(cols, rows, chance, ApperanceSettings::default());
    }
    else {
        *mine_sweeper = MineSweeper::default();
    }
    *alpha_ai = AlphaAI::new(mine_sweeper.cols(), mine_sweeper.rows());
}


fn main2() {
    // Create a window. the window need to be created before MineSweeper
    let mut window: PistonWindow = WindowSettings::new("Mine Sweeper: Alpha AI", [100.0;2])
        .graphics_api(OpenGL::V3_2)
        .exit_on_esc(true)
        //.samples(4)
        .vsync(false)
        .build()
        .unwrap();

    let mut mine_sweeper: MineSweeper;
    if let Ok((cols, rows, chance)) = get_args() {
        mine_sweeper = MineSweeper::new(cols, rows, chance, ApperanceSettings::default());
    }
    else {
        mine_sweeper = MineSweeper::default();
    }
    let window_size: [f64;2] = [mine_sweeper.apperance.square_size * mine_sweeper.cols() as f64, mine_sweeper.apperance.square_size * mine_sweeper.rows() as f64]; 
    window.set_size(window_size);


    let mut wins: usize = 0;
    let mut losses: usize = 0;
    let mut times: Vec<(Duration, usize)> = Vec::with_capacity(40);
    let mut alpha_ai: AlphaAI = AlphaAI::new(mine_sweeper.cols(), mine_sweeper.rows());
    let sleep_time = time::Duration::from_millis(0);


    let mut gl: GlGraphics = GlGraphics::new(OpenGL::V3_2);
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        e.resize(|args| mine_sweeper.scale = [args.window_size[0] / window_size[0], args.window_size[1] / window_size[1]]);

        if let Some(args) = e.render_args() { 
            mine_sweeper.render(&args, &mut gl);
            // Additional rendering (on top of mine field) goes here...
            // need to start and end again, might move to outside of MineSweeper::render
        }

        if let Some(args) = e.update_args() { 
            match mine_sweeper.game_state {
                GameState::Running => {
                    let time = SystemTime::now();
                    alpha_ai.update_ai(&mut mine_sweeper); 
                    //println!("step: {:?}, took {:?} us", alpha_ai.step, SystemTime::now().duration_since(time).unwrap().as_micros());
                    alpha_ai.step += 1;
        
                }
                GameState::Won => {
                    wins += 1; 
                    let time = SystemTime::now().duration_since(mine_sweeper.start_time).unwrap();
                    times.push((time , alpha_ai.step));
                    restart(&mut alpha_ai, &mut mine_sweeper, &mut window, &wins, &losses, &mut times);
                    println!("Time: {:?} ", time.as_micros());
                    }
                GameState::Lost => {
                    losses += 1; 
                    let time = SystemTime::now().duration_since(mine_sweeper.start_time).unwrap();
                    restart(&mut alpha_ai, &mut mine_sweeper, &mut window, &wins, &losses, &mut times);
                    println!("Time: {:?} ", time.as_micros());
                }
            }
            thread::sleep(sleep_time);
        }
    }
}

fn restart(alpha_ai: &mut AlphaAI, mine_sweeper: &mut MineSweeper, window: &mut PistonWindow, wins: &usize, losses: &usize, times: &mut Vec<(Duration, usize)>) {
    print!("Wins: {:?} Losses: {:?} ", *wins, *losses);
    print!("per step: {:?} ", if let Some(v) = average_time_per_step(times) {v.as_micros()} else {0});
    thread::sleep(time::Duration::from_millis(1));

    if let Ok((cols, rows, chance)) = get_args() {
        *mine_sweeper = MineSweeper::new(cols, rows, chance, ApperanceSettings::default());
    }
    else {
        *mine_sweeper = MineSweeper::default();
    }
    let window_size: [f64;2] = [mine_sweeper.apperance.square_size * mine_sweeper.cols() as f64, mine_sweeper.apperance.square_size * mine_sweeper.rows() as f64]; 
    *alpha_ai = AlphaAI::new(mine_sweeper.cols(), mine_sweeper.rows());
    window.set_size(window_size)
}

fn average_time_per_step(times: &Vec<(Duration, usize)>) -> Option<Duration> {
    if times.len() < 1 {return None;}
    let mut sum: Duration = Duration::from_nanos(0);
    for (time, steps) in times {
        sum += *time / *steps as u32;
    }   
    return Some(sum / times.len() as u32)
}

enum AiActions {
    Reveal,
    Flag,
    EvalSurroundingReveals
}

struct AlphaAI {
    action_queue: VecDeque<(AiActions, [usize;2])>,
    step: usize,
    starts: Vec<(usize, usize)>,
    ends: Vec<(usize, usize)>,
    safes: Vec<Vec<bool>>,
}
impl AlphaAI {
    pub fn new(cols: usize, rows: usize) -> Self {
        AlphaAI {
            action_queue: VecDeque::with_capacity(10),
            step: 0,
            starts: Vec::new(),
            ends: Vec::new(),
            safes: vec![vec![false;cols];rows],
        }
    }

    pub fn update_ai(&mut self, mine_sweeper: &mut MineSweeper) {
        match mine_sweeper.game_state {
            GameState::Running => {
                if let Some(next) = self.action_queue.pop_front() {
                    match next.0 {
                        AiActions::Reveal => {
                            mine_sweeper.left_click_cell(next.1);
                        }
                        AiActions::Flag => {
                            mine_sweeper.right_click_cell(next.1);
                        }
                        AiActions::EvalSurroundingReveals => {
                            self.eval_cell_for_safe_surrounding_reveals(mine_sweeper, next.1);
                        }
                    }
                } else {
                    if self.search_field(mine_sweeper) {return;}
                    //self.reveal_least_risky(mine_sweeper); // 6.5% <- bad version // 18%  28% 67%
                    self.reveal_random(mine_sweeper); // 12%  22%
                }
            }
            _ => (),
        }
    }

    fn reveal_random(&mut self, mine_sweeper: &mut MineSweeper) {
        let mut rng_thread = rand::thread_rng();
        loop {
            let x = rng_thread.gen_range(0, mine_sweeper.cols());
            let y = rng_thread.gen_range(0, mine_sweeper.rows());
            if mine_sweeper.states[y][x] == ShownState::Hidden {
                self.action_queue.push_back((AiActions::Reveal, [x,y]));
                //println!("! revealing random at {:?} !", [x,y]);
                return;
            }
        }
    }

    /// reveal something that is of low risk and high value.
    /// i.e away from walls, adjacent to a cell with 
    fn reveal_least_risky(&mut self, mine_sweeper: &mut MineSweeper) {
        let risks: Vec<Vec<Option<f32>>> = self.calculate_risks(mine_sweeper);
        // get lowest risk
        let mut res: Option<(f32,usize,usize)> = None;
        for (y, row) in risks.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {
                if let Some(risk) = item {
                    if res.is_some() {
                        if res.unwrap().0 > *risk {
                            res = Some((*risk, x, y));
                        }
                    } else {
                        res = Some((*risk, x, y));
                    }
                }
            }
        }

        // reveal
        if let Some(result) = res {
            self.action_queue.push_back((AiActions::Reveal, [result.1, result.2]));
        } else {
            self.reveal_random(mine_sweeper);
        }
    }

    fn calculate_risks(&mut self, mine_sweeper: &mut MineSweeper) -> Vec<Vec<Option<f32>>> {
        let mut risks: Vec<Vec<Option<f32>>> = vec![vec![None;mine_sweeper.cols()]; mine_sweeper.rows()];
        let mut y = 0;
        let mut x = 0;
        let mut skip = 0;
        'outer: while y < mine_sweeper.rows() { // TODO: use skips
            while x < mine_sweeper.cols() {
                if self.try_skip(&mut x, &mut y, &mut skip) { continue 'outer; }

                if let Ok((flags, hiddens)) = AlphaAI::count_surrounding_possible_mines(mine_sweeper, [x,y]) {
                    let val = mine_sweeper.mine_field[y][x].unwrap();
                    let cell_risk: f32 = (val - flags) as f32 / hiddens.len() as f32;
                    for hidden in hiddens {
                        if let Some(cell) = risks[hidden[1]][hidden[0]] {
                            if cell_risk > cell {
                                risks[hidden[1]][hidden[0]] = Some(cell_risk);
                            }
                        } else {
                            risks[hidden[1]][hidden[0]] = Some(cell_risk);
                        }
                    }
                }
                x += 1;
            }
            y += 1;
            x = 0;
        }
        return risks;
    }

    /// Returns true if found cell to reveal/flag
    fn search_field(&mut self, mine_sweeper: &mut MineSweeper) -> bool {
        let mut skip: usize = 0;
        let mut y: usize = 0;
        let mut x: usize = 0;
        'outer: while y < mine_sweeper.rows() {
            while x < mine_sweeper.cols() {
                if self.try_skip(&mut x, &mut y, &mut skip) { continue 'outer; }
                if self.eval_cell_for_safe_surrounding_reveals(mine_sweeper, [x, y]) {
                    //println!("found cells at {:?}", [x,y]); 
                    self.update_skips();
                    return true;
                }
                x += 1;
            }
            y += 1;
            x = 0;
        }
        return false;
    }

    /// Skips start on a safe and ends on an unsafe
    fn update_skips(&mut self) {
        let mut skip: usize = 0;
        let mut y: usize = 0;
        let mut x: usize = 0;
        'outer: while y < self.safes.len() {
            while x < self.safes[0].len() {
                // if its safe then there should be a skip here
                if self.safes[y][x] {
                    // if there are more skips
                    if skip < self.starts.len() {
                        // and it starts here
                        if (x,y) == self.starts[skip] {
                            // then skip
                            x = self.ends[skip].0;
                            y = self.ends[skip].1;
                            skip += 1;
                            // and try to update skip
                            self.update_skip(&mut x, &mut y, &mut skip);
                            continue 'outer;
                        }
                    }
                    self.create_skip(&mut x, &mut y, &mut skip);
                    continue 'outer;
                }
                x += 1;
            }
            y += 1;
            x = 0;
        }
    }

    /// creates a skip at the current indexies
    fn create_skip(&mut self, x:  &mut usize, y:  &mut usize, skip:  &mut usize) {
        self.starts.insert(*skip, (*x,*y));
        if *x+1 < self.safes[0].len() {
            // inc x
            *x = *x+1;
        } else {
            // inc y
            *x = 0;
            *y += 1;
        }
        self.ends.insert(*skip, (*x,*y));
        *skip += 1;
        self.update_skip(x, y, skip);
    }

    /// Only run after having skipped, updates the skip just used.
    fn update_skip(&mut self, x:  &mut usize, y:  &mut usize, skip:  &mut usize) {
        'outer: while *y < self.safes.len() {
            while *x < self.safes[0].len() {
                if self.safes[*y][*x] != true {return;}
                // skip longer
                if *x+1 < self.safes[0].len() {
                    // inc x
                    self.ends[*skip-1].0 += 1;
                } else if *y+1 < self.safes.len(){
                    // inc y
                    self.ends[*skip-1].1 += 1;
                    // reset x
                    self.ends[*skip-1].0 = 0;
                } else {
                    return;
                }
                // if there are more skips
                if *skip < self.starts.len() {
                    // if it starts here
                    if self.starts[*skip] == self.ends[*skip-1] {
                        // merge and remove old
                        self.ends[*skip-1] = self.ends.remove(*skip);
                        self.starts.remove(*skip);
                        // jump to next
                        *x = self.ends[*skip-1].0;
                        *y = self.ends[*skip-1].1;
                        continue 'outer;
                    }
                }
                *x += 1;
            }
            *y += 1;
            *x = 0;
        }
    }

    fn try_skip(&mut self, x: &mut usize, y: &mut usize, skip: &mut usize) -> bool {
        if *skip < self.starts.len() {
            if (*x,*y) == self.starts[*skip] {
                *x = self.ends[*skip].0;
                *y = self.ends[*skip].1;
                *skip += 1;
                return true;
            }
        }
        return false
    }

    fn eval_cell_for_safe_surrounding_reveals(&mut self, mine_sweeper: &mut MineSweeper, target: [usize;2]) -> bool {
        //println!("searching: {:?}", target);
        if let Some(val) = &mine_sweeper.mine_field[target[1]][target[0]] {
            if let Ok((flags ,hiddens)) = AlphaAI::count_surrounding_possible_mines(mine_sweeper, target) {
                if hiddens.len() != 0 {
                    // if val - flags == hidden, flag cells
                    if val - flags == hiddens.len() as u8 {
                        for position in hiddens.iter() {
                            self.action_queue.push_back((AiActions::Flag, *position))
                        }
                        self.safes[target[1]][target[0]] = true;
                        return true;
                    }
                    // if val == flags, reveal cells
                    else if flags == *val {
                        for position in hiddens.iter() {
                            self.action_queue.push_back((AiActions::Reveal, *position))
                        }
                        self.safes[target[1]][target[0]] = true;
                        return true;
                    }
                } else {
                    self.safes[target[1]][target[0]] = true;
                }
            }
        }
        return false;
    }

    fn count_surrounding_possible_mines(mine_sweeper: &MineSweeper, target: [usize;2]) -> Result<(u8, Vec<[usize;2]>), &str>{
        if mine_sweeper.states[target[1]][target[0]] == ShownState::Revealed {
            if let Some(_) = &mine_sweeper.mine_field[target[1]][target[0]] {
                let mut count: u8 = 0;
                let mut hidden: Vec<[usize;2]> = Vec::with_capacity(8);
                for neighbour in NEIGHBOURS.iter() {
                    let x: i32 = target[0] as i32 + neighbour[0] as i32;
                    if x < 0 || x >= mine_sweeper.cols() as i32 { continue;}
                    let y: i32 = target[1] as i32 + neighbour[1] as i32;
                    if y < 0 || y >= mine_sweeper.rows() as i32 { continue;}
                    
                    match mine_sweeper.states[y as usize][x as usize] {
                        ShownState::Hidden => hidden.push([x as usize, y as usize]),
                        ShownState::Flagged => {count += 1;},
                        _ => {}
                    }
                }
                hidden.shrink_to_fit();
                return Ok((count, hidden))

            } else {
                return Err("target cannot be a mine")
            }

        } else {
            return Err("target must be revealed")
        }
    }
}
