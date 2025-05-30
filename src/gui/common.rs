use crate::puzzle_logic::{ComplexityColor, Dot, DotComplexity};
use eframe::egui::{self, Stroke};
use eframe::egui::{Color32, Pos2, Rect, Vec2};

pub struct EguiDrawer {
    pub now_pos: Option<Pos2>,
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
    fn convert_color(&self, color: ComplexityColor) -> Color32 {
        match color {
            ComplexityColor::White => Color32::WHITE,
            ComplexityColor::Black => Color32::BLACK,
        }
    }
    pub fn draw_dot(
        &self,
        ui: &mut egui::Ui,
        dot: Dot,
        line_width: f32,
        color: Color32,
        start_dot: bool,
    ) {
        let mut radius = line_width / 2.0;
        if start_dot {
            radius *= 3.0;
        }
        let stroke = Stroke::NONE;

        let pos = self.get_point(dot);
        // ui.painter().circle_filled(pos, radius, color);
        ui.painter().circle(pos, radius, color, stroke);
    }
    pub fn draw_line(
        &self,
        ui: &mut egui::Ui,
        dot1: Dot,
        dot2: Dot,
        line_width: f32,
        color: Color32,
    ) {
        let stroke = egui::Stroke::new(line_width, color);
        let pos1 = self.get_point(dot1);
        let pos2 = self.get_point(dot2);
        ui.painter().line_segment([pos1, pos2], stroke);
    }
    pub fn draw_dot_complexity(
        &self,
        ui: &mut egui::Ui,
        dot: Dot,
        dot_complexity: DotComplexity,
        line_width: f32,
    ) {
        let pos = self.get_point(dot);
        match dot_complexity {
            DotComplexity::BlackHexagon => ui.painter().circle_filled(
                pos,
                line_width * 0.5,
                self.convert_color(ComplexityColor::Black),
            ),
        };
    }
}
