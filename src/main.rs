extern crate rand;
use std::collections::HashSet;
use tokio::task;

const ROWS: usize = 20;
const COLS: usize = 50;
const TOTAL_TREES: usize = ROWS * COLS;
const SIMULATIONS: usize = 1_000_000;
const THRESHOLD: usize = (0.3 * TOTAL_TREES as f64) as usize;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum TreeState {
    Unburned,
    Burning,
    Burned,
}

async fn simulate_fire() -> usize {
    let mut forest = vec![vec![TreeState::Unburned; COLS]; ROWS];
    forest[0][0] = TreeState::Burning;

    let mut burning_trees = HashSet::new();
    let mut new_burning_trees = HashSet::new();
    burning_trees.insert((0, 0));

    while !burning_trees.is_empty() {
        new_burning_trees.clear();
        for &(r, c) in &burning_trees {
            forest[r][c] = TreeState::Burned;
            if c > 0 && forest[r][c - 1] == TreeState::Unburned && rand::random::<f32>() <= 0.3 {
                new_burning_trees.insert((r, c - 1));
                forest[r][c - 1] = TreeState::Burning;
            }
            if c < COLS - 1
                && forest[r][c + 1] == TreeState::Unburned
                && rand::random::<f32>() <= 0.8
            {
                new_burning_trees.insert((r, c + 1));
                forest[r][c + 1] = TreeState::Burning;
            }
            if r > 0 && forest[r - 1][c] == TreeState::Unburned && rand::random::<f32>() <= 0.3 {
                new_burning_trees.insert((r - 1, c));
                forest[r - 1][c] = TreeState::Burning;
            }
            if r < ROWS - 1
                && forest[r + 1][c] == TreeState::Unburned
                && rand::random::<f32>() <= 0.3
            {
                new_burning_trees.insert((r + 1, c));
                forest[r + 1][c] = TreeState::Burning;
            }
        }
        std::mem::swap(&mut burning_trees, &mut new_burning_trees);
    }

    forest
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&tree| tree == TreeState::Burned)
        .count()
}

#[tokio::main]
async fn main() {
    let mut handles = Vec::new();
    for _ in 0..SIMULATIONS {
        let handle = task::spawn(async { simulate_fire().await });
        handles.push(handle);
    }

    let mut over_threshold_count = 0;
    for handle in handles {
        let burned_trees = handle.await.unwrap();
        if burned_trees > THRESHOLD {
            over_threshold_count += 1;
        }
    }

    let probability_estimate = over_threshold_count as f64 / SIMULATIONS as f64;
    println!("Estimated probability: {}", probability_estimate);
}
