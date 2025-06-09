use crate::puzzle_logic::*;
use eframe::egui::{self, Color32, Pos2, Rect, Vec2};

const START_DOT_SCALE: f32 = 3.0;
const PANE_SCALE: f32 = 2.0;

pub struct EguiDrawer {
    now_pos: Option<Pos2>,
    last_pos: Option<Pos2>,
    delta_pos: Option<Vec2>,
    clicked: bool,
    draw_rect: Rect,
}

impl Default for EguiDrawer {
    fn default() -> Self {
        Self {
            now_pos: None,
            last_pos: None,
            delta_pos: None,
            clicked: false,
            draw_rect: Rect::ZERO,
        }
    }
}

impl EguiDrawer {
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
impl EguiDrawer {
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
    pub fn get_line_width(&self, puzzle: &Puzzle) -> f32 {
        puzzle.line_width * self.draw_rect.width()
    }
    pub fn draw_puzzle(&self, ui: &mut egui::Ui, puzzle: &Puzzle) {
        let color = puzzle.puzzle_color;
        let width = puzzle.line_width * self.draw_rect.width();

        let get_dot = |dot: DotIndex| puzzle.dots[dot.0 as usize];

        for &dot in &puzzle.dots {
            self.draw_dot(ui, dot, width, color);
        }
        for &start_dot in &puzzle.start_dots {
            let dot = get_dot(start_dot);
            self.draw_dot(ui, dot, width * START_DOT_SCALE, color);
        }

        for &line in &puzzle.lines {
            let dot1 = get_dot(line.0);
            let dot2 = get_dot(line.1);
            self.draw_line(ui, (dot1, dot2), width, color);
        }

        for (&dot_index, &dot_complexity) in &puzzle.dot_complexity {
            let dot = get_dot(dot_index);
            match dot_complexity {
                DotComplexity::BlackHexagon => self.draw_hexagon_dot(ui, dot, width),
            };
        }

        for (&line_index, &line_complexity) in &puzzle.line_complexity {
            let dot1 = get_dot(line_index.0);
            let dot2 = get_dot(line_index.1);
            let dot = Dot {
                x: (dot1.x + dot2.x) / 2.0,
                y: (dot1.y + dot2.y) / 2.0,
            };
            match line_complexity {
                LineComplexity::LineBreak => self.draw_line_break_dot(ui, (dot1, dot2), width, puzzle.background_color),
                LineComplexity::BlackHexagon => self.draw_hexagon_dot(ui, dot, width),
            };
        }

        let width = width * PANE_SCALE;
        for (&pane_index, &pane_complexity) in &puzzle.pane_complexity {
            let dot = puzzle.panes[pane_index.0 as usize];
            match pane_complexity {
                PaneComplexity::Square(color) => self.draw_square_dot(ui, dot, width, color),
            };
        }
    }
    pub fn draw_path(&self, ui: &mut egui::Ui, puzzle: &Puzzle, solution_manager: &PuzzleSolutionManager) {
        let color = puzzle.solution_color;
        let width = puzzle.line_width * self.draw_rect.width();

        if solution_manager.is_drawing_solution() {
            let start_dot = solution_manager.get_start_dot_dot_draw();
            self.draw_dot(ui, start_dot, width * START_DOT_SCALE, color);

            for &(dot1, dot2) in &solution_manager.get_lines_to_draw() {
                self.draw_dot(ui, dot1, width, color);
                self.draw_dot(ui, dot2, width, color);
                self.draw_line(ui, (dot1, dot2), width, color);
            }
        }
    }
    pub fn draw_dot(&self, ui: &mut egui::Ui, dot: Dot, width: f32, color: Color32) {
        let radius = width / 2.0;

        let pos = self.get_point(dot);
        ui.painter().circle_filled(pos, radius, color);
    }
    fn draw_line(&self, ui: &mut egui::Ui, line: (Dot, Dot), width: f32, color: Color32) {
        let stroke = egui::Stroke::new(width, color);
        let pos1 = self.get_point(line.0);
        let pos2 = self.get_point(line.1);
        ui.painter().line_segment([pos1, pos2], stroke);
    }
    fn draw_line_break_dot(&self, ui: &mut egui::Ui, line: (Dot, Dot), width: f32, color: Color32) {
        self.draw_line_break(ui, (self.get_point(line.0), self.get_point(line.1)), width, color);
    }
    pub fn draw_line_break(&self, ui: &mut egui::Ui, line: (Pos2, Pos2), width: f32, color: Color32) {
        let scale = LINE_BREAK_WIDTH;
        let width = width + 2.0;

        let delta_pos = line.1.to_vec2() - line.0.to_vec2();
        let pos1 = line.0 + delta_pos * scale;
        let pos2 = line.0 + delta_pos * (1.0 - scale);
        let stroke = egui::Stroke::new(width, color);
        ui.painter().line_segment([pos1, pos2], stroke);
    }

    fn draw_hexagon_dot(&self, ui: &mut egui::Ui, dot: Dot, width: f32) {
        self.draw_hexagon(ui, self.get_point(dot), width);
    }
    pub fn draw_hexagon(&self, ui: &mut egui::Ui, pos: Pos2, width: f32) {
        let rect = Rect::from_center_size(pos, Vec2::new(width * 0.9, width * 0.8));
        let image = egui::Image::new(egui::include_image!("../../assets/hexagon.png"));
        image.paint_at(ui, rect);
    }

    fn draw_square_dot(&self, ui: &mut egui::Ui, dot: Dot, width: f32, color: ComplexityColor) {
        self.draw_square(ui, self.get_point(dot), width, color);
    }
    pub fn draw_square(&self, ui: &mut egui::Ui, pos: Pos2, width: f32, color: ComplexityColor) {
        let color = self.convert_color(color);
        let rect = Rect::from_center_size(pos, Vec2::splat(width));
        let corner_radius = width / 3.0;
        ui.painter().rect_filled(rect, corner_radius, color);
    }
}
