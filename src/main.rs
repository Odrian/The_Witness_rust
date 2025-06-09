use the_witness::gui::{EditorApp, SolverApp};
use the_witness::puzzle_logic::Puzzle;

fn main() -> eframe::Result {
    let mut puzzle = Puzzle::default();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "The Witness",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(SolverApp::new(cc, &mut puzzle)))
        }),
    )
}
