use crate::puzzle_logic::*;
use eframe::egui::{self, Stroke};
use eframe::egui::{Color32, Pos2, Rect, Vec2};

pub struct EguiDrawer<'a> {
    puzzle: &'a Puzzle

    now_pos: Option<Pos2>,
    last_pos: Option<Pos2>,
    delta_pos: Option<Vec2>,
    clicked: bool,
    draw_rect: Rect,
}

impl<'a> EguiDrawer<'a> {
   pub fn new(puzzle: &'a Puzzle) -> Self {
        Self {
            puzzle,
            now_pos: None,
            last_pos: None,
            delta_pos: None,
            clicked: false,
            draw_rect: Rect::ZERO,
        }
    }
}

impl EguiDrawer<'_> {
    pub fn update(&mut self, ctx: &egui::Context) {
        self.draw_rect = {
            let screen_rect = ctx.screen_rect();
            let size = {
                let Vec2 { x, y } = screen_rect.size();
                x.min(y)
            };
            let center = screen_rect.center();
            Rect::from_center_size(center, Vec2::splat(size))
        };

        self.last_pos = self.now_pos;
        self.now_pos = {
            let pos = ctx.input(|i| i.pointer.hover_pos());
            if let Some(Pos2 { x, y }) = pos {
                let x = (x - self.draw_rect.left()) / self.draw_rect.width();
                let y = 1.0 - (y - self.draw_rect.top()) / self.draw_rect.height();
                Some(Pos2 { x, y })
            } else {
                None
            }
        };
        if self.now_pos.is_some() && self.last_pos.is_some() {
            self.delta_pos = Some(self.now_pos.unwrap() - self.last_pos.unwrap());
        } else {
            self.delta_pos = None;
        }
        self.clicked = ctx.input(|i| i.pointer.button_pressed(egui::PointerButton::Primary));
    }

    pub fn get_mouse_pos(&self) -> Option<Pos2> {
        self.now_pos
    }
    pub fn get_mouse_delta(&self) -> Option<Vec2> {
        self.delta_pos
    }
    pub fn clicked(&self) -> bool {
        self.clicked
    }
}
impl EguiDrawer<'_> {
    fn get_dot(&self, dot: DotIndex) -> Dot {
        self.puzzle.dots[dot.0 as usize]
    }
    fn get_point(&self, dot: Dot) -> Pos2 {
        let Dot { x, y } = dot;
        Pos2 {
            x: self.draw_rect.left() + x * self.draw_rect.width(),
            y: self.draw_rect.bottom() - y * self.draw_rect.height(),
        }
    }
    pub fn convert_color(&self, color: ComplexityColor) -> Color32 {
        match color {
            ComplexityColor::White => Color32::WHITE,
            ComplexityColor::Black => Color32::BLACK,
        }
    }
    pub fn draw_puzzle(&self, ui: &mut egui::Ui) {
        let color = self.puzzle.puzzle_color;
        let start_dot_scale = 3.0;
        for &dot in &self.puzzle.dots {
            self.draw_dot(ui, dot, 1.0, color);
        }
        for &start_dot in &self.puzzle.start_dots {
            let dot = self.get_dot(start_dot);
            self.draw_dot(ui, dot, start_dot_scale, color);
        }

        for &line in &self.puzzle.lines {
            let dot1 = self.get_dot(line.0);
            let dot2 = self.get_dot(line.1);
            self.draw_line(ui, (dot1, dot2), color);
        }

        for (&dot_index, &dot_complexity) in &self.puzzle.dot_complexity {
            let dot = self.get_dot(dot_index);
            match dot_complexity {
                DotComplexity::BlackHexagon => self.draw_hexagon(ui, dot),
            };
        }

        for (&line_index, &line_complexity) in &self.puzzle.line_complexity {
            let dot1 = self.get_dot(line_index.0);
            let dot2 = self.get_dot(line_index.1);
            let dot = Dot { x: (dot1.x + dot2.x) / 2.0, y: (dot1.y + dot2.y) / 2.0 };
            match line_complexity {
                LineComplexity::BlackHexagon => self.draw_hexagon(ui, dot),
            };
        }

        for (&pane_index, &pane_complexity) in &self.puzzle.pane_complexity {
            let dot = self.puzzle.panes[pane_index.0 as usize];
            match pane_complexity {
                PaneComplexity::Square(color) => self.draw_square(ui, dot, color),
            };
        }
    }
    pub fn draw_path(&self, ui: &mut egui::Ui, solution_manager: &PuzzleSolutionManager) {
        let color = self.puzzle.solution_color;
        let start_dot_scale = 3.0;

        if solution_manager.is_drawing_solution() {
            let start_dot = solution_manager.get_start_dot_dot_draw();
            self.draw_dot(ui, start_dot, start_dot_scale, color);

            for &(dot1, dot2) in &solution_manager.get_lines_to_draw() {
                self.draw_dot(ui, dot1, 1.0, color);
                self.draw_dot(ui, dot2, 1.0, color);
                self.draw_line(ui, (dot1, dot2), color);
            }
        }
    }
    pub fn draw_dot(&self, ui: &mut egui::Ui, dot: Dot, scale: f32, color: Color32) {
        let line_width = self.puzzle.line_width * self.draw_rect.width();
        let radius = line_width / 2.0 * scale;

        let pos = self.get_point(dot);
        ui.painter().circle_filled(pos, radius, color);
    }
    pub fn draw_line(&self, ui: &mut egui::Ui, line: (Dot, Dot), color: Color32) {
        let line_width = self.puzzle.line_width * self.draw_rect.width();
        let stroke = egui::Stroke::new(line_width, color);
        let pos1 = self.get_point(line.0);
        let pos2 = self.get_point(line.1);
        ui.painter().line_segment([pos1, pos2], stroke);
    }
    pub fn draw_hexagon(&self, ui: &mut egui::Ui, dot: Dot) {
        let line_width = self.puzzle.line_width * self.draw_rect.width();
        let pos = self.get_point(dot);
        let color = self.convert_color(ComplexityColor::Black);
        ui.painter().circle_filled(pos, line_width * 0.5, color);
    }
    pub fn draw_square(&self, ui: &mut egui::Ui, dot: Dot, color: ComplexityColor) {
        let line_width = self.puzzle.line_width * self.draw_rect.width();
        let pos = self.get_point(dot);
        let color = self.convert_color(color);
        ui.painter().circle_filled(pos, line_width, color);
    }
}
