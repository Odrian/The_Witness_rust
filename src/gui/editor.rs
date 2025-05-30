use super::EguiDrawer;
use crate::puzzle_logic::*;
use eframe::egui::{self, Pos2};

enum SelectedObject {
    None,
    Dot(DotIndex),
    Line(LineIndex),
    Pane(PaneIndex),
}

enum SelectedComplexity {
    None,
    Hexagon,
    Square,
    // Star,
    // Jack,
    // Triangle,
}

pub struct EditorApp<'a> {
    puzzle: &'a mut Puzzle,
    drawer: EguiDrawer,

    selected_object: SelectedObject,
    selected_complexity: SelectedComplexity,
    selected_color: ComplexityColor,
}

impl eframe::App for EditorApp<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.drawer.update(ctx);

        if let Some(pos) = self.drawer.get_mouse_pos() {
            self.update_selection(pos);
        }
        if self.drawer.clicked() {
            self.click();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.painter().rect_filled(
                ctx.screen_rect(),
                egui::CornerRadius::ZERO,
                self.puzzle.background_color,
            );
            self.render(ui);
            self.render_ui(ui);

            ctx.request_repaint();
        });
    }
}

impl<'a> EditorApp<'a> {
    pub fn new(_cc: &eframe::CreationContext<'_>, puzzle: &'a mut Puzzle) -> Self {
        Self {
            puzzle,
            drawer: EguiDrawer::default(),

            selected_object: SelectedObject::None,
            selected_complexity: SelectedComplexity::None,
            selected_color: ComplexityColor::Black,
        }
    }
    fn get_dot(&self, dot_index: DotIndex) -> Dot {
        self.puzzle.dots[dot_index.0 as usize]
    }
    fn update_selection(&mut self, pos: Pos2) {
        let mouse_dot = Dot { x: pos.x, y: pos.y };
        // Dot
        let dot_radius = self.puzzle.line_width / 2.0;
        for (i, dot) in self.puzzle.dots.iter().enumerate() {
            let dist = mouse_dot.dist(dot);
            if dist < dot_radius {
                self.selected_object = SelectedObject::Dot(DotIndex(i as u16));
                return;
            }
        }
        // Line
        for line_index in &self.puzzle.lines {
            let &LineIndex(dot1, dot2) = line_index;
            let dot1 = self.get_dot(dot1);
            let dot2 = self.get_dot(dot2);
            let dist = {
                let ab = Dot {
                    x: dot2.x - dot1.x,
                    y: dot2.y - dot1.y,
                };
                let ap = Dot {
                    x: mouse_dot.x - dot1.x,
                    y: mouse_dot.y - dot1.y,
                };

                let ab_len_sq = ab.x * ab.x + ab.y * ab.y;
                let t = (ap.x * ab.x + ap.y * ab.y) / ab_len_sq;
                if !(0.0..=1.0).contains(&t) {
                    continue;
                }

                let proj = Dot {
                    x: dot1.x + ab.x * t,
                    y: dot1.y + ab.y * t,
                };

                let dx = mouse_dot.x - proj.x;
                let dy = mouse_dot.y - proj.y;

                (dx * dx + dy * dy).sqrt()
            };
            if dist < dot_radius {
                self.selected_object = SelectedObject::Line(*line_index);
                return;
            }
        }
        // Pane
        let pane_radius = self.puzzle.cell_size / 2.0;
        for (i, dot) in self.puzzle.panes.iter().enumerate() {
            let dist = {
                let dx = (mouse_dot.x - dot.x).abs();
                let dy = (mouse_dot.y - dot.y).abs();
                dx.max(dy)
            };
            if dist < pane_radius {
                self.selected_object = SelectedObject::Pane(PaneIndex(i as u16));
                return;
            }
        }
        self.selected_object = SelectedObject::None;
    }
    fn click(&mut self) {
        match self.selected_complexity {
            SelectedComplexity::None => {}
            SelectedComplexity::Hexagon => match self.selected_object {
                SelectedObject::None => {}
                SelectedObject::Dot(key) => {
                    let map = &mut self.puzzle.dot_complexity;
                    if map.remove(&key).is_none() {
                        map.insert(key, DotComplexity::BlackHexagon);
                    }
                }
                SelectedObject::Line(key) => {
                    let map = &mut self.puzzle.line_complexity;
                    if map.remove(&key).is_none() {
                        map.insert(key, LineComplexity::BlackHexagon);
                    }
                }
                SelectedObject::Pane(_) => {}
            },
            SelectedComplexity::Square => match self.selected_object {
                SelectedObject::None => {}
                SelectedObject::Dot(_) => {}
                SelectedObject::Line(_) => {}
                SelectedObject::Pane(key) => {
                    let map = &mut self.puzzle.pane_complexity;
                    if map.remove(&key).is_none() {
                        map.insert(key, PaneComplexity::Square(self.selected_color));
                    }
                }
            },
        }
    }
    fn render_ui(&self, ui: &mut egui::Ui) {}
    fn render(&self, ui: &mut egui::Ui) {
        for &dot in &self.puzzle.dots {
            self.drawer.draw_dot(ui, self.puzzle, dot, false, false);
        }
        for &start_dot in &self.puzzle.start_dots {
            let dot = self.get_dot(start_dot);
            self.drawer.draw_dot(ui, self.puzzle, dot, false, true);
        }

        for &line in &self.puzzle.lines {
            let dot1 = self.get_dot(line.0);
            let dot2 = self.get_dot(line.1);
            self.drawer.draw_line(ui, self.puzzle, dot1, dot2, false);
        }

        for (&dot_index, &dot_complexity) in &self.puzzle.dot_complexity {
            let dot = self.get_dot(dot_index);
            self.drawer
                .draw_dot_complexity(ui, self.puzzle, dot, dot_complexity);
        }

        match self.selected_object {
            SelectedObject::None => {}
            SelectedObject::Dot(dot_index) => {
                let dot = self.get_dot(dot_index);
                self.drawer.draw_dot(ui, self.puzzle, dot, true, false);
            }
            SelectedObject::Line(line_index) => {
                let dot1 = self.get_dot(line_index.0);
                let dot2 = self.get_dot(line_index.1);
                let dot = Dot {
                    x: (dot1.x + dot2.x) / 2.0,
                    y: (dot1.y + dot2.y) / 2.0,
                };
                self.drawer.draw_dot(ui, self.puzzle, dot, true, false);
            }
            SelectedObject::Pane(pane_index) => {
                let dot = self.puzzle.panes[pane_index.0 as usize];
                self.drawer.draw_dot(ui, self.puzzle, dot, true, false);
            }
        }
    }
}
