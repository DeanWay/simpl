use crate::dictionary::PICKABLE_WORDS;
use crate::game::WordleGame;
use crate::solver::WordleSolver;
use crate::solver_strategy::narrowing_random::NarrowingRandomWordleSolver;
use crate::types::GameCondition;

pub fn run_solver() {
    let dictionary: Vec<&str> = PICKABLE_WORDS.to_vec();
    let mut win_total = 0;
    let games = 10000;
    let mut num_guesses = Vec::<usize>::with_capacity(games);
    for _ in 0..games {
        let mut strategy = NarrowingRandomWordleSolver::new(&dictionary);
        let mut game = WordleGame::new_with_random_secret_word(&dictionary);
        let mut solver = WordleSolver::new(&mut game, &mut strategy);
        let result = solver.run_game();
        if result.result == GameCondition::Win {
            win_total += 1;
        }
        num_guesses.push(result.num_guesses)
    }
    let win_percentage = (win_total as f64 / games as f64) * 100_f64;
    let sum_guesses: usize = num_guesses.iter().sum();
    let avg_guesses: f64 = sum_guesses as f64 / games as f64;
    println!("win_total: {win_total} win percentage: {win_percentage}");
    println!("average_guesses: {avg_guesses}");
}
