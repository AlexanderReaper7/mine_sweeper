mod mine_field;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;

use piston_window::*;
use std::{char::from_digit, env};
use env::args;
use graphics::{Rectangle, color::{BLACK, WHITE}, grid::Grid, line::Line, rectangle};
use mine_field::{MineField, ShownState};
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL};
use piston::{Button, MouseButton, MouseCursorEvent, PressEvent, ResizeEvent, event_loop::{EventSettings, Events}};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

const GREY: [f32; 4] = [0.5,0.5,0.5,1.0];

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    mines: MineField,
    /// Contains the shown state for all squares
    states: Vec<Vec<ShownState>>,
    grid: Grid,
    line: Line,
    size: [f64;2]
}

impl App {
    #![feature(assoc_char_funcs)]
    fn render(&mut self, args: &RenderArgs, glyphs: &mut Glyphs) {
        use graphics::*;

        let c = self.gl.draw_begin(args.viewport());
        // Clear the screen.
        clear(BLACK, &mut self.gl);

        let transform = c.transform.scale(args.window_size[0] / self.size[0], args.window_size[1] / self.size[1]);

        // Draw the grid lines
        self.grid.draw(&self.line, &Default::default(), transform, &mut self.gl);
        // Draw the cells
        let mut y: usize = 0;
        for row in &self.states {
            let mut x: usize = 0;
            for col in row {
                match col {
                    // draw gray square
                    ShownState::Hidden => { 
                        rectangle(GREY, 
                            [self.grid.x_pos((x as u32,y as u32))+LINE_RADIUS, self.grid.y_pos((x as u32, y as u32))+LINE_RADIUS, SQUARE_SIZE-LINE_RADIUS*2.0, SQUARE_SIZE-LINE_RADIUS*2.0],
                             transform,
                              &mut self.gl
                            )
                    },
                    ShownState::Revealed => {
                        if let Some(sq) = &self.mines.squares[y][x] { 
                            // unless the cell is 0, draw the character
                            if *sq != 0 {
                                //let mut tmp = [0; 4];
                                //text(WHITE, 12, from_digit(*sq as u32, 10).unwrap().encode_utf8(&mut tmp), glyphs,transform, &mut self.gl);
                            }
                            else {

                            }
                         }
                         else {

                         }

                    },
                    // draw a flag here
                    ShownState::Flagged => {todo!()}
                }
                x += 1;
            }
            y += 1;
        }
        self.gl.draw_end();
    }

    fn update(&mut self, args: &UpdateArgs) {

    }


    fn left_click(&mut self, mouse_pos: [f64;2]) {
        println!("L click at: {:?}", mouse_pos);
        // get cell
        let v = get_cell_from_position(self.grid.units, mouse_pos);
        // set cell
    }

    fn right_click(&mut self, mouse_pos: [f64;2]) {
        println!("R click at: {:?}", mouse_pos);
    }
}

pub fn get_cell_from_position(units: f64, position: [f64;2]) -> [u32;2] {
    [(position[0] / units).trunc() as u32, (position[1] / units).trunc() as u32]
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
const FONT_PATH: &str = "assets/RobotoMono-Regular.ttf";

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

    // Import font
    let mut glyphs = window.load_font(FONT_PATH).unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        mines: MineField::generate_random_grid(cols, rows, chance),
        states: vec![vec![ShownState::Hidden; cols]; rows],
        grid: Grid {
            cols:(cols as u32),
            rows:(rows as u32),
            units: SQUARE_SIZE,
        },
        line: Line::new(WHITE, LINE_RADIUS),
        size: window_size // No real need to have these in the App strukt, could just compute it every run
    };
    let mut cursor = [0.0, 0.0];
    let mut scale = [0.0, 0.0];
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(Button::Mouse(button)) = e.press_args() {
            match button {
                MouseButton::Left => app.left_click([cursor[0] / scale[0], cursor[1] / scale[1]]),
                MouseButton::Right => app.right_click([cursor[0] / scale[0], cursor[1] / scale[1]]),
                _ => ()
            }
        }

        e.mouse_cursor(|pos| cursor = pos);
        e.resize(|args| scale = [args.window_size[0] / window_size[0], args.window_size[1] / window_size[1]]);

        if let Some(args) = e.render_args() {
            app.render(&args, &mut glyphs);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
