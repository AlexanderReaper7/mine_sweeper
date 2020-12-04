use std::{char::from_digit, io::*};
use rand::prelude::*;
use std::time::SystemTime;

#[allow(dead_code)]

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ShownState {
    Hidden,
    Revealed,
    Flagged
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

fn rand_func(rng: &mut ThreadRng, chance: f64) -> bool {
    let y = rng.gen_range(0.0, 1.0);
    return y <= chance
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
    stdout().flush().unwrap();
}

#[allow(dead_code)]
fn benchmark() {
    let mut times = 0;
    let mut i = 1;
    while times < 21 {
        let time = SystemTime::now();
        let mut _mine_count: usize = 0;
        let _grid = generate_random_grid(1000, i, 0.2, &mut _mine_count);
        let elap = time.elapsed();
        print!("{:?} {:?} {:?} ", times, i, elap);
        println!("time per 1000: {:?} µs", elap.unwrap().as_micros() /  i as u128);
        i *= 2;
        times += 1;
    }
}
