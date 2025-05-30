use the_witness::gui::{EditorApp, SolverApp};
use the_witness::puzzle_logic::Puzzle;

fn main() -> eframe::Result {
    let mut puzzle = Puzzle::default();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "The Witness",
        options,
        Box::new(|cc| Ok(Box::new(EditorApp::new(cc, &mut puzzle)))),
    )
}
