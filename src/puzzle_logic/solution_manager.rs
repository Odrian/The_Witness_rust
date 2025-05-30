use super::*;
use std::cmp;

fn compare_dots(dot0: Dot, dot1: &Dot, dot2: &Dot, delta: (f32, f32)) -> cmp::Ordering {
    let dot1 = (dot1.x - dot0.x, dot1.y - dot0.y);
    let dot2 = (dot2.x - dot0.x, dot2.y - dot0.y);
    let angle0 = (delta.1).atan2(delta.0);
    let pi = std::f32::consts::PI;
    let mut angle1 = ((dot1.1).atan2(dot1.0) - angle0).abs();
    if angle1 > pi {
        angle1 = 2.0 * pi - angle1;
    }
    let mut angle2 = ((dot2.1).atan2(dot2.0) - angle0).abs();
    if angle2 > pi {
        angle2 = 2.0 * pi - angle2;
    }
    angle1
        .partial_cmp(&angle2)
        .expect("don't expect lines to degenerate")
}

pub struct PuzzleSolutionManager<'a> {
    puzzle: &'a Puzzle,
    dot_path: Vec<DotIndex>,
    line_path: Vec<LineIndex>,
    now_at_dot: bool,
    line_progress: f32,
    is_solving: bool,
    is_drawing_solution: bool,
}

impl<'a> PuzzleSolutionManager<'a> {
    pub fn new(puzzle: &'a Puzzle) -> Self {
        Self {
            puzzle,
            dot_path: Vec::new(),
            line_path: Vec::new(),
            now_at_dot: false,
            line_progress: 0.0,
            is_solving: false,
            is_drawing_solution: false,
        }
    }

    pub fn is_solving(&self) -> bool {
        self.is_solving
    }
    pub fn is_drawing_solution(&self) -> bool {
        self.is_drawing_solution
    }

    pub fn get_start_dot_dot_draw(&self) -> Dot {
        if !self.is_drawing_solution {
            panic!("not drawing now")
        }
        self.get_dot(self.dot_path[0])
    }
    pub fn get_lines_to_draw(&self) -> Vec<(Dot, Dot)> {
        if !self.is_drawing_solution {
            panic!("not drawing now")
        }
        let line_to_dots =
            |line: &LineIndex| -> (Dot, Dot) { (self.get_dot(line.0), self.get_dot(line.1)) };
        if self.now_at_dot {
            self.line_path.iter().map(line_to_dots).collect()
        } else {
            let (last_line, lines) = self
                .line_path
                .split_last()
                .expect("line_path can't be empty while on line");

            let mut lines: Vec<(Dot, Dot)> = lines.iter().map(line_to_dots).collect();

            let dot1 = self.get_dot(last_line.0);
            let dot2 = self.get_dot(last_line.1);

            if last_line.0 == self.last_dot() {
                let dot2 = dot1.interp(&dot2, &self.line_progress);
                lines.push((dot1, dot2));
            } else {
                let dot1 = dot1.interp(&dot2, &self.line_progress);
                lines.push((dot1, dot2));
            }
            lines
        }
    }

    pub fn puzzle(&self) -> &'a Puzzle {
        self.puzzle
    }
    pub fn dot_path(&self) -> &Vec<DotIndex> {
        &self.dot_path
    }
    pub fn line_path(&self) -> &Vec<LineIndex> {
        &self.line_path
    }
    pub fn now_at_dot(&self) -> bool {
        self.now_at_dot
    }
}

impl PuzzleSolutionManager<'_> {
    fn get_dot(&self, dot: DotIndex) -> Dot {
        self.puzzle.dots[dot.0 as usize]
    }

    fn clear(&mut self) {
        self.dot_path.clear();
        self.line_path.clear();
        self.is_solving = false;
        self.is_drawing_solution = false;
    }
    fn start_from(&mut self, start_dot: DotIndex) {
        if !self.puzzle.start_dots.contains(&start_dot) {
            panic!("start_dot {start_dot:?} doesn't exist in puzzle")
        }
        self.clear();
        self.is_drawing_solution = true;
        self.is_solving = true;
        self.dot_path.push(start_dot);
        self.now_at_dot = true;
    }

    fn move_to_dot(&mut self, dot: DotIndex) {
        if !self.is_solving {
            panic!("not solving now");
        }
        if self.now_at_dot {
            panic!("can't move from line to dot, while you at dot")
        }

        let last_line = self.last_line_while_at_line();
        if !last_line.contains(dot) {
            panic!("can't move to dot {dot:?} from line {last_line:?}")
        }

        let last_dot = self.dot_path.last().expect("dot_path is never empty");
        if dot == *last_dot {
            self.line_path.pop();
        } else {
            self.dot_path.push(dot);
        }
        self.now_at_dot = true;
    }

    fn move_to_line(&mut self, line: LineIndex) {
        if !self.is_solving {
            panic!("not solving now");
        }
        if !self.now_at_dot {
            panic!("can't move from dot to line, while you at line")
        }

        let last_dot = self.last_dot();
        if !line.contains(last_dot) {
            panic!("can't move to line {line:?} from dot {last_dot:?}")
        }

        let last_line_option = self.line_path.last();
        if let Some(last_line) = last_line_option {
            if *last_line == line {
                self.dot_path.pop();
            } else {
                self.line_path.push(line);
            }
        } else {
            self.line_path.push(line);
        }
        self.now_at_dot = false;
    }

    fn last_dot(&self) -> DotIndex {
        *self.dot_path.last().expect("dot_path is never empty")
    }
    fn last_line_while_at_line(&self) -> LineIndex {
        if self.now_at_dot {
            panic!("don't use it while on dot");
        }
        *self
            .line_path
            .last()
            .expect("line_path can't be empty while now_at_dot==false")
    }
}

impl PuzzleSolutionManager<'_> {
    /// returns 'is_solving'
    pub fn click(&mut self, mouse_pos: (f32, f32)) -> bool {
        if self.is_solving {
            match check_solution(self) {
                Ok(()) => {
                    self.is_solving = false;
                    println!("Correct solution!")
                }
                Err(err) => {
                    self.clear();
                    println!("Incorrect: {err:?}")
                }
            }
            false
        } else {
            // try start solving
            for &dot_index in &self.puzzle.start_dots {
                let dot = self.get_dot(dot_index);
                let dist2 = {
                    let dx = dot.x - mouse_pos.0;
                    let dy = dot.y - mouse_pos.1;
                    dx * dx + dy * dy
                };
                let start_dot_radius = 0.045; // TODO: dont use magic value
                if dist2 <= start_dot_radius * start_dot_radius {
                    self.start_from(dot_index);
                    return true;
                }
            }
            false
        }
    }

    pub fn update_mouse(&mut self, delta: (f32, f32)) {
        if !self.is_solving {
            return;
        }
        if delta.0.abs() < f32::EPSILON && delta.1.abs() < f32::EPSILON {
            return;
        }
        if self.now_at_dot {
            let dot_ind = self.last_dot();
            let dot = self.get_dot(dot_ind);

            let line_get_not = |line: LineIndex| DotIndex(line.0.0 + line.1.0 - dot_ind.0);
            let (&near_line, _) = self
                .puzzle
                .lines
                .iter()
                .filter(|line| line.contains(dot_ind)) // get line from 'dot'
                .map(|line| (line, self.get_dot(line_get_not(*line)))) // get second Dot
                .min_by(|(_, dot1), (_, dot2)| compare_dots(dot, dot1, dot2, delta)) // get nearest to delta vector
                .expect("dot {dot} don't have line from it");

            let scalar = {
                let dot2 = self.get_dot(line_get_not(near_line));
                (dot2.x - dot.x) * delta.0 + (dot2.y - dot.y) * delta.1
            };
            if scalar > 0.0 {
                self.move_to_line(near_line);
                self.line_progress = if near_line.0 == dot_ind { 0.0 } else { 1.0 };
                self.update_mouse(delta);
            }
        } else {
            let line = self.last_line_while_at_line();
            let (dx, dy) = {
                let dot1 = self.get_dot(line.0);
                let dot2 = self.get_dot(line.1);
                (dot2.x - dot1.x, dot2.y - dot1.y)
            };

            let proj = {
                let scalar = delta.0 * dx + delta.1 * dy;
                let length = (dx * dx + dy * dy).sqrt();
                scalar / length / length
            };

            if proj > 0.0 {
                if self.line_progress + proj > 1.0 {
                    self.move_to_dot(line.1);
                    let scale = 1.0 - (1.0 - self.line_progress) / proj;
                    self.update_mouse((delta.0 * scale, delta.1 * scale));
                } else {
                    self.line_progress += proj;
                }
            } else {
                if self.line_progress + proj < 0.0 {
                    self.move_to_dot(line.0);
                    let scale = 1.0 - self.line_progress / (-proj);
                    self.update_mouse((delta.0 * scale, delta.1 * scale));
                } else {
                    self.line_progress += proj;
                }
            }
        }
    }
}
