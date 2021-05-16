extern crate mine_sweeper;
#[path = "../src/bin/alpha_ai.rs"] mod alpha_ai;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use alpha_ai::AlphaAI;
use mine_sweeper::mine_sweeper::{GameState, MineSweeper, ApperanceSettings};

fn bench_alpha_ai_noui(cols: usize, rows: usize, concentration: f64) {
    let mut mine_sweeper = MineSweeper::new(cols, rows, concentration, ApperanceSettings::default());
    let mut alpha_ai: AlphaAI = AlphaAI::new(mine_sweeper.cols(), mine_sweeper.rows());

    loop {
        match mine_sweeper.game_state {
            GameState::Running => {
                alpha_ai.update_ai(&mut mine_sweeper); 
                alpha_ai.step += 1;
            }
            GameState::Won => {
                return;
            }
            GameState::Lost => {
                return;
            }
        }
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Minesweeper creation 10x10 0.2",|b| b.iter(|| MineSweeper::new(black_box(10), black_box(10), 0.2, ApperanceSettings::default())));
    c.bench_function("Minesweeper creation 100x100 0.2",|b| b.iter(|| MineSweeper::new(black_box(100), black_box(100), 0.2, ApperanceSettings::default())));
    c.bench_function("Minesweeper creation 1000x1000 0.2",|b| b.iter(|| MineSweeper::new(black_box(1000), black_box(1000), 0.2, ApperanceSettings::default())));
    
    //c.bench_function("Alpha Ai 10x10 0.1",|b| b.iter(|| bench_alpha_ai_noui(black_box(10), black_box(10), 0.1f64)));
    //c.bench_function("Alpha Ai 10x10 0.15",|b| b.iter(|| bench_alpha_ai_noui(black_box(10), black_box(10), 0.2f64)));
    //c.bench_function("Alpha Ai 10x10 0.2",|b| b.iter(|| bench_alpha_ai_noui(black_box(10), black_box(10), 0.2f64)));
    //c.bench_function("Alpha Ai 10x10 0.5",|b| b.iter(|| bench_alpha_ai_noui(black_box(10), black_box(10), 0.2f64)));
    c.bench_function("Alpha Ai 25x25 0.1",|b| b.iter(|| bench_alpha_ai_noui(black_box(25), black_box(25), 0.1f64)));
    c.bench_function("Alpha Ai 100x100 0.1",|b| b.iter(|| bench_alpha_ai_noui(black_box(100), black_box(100), 0.1f64)));
    let mut group = c.benchmark_group("large");
    group.sample_size(10);
    group.bench_function("Alpha Ai 400x400 0.1",|b| b.iter(|| bench_alpha_ai_noui(black_box(400), black_box(400), 0.1f64)));
    // c.bench_function("Alpha Ai 25x25 0.2",|b| b.iter(|| bench_alpha_ai_noui(black_box(25), black_box(25), 0.2f64)));
    // c.bench_function("Alpha Ai 100x100 0.2",|b| b.iter(|| bench_alpha_ai_noui(black_box(100), black_box(100), 0.2f64)));
    // c.bench_function("Alpha Ai 400x400 0.2",|b| b.iter(|| bench_alpha_ai_noui(black_box(400), black_box(400), 0.2f64)));
    group.finish();
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);