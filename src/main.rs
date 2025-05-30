use the_witness::gui::SolverApp;
use the_witness::puzzle_logic::Puzzle;

fn main() -> eframe::Result {
    let puzzle = Puzzle::default();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "The Witness",
        options,
        Box::new(|cc| Ok(Box::new(SolverApp::new(cc, &puzzle)))),
    )
}
