mod mine_field;
mod mine_sweeper;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;

use color::BLACK;
use piston_window::*;
use std::{time::SystemTime, env};
use env::args;
use graphics::{grid::Grid, line::Line};
use mine_field::*;
use mine_sweeper::*;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{Button, MouseButton, MouseCursorEvent, PressEvent, ResizeEvent, event_loop::{EventSettings, Events}};
use piston::input::{RenderEvent,  UpdateEvent};
use piston::window::WindowSettings;

enum GameState {
    Running,
    Won,
    Lost
}


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

const SQUARE_SIZE: f64 = 20.0;
const LINE_RADIUS: f64 = 1.0;

fn main() {
    let (cols, rows, chance) = get_args().unwrap();

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    
    let window_size: [f64;2] = [SQUARE_SIZE * cols as f64, SQUARE_SIZE * rows as f64]; 
    
    // Create an Glutin window.
    let mut window: PistonWindow = WindowSettings::new("Mine Sweeper", window_size)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .samples(4)
        .build()
        .unwrap();

    window.set_lazy(true);
    window.set_max_fps(120);

    let temp = generate_random_grid(cols, rows, chance);
    // Create a new game and run it.
    let mut mine_sweeper = MineSweeper {
        gl: GlGraphics::new(opengl),
        mine_field: temp.0,
        mine_count: temp.1,
        states: vec![vec![ShownState::Hidden; cols]; rows],
        grid: Grid {
            cols:(cols as u32),
            rows:(rows as u32),
            units: SQUARE_SIZE,
        },
        line: Line::new(BLACK, LINE_RADIUS),
        scale: window_size // No real need to have these in the MineSweeper struct, could just compute it every run
    };
    let mut game_state: GameState = GameState::Running; 
    let mut cursor = [0.0, 0.0];
    let time = SystemTime::now();
    let mut flagged_mines: usize = 0;
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        match game_state {
            GameState::Running => {
                if let Some(Button::Mouse(button)) = e.press_args() {
                    match button {
                        MouseButton::Left => {
                            if let Ok(hit_mine) = mine_sweeper.left_click([cursor[0] / mine_sweeper.scale[0], cursor[1] / mine_sweeper.scale[1]]) {
                                if hit_mine {
                                    // Gameover 
                                    game_state = GameState::Lost;
                                }
                            }
                        },
                        MouseButton::Right => {
                            if let Ok(res) = mine_sweeper.right_click([cursor[0] / mine_sweeper.scale[0], cursor[1] / mine_sweeper.scale[1]]) {
                                if res.1 {
                                    flagged_mines = if res.0 == ShownState::Flagged {flagged_mines+1} else {flagged_mines-1};
                                    if flagged_mines >= mine_sweeper.mine_count {
                                        game_state = GameState::Won;
                                    }
                                }
                            }
                        },
                        _ => ()
                    }
                }

                e.mouse_cursor(|pos| cursor = pos);
                e.resize(|args| mine_sweeper.scale = [args.window_size[0] / window_size[0], args.window_size[1] / window_size[1]]);

                if let Some(args) = e.render_args() { mine_sweeper.render(&args, &time); }
            }
            GameState::Won => {
                if let Some(_) = e.render_args() {
                    window.draw_2d(&e, |_c, g, _device| {
                     clear([0.0, 1.0, 0.0, 1.0], g);
                    });
                }

            }
            GameState::Lost => {
                if let Some(_) = e.render_args() {
                    window.draw_2d(&e, |_c, g, _device| {
                     clear([1.0, 0.0, 0.0, 1.0], g);
                    });
                }
            }
        }
        //if let Some(args) = e.update_args() { mine_sweeper.update(&args); }
    }
}
