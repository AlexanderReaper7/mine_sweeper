mod mine_field;
mod mine_sweeper;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;

use piston_window::*;
use std::{time::SystemTime, env};
use env::args;
use mine_field::*;
use mine_sweeper::*;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{Button, MouseButton, MouseCursorEvent, PressEvent, ResizeEvent, event_loop::{EventSettings, Events}};
use piston::input::{RenderEvent,  UpdateEvent};
use piston::window::WindowSettings;


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
            3 => match arg.parse() {
                Ok(v) => {chance = v},
                Err(_) => {return Err("Failed to convert chance")},
            }
            _ => return Err("unknown error")
            
        }
    }
    return Ok((col, row, chance))
}

fn main() {
    let mut mine_sweeper: MineSweeper;

    if let Ok((cols, rows, chance)) = get_args() {
        mine_sweeper = MineSweeper::New(cols, rows, chance, ApperanceSettings::default());
    }
    else {
        mine_sweeper = MineSweeper::default();
    }
    
    let window_size: [f64;2] = [mine_sweeper.apperance.square_size * mine_sweeper.cols() as f64, mine_sweeper.apperance.square_size * mine_sweeper.rows() as f64]; 
    
    // Create a window.
    let mut window: PistonWindow = WindowSettings::new("Mine Sweeper", window_size)
        .graphics_api(OpenGL::V3_2)
        .exit_on_esc(true)
        .samples(4)
        .build()
        .unwrap();

    window.set_lazy(true);
    window.set_max_fps(120);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        e.resize(|args| mine_sweeper.scale = [args.window_size[0] / window_size[0], args.window_size[1] / window_size[1]]);

        if let Some(args) = e.render_args() { 
            mine_sweeper.render(&args); 
            // Additional rendering (on top of mine field) goes here...
            // need to start and end again, might move to outside of MineSweeper::render
        }

        if let Some(args) = e.update_args() { update_ai(&mut mine_sweeper, &args); }
    }
}

fn update_ai(mine_sweeper: &mut MineSweeper, args: &UpdateArgs) {
    match mine_sweeper.game_state {
        GameState::Running => {}
        _ => (),
    }
    

}
