use graphics::{Ellipse, Line, circle_arc, color::BLACK, color::WHITE, grid::Grid, ellipse, rectangle};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::RenderArgs;
use std::time::SystemTime;
use crate::mine_field::*;

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

pub enum GameState {
    Running,
    Won,
    Lost
}

pub struct ApperanceSettings {
    pub square_size: f64,
    pub square_color: [f32;4],
    pub line_radius: f64, // TODO: rename to gap_radius?
    pub background_color: [f32;4],
}
impl Default for ApperanceSettings{
    fn default() -> Self {
        Self {
            square_size: 20.0,
            square_color: WHITE,
            line_radius: 1.0,
            background_color: BLACK
        }
    }
}

pub struct MineSweeper {
    gl: GlGraphics, 
    pub mine_field: Vec<Vec<Option<u8>>>,
    mine_count: usize,
    mines_flagged: usize,
    pub states: Vec<Vec<ShownState>>,
    pub apperance: ApperanceSettings,
    pub scale: [f64;2],
    pub game_state: GameState,
    pub start_time: SystemTime,
}

impl Default for MineSweeper {
    /// Returns 16*16 with 0.15 concentration and default apperance
    fn default() -> Self {
        MineSweeper::new(16, 16, 0.15, ApperanceSettings::default())
    }
}

impl MineSweeper {
    pub fn new(cols: usize, rows: usize, concentration: f64, appearance: ApperanceSettings) -> Self {
        let mut mine_count: usize = 0;
        // Create a new game and run it.
        MineSweeper {
            gl: GlGraphics::new(OpenGL::V3_2),
            mine_field: generate_random_grid(cols, rows, concentration, &mut mine_count),
            mine_count,
            mines_flagged: 0,
            states: vec![vec![ShownState::Hidden; cols]; rows],
            apperance: appearance,
            scale: [1f64;2], // No real need to have these in the MineSweeper struct, could just compute it every run
            game_state: GameState::Running,
            start_time: SystemTime::now(),
        }
    }


    pub fn cols(&self) -> usize {
        return self.states[0].len()
    }

    pub fn rows(&self) -> usize {
        return self.states.len()
    }


    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let c = self.gl.draw_begin(args.viewport());
        match self.game_state {
            GameState::Running => {
                clear(self.apperance.background_color, &mut self.gl);

                let transform = c.transform.scale(self.scale[0], self.scale[1]);

                // Draw the cells
                for (y, row) in self.states.iter().enumerate() {
                    for (x, col) in row.iter().enumerate() {
                        let rect: [f64;4] = [
                            (self.apperance.square_size * x as f64) + self.apperance.line_radius, 
                            (self.apperance.square_size * y as f64) + self.apperance.line_radius, 
                            self.apperance.square_size - self.apperance.line_radius * 2.0, 
                            self.apperance.square_size - self.apperance.line_radius * 2.0];
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
                                let time: f32 = SystemTime::now().duration_since(self.start_time).unwrap().as_secs_f32();
                                let color: [f32;4] = [(time.sin()+1.0)/2.0, ((time+1.57).sin()+1.0)/2.0, ((time+3.14).sin()+1.0)/2.0, 1.0];
                                rectangle(color, rect, transform, &mut self.gl);
                            }
                        }
                    }
                }
            }
            GameState::Won => {
                clear([0.0, 1.0, 0.0, 1.0], &mut self.gl);
            }
            GameState::Lost => {
                clear([1.0, 0.0, 0.0, 1.0], &mut self.gl);
            }
        }

        self.gl.draw_end();
    }

    pub fn left_click(&mut self, mouse_pos: [f64;2]) {
        let cell_pos = self.get_cell_from_position(mouse_pos);
        println!("L click at: {:?}={:?}, is {:?}", mouse_pos, cell_pos, self.mine_field[cell_pos[1]][cell_pos[0]]);
        if let Ok(hit_mine) = self.reveal_cell(cell_pos) {
            if hit_mine {
                self.game_state = GameState::Lost;
            }
        }
    }

    pub fn right_click(&mut self, mouse_pos: [f64;2]) {
        let cell_pos = self.get_cell_from_position(mouse_pos);
        println!("R click at: {:?}={:?}, is {:?}", mouse_pos, cell_pos, self.mine_field[cell_pos[1]][cell_pos[0]]);
        if let Ok(res) = self.toggle_flag_cell(cell_pos) {
            if res.1 {
                self.mines_flagged = if res.0 == ShownState::Flagged {self.mines_flagged+1} else {self.mines_flagged-1};
                if self.mines_flagged >= self.mine_count {
                    self.game_state = GameState::Won;
                }
            }
        }

    }

    fn get_cell_from_position(&self, position: [f64;2]) -> [usize;2] { //TODO: add bounds checking and return a result instead
        [(position[0] / self.apperance.square_size).trunc() as usize, (position[1] / self.apperance.square_size).trunc() as usize]
    }


    /// Fails if cell is already revealed or flagged, returns true if cell contains a mine
    fn reveal_cell(&mut self, position: [usize;2]) -> Result<bool, ()> {
        if self.states[position[1]][position[0]] == ShownState::Hidden {
            self.states[position[1]][position[0]] = ShownState::Revealed;
            if let Some(cell) = self.mine_field[position[1]][position[0]] {
                if cell == 0 {
                    for neighbour in NEIGHBOURS.iter() {
                        let x: i32 = position[0] as i32 + neighbour[0] as i32;
                        if x < 0 || x >= self.states[0].len() as i32 { continue;}
                        let y: i32 = position[1] as i32 + neighbour[1] as i32;
                        if y < 0 || y >= self.states.len() as i32 { continue;}
                        
                        self.reveal_cell( [x as usize, y as usize]);
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

    fn toggle_flag_cell (&mut self, position: [usize;2]) -> Result<(ShownState, bool), ()> { // TODO: refactor to not use matches
        match self.states[position[1] as usize][position[0] as usize] {
            ShownState::Hidden => {
                if let Ok(res) = self.flag_cell(position) {
                    return Ok((ShownState::Flagged, res))
                }
            }
            ShownState::Flagged => {
                if let Ok(res) = self.unflag_cell(position) {
                    return Ok((ShownState::Hidden, res))
                }
            }
            ShownState::Revealed => {}
        }
        return Err(());
    }

    /// Returns true if successfully unflagged cell and cell contains a mine
    fn flag_cell(&mut self, position: [usize;2]) -> Result<bool, ()> {
        if self.states[position[1]][position[0]] == ShownState::Hidden {
            self.states[position[1]][position[0]] = ShownState::Flagged;
            return Ok(self.mine_field[position[1]][position[0]] == None)
        }
        return Err(())
    }

    /// Returns true if successfully unflagged cell and cell contains a mine
    fn unflag_cell(&mut self, position: [usize;2]) -> Result<bool, ()> {
        if self.states[position[1]][position[0]] == ShownState::Flagged {
            self.states[position[1]][position[0]] = ShownState::Hidden;
            return Ok(self.mine_field[position[1]][position[0]] == None)
        }
        return Err(())
    }
}

