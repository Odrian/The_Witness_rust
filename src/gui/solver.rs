use super::EguiDrawer;
use crate::puzzle_logic::*;
use eframe::egui::{self, Color32};

pub struct SolverApp<'a> {
    puzzle: &'a Puzzle,
    solution_manager: PuzzleSolutionManager<'a>,
    drawer: EguiDrawer,
    is_grabbing_cursor: bool,

    line_width: f32,
    background_color: Color32,
    puzzle_color: Color32,
    solution_color: Color32,
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
                self.background_color,
            );
            self.render(ui);

            ctx.request_repaint();
        });
    }
}

impl<'a> SolverApp<'a> {
    pub fn new(_cc: &eframe::CreationContext<'_>, puzzle: &'a Puzzle) -> Self {
        Self {
            puzzle,
            solution_manager: PuzzleSolutionManager::new(puzzle),
            drawer: EguiDrawer::default(),
            is_grabbing_cursor: false,

            line_width: 20.0,
            background_color: Color32::from_rgb(228, 165, 0),
            puzzle_color: Color32::from_rgb(61, 46, 3),
            solution_color: Color32::from_rgb(255, 234, 84),
        }
    }
    fn get_dot(&self, dot_index: DotIndex) -> Dot {
        self.puzzle.dots[dot_index.0 as usize]
    }
    fn render(&self, ui: &mut egui::Ui) {
        for &dot in &self.puzzle.dots {
            self.drawer
                .draw_dot(ui, dot, self.line_width, self.puzzle_color, false);
        }
        for &start_dot in &self.puzzle.start_dots {
            let dot = self.get_dot(start_dot);
            self.drawer
                .draw_dot(ui, dot, self.line_width, self.puzzle_color, true);
        }

        for &line in &self.puzzle.lines {
            let dot1 = self.get_dot(line.0);
            let dot2 = self.get_dot(line.1);
            self.drawer
                .draw_line(ui, dot1, dot2, self.line_width, self.puzzle_color);
        }

        for (&dot_index, &dot_complexity) in &self.puzzle.dot_complexity {
            let dot = self.get_dot(dot_index);
            self.drawer
                .draw_dot_complexity(ui, dot, dot_complexity, self.line_width);
        }

        if self.solution_manager.is_drawing_solution() {
            let start_dot = self.solution_manager.get_start_dot_dot_draw();
            self.drawer
                .draw_dot(ui, start_dot, self.line_width, self.solution_color, true);

            for &(dot1, dot2) in &self.solution_manager.get_lines_to_draw() {
                self.drawer
                    .draw_dot(ui, dot1, self.line_width, self.solution_color, false);
                self.drawer
                    .draw_dot(ui, dot2, self.line_width, self.solution_color, false);
                self.drawer
                    .draw_line(ui, dot1, dot2, self.line_width, self.solution_color);
            }
        }
    }
}
