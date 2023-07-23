mod optimization;

use optimization::{QueensState, hill_climbing, hill_climbing_with_slide_mov};
use crate::optimization::{OptimizationProblem, simulated_annealing, basic_scheduler};

fn main() {
    //let mut qs = QueensState::from_file("resources/queens2.txt").unwrap();
    let mut qs = QueensState::new_random(8);
    println!("{}", qs);
    println!("{}", qs.cost_function());
    println!("{}", qs.next_states().len());

    let mut best = simulated_annealing(qs, basic_scheduler);
    println!("{}", best);
    println!("{}", best.cost_function());
}