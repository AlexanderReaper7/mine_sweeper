use graphics::{Ellipse, Line, circle_arc, color::BLACK, color::WHITE, grid::Grid, ellipse, rectangle};
use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};
use std::{cell::RefCell, time::{SystemTime, Duration}};
use crate::mine_field::*;
use std::f32::*;

const COLORS: [[f32; 4]; 8] = [
    [0.0,0.0,1.0,1.0], // Blue
    [0.0,1.0,0.0,1.0], // Green
    [1.0,0.0,0.0,1.0], // Red
    [1.0,0.4,0.9,1.0], // Pink
    [0.0,0.0,0.6,1.0], // Dark Blue
    [0.0,0.6,0.0,1.0], // Dark green
    [0.6,0.0,0.0,1.0], // Dark Red
    [0.65,0.0,0.55,1.0], // purple
    ];
const NEIGHBOURS: [[i8;2];8] = [[-1, -1], [-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 0], [1, 1] ];



pub struct MineSweeper {
    /// OpenGL drawing backend.
    pub gl: GlGraphics, 
    pub mine_field: Vec<Vec<Option<u8>>>,
    pub mine_count: usize,
    /// Contains the shown state for all squares
    pub states: Vec<Vec<ShownState>>,
    pub grid: Grid, // TODO: remove grid and line
    pub line: Line,
    pub scale: [f64;2]
}

impl MineSweeper {
    pub fn Cols(&self) -> usize {
        return self.states[0].len()
    }

    pub fn rows(&self) -> usize {
        return self.states.len()
    }


    pub fn render(&mut self, args: &RenderArgs, time: &SystemTime) {
        use graphics::*;

        let c = self.gl.draw_begin(args.viewport());
        // Clear the screen.
        clear(self.line.color, &mut self.gl);

        let transform = c.transform.scale(self.scale[0], self.scale[1]);

        // Draw the cells
        for (y, row) in self.states.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                let rect = [self.grid.x_pos((x as u32, y as u32))+self.line.radius, self.grid.y_pos((x as u32, y as u32))+self.line.radius, self.grid.units-self.line.radius*2.0, self.grid.units-self.line.radius*2.0];
                match col {
                    // draw White square
                    ShownState::Hidden => { 
                        rectangle(WHITE, rect, transform,&mut self.gl);
                    },
                    ShownState::Revealed => {
                        if let Some(sq) = &self.mine_field[y][x] { 
                            // unless the cell is 0, draw the character
                            if *sq != 0 {
                                //let color: [f32;4] = [(*sq%2).into(), f32::from(*sq%3)/2.0, f32::from(*sq%4)/3.0, 1.0];
                                rectangle(COLORS[(*sq -1) as usize], rect, transform, &mut self.gl);
                            }
                         }
                         else {
                            // Draw bomb
                            ellipse(WHITE, rect, transform, &mut self.gl);
                         }

                    },
                    // draw a flag here
                    ShownState::Flagged => {
                        let time: f32 = (SystemTime::now().duration_since(*time).unwrap().as_secs_f32()) as f32;
                        let color: [f32;4] = [(time.sin()+1.0)/2.0, ((time+1.57).sin()+1.0)/2.0, ((time+3.14).sin()+1.0)/2.0, 1.0];
                        rectangle(color, rect, transform, &mut self.gl);
                    }
                }
            }
        }
        self.gl.draw_end();
    }

    /// Fails if cell is already revealed or flagged, returns true if cell contains a mine TODO: move these to main
    pub fn left_click(&mut self, mouse_pos: [f64;2]) -> Result<bool, ()> {
        let v = MineSweeper::get_cell_from_position(self.grid.units, mouse_pos);
        println!("L click at: {:?}={:?}, is {:?}", mouse_pos, v, self.mine_field[v[1] as usize][v[0] as usize]);
        MineSweeper::reveal_cell(&self.mine_field, &mut self.states, v)
    }

    /// Returns (new state, does cell contain a mine?)
    pub fn right_click(&mut self, mouse_pos: [f64;2]) -> Result<(ShownState, bool), ()> {
        let v = MineSweeper::get_cell_from_position(self.grid.units, mouse_pos);
        println!("R click at: {:?}={:?}, is {:?}", mouse_pos, v, self.mine_field[v[1] as usize][v[0] as usize]);
        match self.states[v[1] as usize][v[0] as usize] {
            ShownState::Hidden => {
                if let Ok(res) = self.flag_cell(v) {
                    return Ok((ShownState::Flagged, res))
                }
            }
            ShownState::Flagged => {
                if let Ok(res) = self.unflag_cell(v) {
                    return Ok((ShownState::Hidden, res))
                }
            }
            ShownState::Revealed => {}
        }
        return Err(());
    }

    fn get_cell_from_position(units: f64, position: [f64;2]) -> [u32;2] { //TODO: add bounds checking and return a result instead
        [(position[0] / units).trunc() as u32, (position[1] / units).trunc() as u32]
    }


    /// Fails if cell is already revealed or flagged, returns true if cell contains a mine
    fn reveal_cell(mine_field: &Vec<Vec<Option<u8>>>, states: &mut Vec<Vec<ShownState>>, position: [u32;2]) -> Result<bool, ()> {
        if states[position[1] as usize][position[0] as usize] == ShownState::Hidden {
            states[position[1] as usize][position[0] as usize] = ShownState::Revealed;
            if let Some(cell) = mine_field[position[1] as usize][position[0] as usize] {
                if cell == 0 {
                    for neighbour in NEIGHBOURS.iter() {
                        let x: i32 = position[0] as i32 + neighbour[0] as i32;
                        if x < 0 || x >= states[0].len() as i32 { continue;}
                        let y: i32 = position[1] as i32 + neighbour[1] as i32;
                        if y < 0 || y >= states.len() as i32 { continue;}
                        
                        MineSweeper::reveal_cell(mine_field, states, [x as u32, y as u32]);
                    }
                }
                return Ok(false)
            }
            else{
                return Ok(true)
            }
        }
        return Err(())
    }

    /// Returns true if successfully unflagged cell and cell contains a mine
    fn flag_cell(&mut self, position: [u32;2]) -> Result<bool, ()> {
        if self.states[position[1] as usize][position[0] as usize] == ShownState::Hidden {
            self.states[position[1] as usize][position[0] as usize] = ShownState::Flagged;
            return Ok(self.mine_field[position[1] as usize][position[0] as usize] == None)
        }
        return Err(())
    }

    /// Returns true if successfully unflagged cell and cell contains a mine
    fn unflag_cell(&mut self, position: [u32;2]) -> Result<bool, ()> {
        if self.states[position[1] as usize][position[0] as usize] == ShownState::Flagged {
            self.states[position[1] as usize][position[0] as usize] = ShownState::Hidden;
            return Ok(self.mine_field[position[1] as usize][position[0] as usize] == None)
        }
        return Err(())
    }
}

