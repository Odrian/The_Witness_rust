mod puzzle;
mod solution_checker;
mod solution_manager;

pub use puzzle::{ComplexityColor, DotComplexity, LineComplexity, PaneComplexity};
pub use puzzle::{Dot, DotIndex, LineIndex, PaneIndex, Puzzle};
pub use solution_checker::{SolutionError, check_solution};
pub use solution_manager::PuzzleSolutionManager;

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn find_random_path() {
        let mut rng = rand::rng();

        let puzzle = Puzzle::default();
        let mut puzzle_manager = PuzzleSolutionManager::new(&puzzle);
        while !puzzle_manager.is_solving() {
            let x = rng.random();
            let y = rng.random();
            puzzle_manager.click((x, y));
        }
        while !puzzle
            .end_dots
            .contains(puzzle_manager.dot_path().last().unwrap())
        {
            let delta = 0.02;
            let x: f32 = rng.random();
            let y: f32 = rng.random();
            puzzle_manager.update_mouse((x * delta, y * delta));
        }
        let correct_solution = check_solution(&puzzle_manager);
        assert!(correct_solution.is_ok());
    }
}
