mod mine_field;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use graphics::{Graphics, clear, color::{BLACK, WHITE}, grid::Grid, line::Line, draw_state, math, rectangle};
use mine_field::MineField;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    mines: MineField,
    grid: Grid,
    line: Line,
}

pub fn grid<G>(grid: &Grid, line: &Line, transform: math::Matrix2d, g: &mut G)
where G: Graphics
{
    grid.draw(line, &Default::default(), transform, g);
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, 50.0);
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            let transform = c
                .transform
                .trans(x, y)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(BLACK, square, transform, gl);
            grid(&self.grid, &self.line, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        //self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let x_size = 800;
    let y_size = 800;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Mine Sweeper", [x_size, y_size])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .samples(4)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        mines: MineField::generate_random_grid(12, 12, 0.2),
        grid: Grid{cols: 12, rows: 12, units: 20.0},
        line: Line::new(WHITE, 1.0)
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
