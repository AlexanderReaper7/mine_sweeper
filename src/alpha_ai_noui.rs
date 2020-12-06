fn main() {

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
    let sleep_time = time::Duration::from_millis(1);


    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        e.resize(|args| mine_sweeper.scale = [args.window_size[0] / window_size[0], args.window_size[1] / window_size[1]]);

        if let Some(args) = e.render_args() { 
            mine_sweeper.render(&args); 
            // Additional rendering (on top of mine field) goes here...
            // need to start and end again, might move to outside of MineSweeper::render
        }

        if let Some(args) = e.update_args() { 
            match mine_sweeper.game_state {
                GameState::Running => {
                    let time = SystemTime::now();
                    alpha_ai.update_ai(&mut mine_sweeper, &args); 
                    //println!("step: {:?}, took {:?} us", alpha_ai.step, SystemTime::now().duration_since(time).unwrap().as_micros());
                    alpha_ai.step += 1;
        
                }
                GameState::Won => {
                    wins += 1; 
                    println!("\n WON!!!!");
                    times.push((SystemTime::now().duration_since(mine_sweeper.start_time).unwrap(), alpha_ai.step));
                    restart(&mut alpha_ai, &mut mine_sweeper, &mut window, &wins, &losses, &mut times);
                }
                GameState::Lost => {
                    losses += 1; 
                    println!("\n LOST!!!!");
                    restart(&mut alpha_ai, &mut mine_sweeper, &mut window, &wins, &losses, &mut times);
                }
            }
            //thread::sleep(sleep_time);
        }
    }
}
