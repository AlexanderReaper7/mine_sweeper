use graphics::{color::BLACK, color::WHITE, ellipse, rectangle};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::RenderArgs;
use std::time::SystemTime;
use std::char::from_digit;
use rand::prelude::*;

pub const COLORS: [[f32; 4]; 8] = [
    [0.0,0.0,1.0,1.0], // Blue
    [0.0,1.0,0.0,1.0], // Green
    [1.0,0.0,0.0,1.0], // Red
    [1.0,0.4,0.9,1.0], // Pink
    [0.0,0.0,0.6,1.0], // Dark Blue
    [0.0,0.6,0.0,1.0], // Dark green
    [0.6,0.0,0.0,1.0], // Dark Red
    [0.65,0.0,0.55,1.0], // purple
    ];

pub const NEIGHBOURS: [[i8;2];8] = [[-1, -1], [-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 0], [1, 1] ];

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ShownState {
    Hidden,
    Revealed,
    Flagged
}
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

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::*;

        let c = gl.draw_begin(args.viewport());
        match self.game_state {
            GameState::Running => {
                clear(self.apperance.background_color, gl);

                let windows_scaling: [f64;2] = [c.get_view_size()[0] / c.viewport.unwrap().draw_size[0] as f64, c.get_view_size()[1] / c.viewport.unwrap().draw_size[1] as f64];
                let transform = c.transform.scale(windows_scaling[0], windows_scaling[1]).scale(self.scale[0], self.scale[1]);

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
                                rectangle(WHITE, rect, transform,gl);
                            },
                            ShownState::Revealed => {
                                if let Some(sq) = &self.mine_field[y][x] { 
                                    // unless the cell is 0, draw the character
                                    if *sq != 0 {
                                        //let color: [f32;4] = [(*sq%2).into(), f32::from(*sq%3)/2.0, f32::from(*sq%4)/3.0, 1.0];
                                        rectangle(COLORS[(*sq -1) as usize], rect, transform, gl);
                                    }
                                }
                                else {
                                    // Draw bomb
                                    // ellipse(WHITE, rect, transform, gl);
                                }
                            },
                            // draw a flag here
                            ShownState::Flagged => {
                                let time: f32 = SystemTime::now().duration_since(self.start_time).unwrap().as_secs_f32();
                                let color: [f32;4] = [(time.sin()+1.0)/2.0, ((time+1.57).sin()+1.0)/2.0, ((time+3.14).sin()+1.0)/2.0, 1.0];
                                rectangle(color, rect, transform, gl);
                            }
                        }
                    }
                }
            }
            GameState::Won => {
                clear([0.0, 1.0, 0.0, 1.0], gl);
            }
            GameState::Lost => {
                clear([1.0, 0.0, 0.0, 1.0], gl);
            }
        }

        gl.draw_end();
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

    pub fn left_click_cell(&mut self, cell_pos: [usize;2]) {
        //println!("L click at: {:?}, is {:?}", cell_pos, self.mine_field[cell_pos[1]][cell_pos[0]]);
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

    pub fn right_click_cell(&mut self, cell_pos: [usize;2]) {
        //println!("R click at: {:?}, is {:?}", cell_pos, self.mine_field[cell_pos[1]][cell_pos[0]]);
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
                        
                        let _ = self.reveal_cell( [x as usize, y as usize]);
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

    fn toggle_flag_cell(&mut self, position: [usize;2]) -> Result<(ShownState, bool), ()> { // TODO: refactor to not use matches
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

fn rand_func(rng: &mut ThreadRng, chance: f64) -> bool {
    let y = rng.gen_range(0.0, 1.0);
    return y <= chance
}

pub fn generate_random_grid(cols: usize, rows: usize, mine_concentration: f64, mine_count_output: &mut usize) -> Vec<Vec<Option<u8>>> {
    let mut rng_thread = rand::thread_rng();
    let mut squares: Vec<Vec<Option<u8>>>  = vec![vec![None;cols];rows];
    for row in squares.iter_mut() {
        for sq in row.iter_mut() {
            *sq = if rand_func(&mut rng_thread, mine_concentration) {*mine_count_output += 1; None} else {Some(0)}
        }
    }
    // count mines in neighbouring squares
    count_adjacent_mines(&mut squares);

    return squares;
}

fn count_adjacent_mines(squares: &mut Vec<Vec<Option<u8>>>) {
    for row in 0..squares.len() {
        for col in 0..squares[row].len() {
            if squares[row][col].is_some() {
                for y in 0..3 {
                    for x in 0..3 {
                        if x == 1 && y == 1 {continue;} // skip self
                        let y_sq = (row+y) as isize -1;
                        let x_sq = (col+x) as isize -1;
                        if y_sq < 0 || y_sq >= squares.len() as isize {continue;}
                        if x_sq < 0 || x_sq >= squares[y_sq as usize].len() as isize {continue;}
                        if squares[row+y-1][col+x-1].is_none() {
                            match &mut squares[row][col] {
                                None => {},
                                Some(sq) => {
                                    *sq += 1;
                                }
                            }
                        }
                    }
                }
            }
            // Print the values as they are being made
            // if squares[row][col].is_none() {print!("X ")} else {print!("{} ", from_digit(squares[row][col].unwrap().into(), 10).unwrap())};  // add this square´s character to output
            // if col +1 % squares[row].len() == 0 { // if at the end of row, add new-line character
            //     println!();
            // }  
        }
    }
}

#[allow(dead_code)]
pub fn print(mine_field: &Vec<Vec<Option<u8>>>) {
    let mut output: String = "\n".to_string();
    for row in mine_field.iter() {
        for (i, sq) in row.iter().enumerate() {
            output.push(if *sq == None {'X'} else {from_digit(sq.unwrap().into(), 10).unwrap()});  // add this square´s character to output
            if (i + 1) % row.len() == 0 { // if at the end of row, add new-line character
                output.push('\n');
            }  
            else {  // else add space
                output.push(' ');
            }
        }
    }
    println!("{}", output);
    //stdout().flush().unwrap();
}
/* 
#[cfg(test)]
 mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(4, 2*2);
    }

    #[bench]
    fn bench_minefield_creation(b: &mut Bencher) {
        let mut _mines: usize = 0;
        b.iter(|| generate_random_grid(100, 100, 0.15, &mut _mines))
    }
} */