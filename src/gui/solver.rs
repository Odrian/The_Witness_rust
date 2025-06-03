use super::EguiDrawer;
use crate::puzzle_logic::*;
use eframe::egui;

pub struct SolverApp<'a> {
    puzzle: &'a Puzzle,
    solution_manager: PuzzleSolutionManager<'a>,
    drawer: EguiDrawer,
    is_grabbing_cursor: bool,
}

impl eframe::App for SolverApp<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.drawer.update(ctx);

        if let Some(delta) = self.drawer.get_mouse_delta() {
            self.solution_manager.update_mouse((delta.x, delta.y));
        }
        if self.drawer.clicked() {
            if let Some(pos) = self.drawer.get_mouse_pos() {
                let is_solving = self.solution_manager.click((pos.x, pos.y));
                self.is_grabbing_cursor = is_solving;

                let cursor_grab = if is_solving {
                    egui::CursorGrab::Confined
                } else {
                    egui::CursorGrab::None
                };
                ctx.send_viewport_cmd(egui::ViewportCommand::CursorGrab(cursor_grab));
            }
        }

        let cursor_icon = if self.is_grabbing_cursor {
            egui::CursorIcon::None
        } else {
            egui::CursorIcon::Default
        };
        ctx.set_cursor_icon(cursor_icon);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.painter().rect_filled(
                ctx.screen_rect(),
                egui::CornerRadius::ZERO,
                self.puzzle.background_color,
            );
            self.drawer.draw_puzzle(ui, self.puzzle);
            self.drawer.draw_path(ui, self.puzzle, &self.solution_manager);

            ctx.request_repaint();
        });
    }
}

impl<'a> SolverApp<'a> {
    pub fn new(_cc: &eframe::CreationContext<'_>, puzzle: &'a Puzzle) -> Self {
        Self {
            puzzle,
            solution_manager: PuzzleSolutionManager::new(puzzle),
            drawer: EguiDrawer::new(),
            is_grabbing_cursor: false,
        }
    }
}
