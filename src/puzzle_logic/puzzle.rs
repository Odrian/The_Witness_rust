use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct Dot {
    pub x: f32,
    pub y: f32,
}

impl Dot {
    pub fn new(x: f32, y: f32) -> Self {
        Dot { x, y }
    }
    pub fn dist2(&self, other: &Dot) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
    pub fn dist(&self, other: &Dot) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
    pub fn interp(&self, other: &Self, coef: &f32) -> Self {
        let x = self.x + (other.x - self.x) * coef;
        let y = self.y + (other.y - self.y) * coef;
        Self::new(x, y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DotIndex(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LineIndex(pub DotIndex, pub DotIndex);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PaneIndex(pub u16);

use std::fmt::Display;

impl Display for DotIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Display for LineIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line({}, {})", self.0, self.1)
    }
}
impl Display for PaneIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl LineIndex {
    pub fn contains(&self, dot: DotIndex) -> bool {
        self.0 == dot || self.1 == dot
    }
    pub fn get0(&self) -> DotIndex {
        self.0
    }
    pub fn get1(&self) -> DotIndex {
        self.1
    }
}

#[derive(Clone, Copy)]
pub enum DotComplexity {
    BlackHexagon,
}

#[derive(Clone, Copy)]
pub enum LineComplexity {
    BlackHexagon,
}

#[derive(Clone, Copy)]
pub enum PaneComplexity {
    Square(ComplexityColor),
    // Star(Color)
    // Block(type)
    // Triangle(num)
    // Jack
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, enum_iterator::Sequence)]
pub enum ComplexityColor {
    Black,
    White,
}

use eframe::egui::Color32;

pub struct Puzzle {
    pub dots: Vec<Dot>,
    pub lines: Vec<LineIndex>,

    pub panes: Vec<Dot>,
    pub cell_size: f32,
    pub pane_nears: Vec<Vec<(LineIndex, PaneIndex)>>,

    pub start_dots: Vec<DotIndex>,
    pub end_dots: Vec<DotIndex>,

    pub dot_complexity: HashMap<DotIndex, DotComplexity>,
    pub line_complexity: HashMap<LineIndex, LineComplexity>,
    pub pane_complexity: HashMap<PaneIndex, PaneComplexity>,

    pub line_width: f32,
    pub background_color: Color32,
    pub puzzle_color: Color32,
    pub solution_color: Color32,
}

impl Default for Puzzle {
    fn default() -> Self {
        let n: usize = 5;
        let padding = 1.0;
        let endline_length = 0.5;
        let size = padding * 2.0 + (n - 1) as f32;

        let cell_size = 1.0 / size;

        let mut dots: Vec<Dot> = Vec::new();

        let mut dots_indexes: Vec<Vec<DotIndex>> = Vec::new();
        (0..n).for_each(|_| dots_indexes.push(Vec::new()));

        {
            // create dots
            let mut i = 0;
            for x in 0..n {
                for y in 0..n {
                    let x_float = (padding + x as f32) / size;
                    let y_float = (padding + y as f32) / size;
                    dots.push(Dot::new(x_float, y_float));
                    dots_indexes[x].push(DotIndex(i));
                    i += 1;
                }
            }
        }

        // create lines
        let mut lines: Vec<LineIndex> = Vec::new();
        let mut horizontal_lines: Vec<Vec<LineIndex>> = Vec::new();
        horizontal_lines.resize(n, Vec::new());
        let mut vertical_lines: Vec<Vec<LineIndex>> = Vec::new();
        vertical_lines.resize(n - 1, Vec::new());
        {
            // horizontal lines
            for x in 0..(n - 1) {
                for y in 0..n {
                    let dot1 = dots_indexes[x][y];
                    let dot2 = dots_indexes[x + 1][y];
                    let line = LineIndex(dot1, dot2);
                    lines.push(line);
                    horizontal_lines[y].push(line);
                }
            }
            // vertical lines
            for y in 0..(n - 1) {
                for x in 0..n {
                    let dot1 = dots_indexes[x][y];
                    let dot2 = dots_indexes[x][y + 1];
                    let line = LineIndex(dot1, dot2);
                    lines.push(line);
                    vertical_lines[y].push(line);
                }
            }
        }

        let mut panes: Vec<Dot> = Vec::new();
        let mut pane_nears: Vec<Vec<(LineIndex, PaneIndex)>> = Vec::new();
        let m = n - 1;
        pane_nears.resize(m * m, Vec::new());
        for y in 0..m {
            for x in 0..m {
                let ind = y * m + x;
                let vec = &mut pane_nears[ind];
                {
                    let x = (padding + (x as f32) + 0.5) / size;
                    let y = (padding + (y as f32) + 0.5) / size;
                    panes.push(Dot { x, y });
                }

                if x > 0 {
                    let ind_near = ind - 1; // left
                    vec.push((horizontal_lines[y][x], PaneIndex(ind_near as u16)));
                }
                if y > 0 {
                    let ind_near = ind - m; // down
                    vec.push((vertical_lines[y][x], PaneIndex(ind_near as u16)));
                }
                if x + 1 < m {
                    let ind_near = ind + 1; // right
                    vec.push((horizontal_lines[y][x], PaneIndex(ind_near as u16)));
                }
                if y + 1 < m {
                    let ind_near = ind + m; // up
                    vec.push((vertical_lines[y][x], PaneIndex(ind_near as u16)));
                }
            }
        }

        let start_dots: Vec<DotIndex> = vec![dots_indexes[0][0]];

        let end_dots: Vec<DotIndex> = {
            let end_dot = Dot {
                x: (size - padding + endline_length) / size,
                y: (size - padding) / size,
            };
            dots.push(end_dot);
            let end_dot = DotIndex((dots.len() - 1) as u16);

            lines.push(LineIndex(end_dot, dots_indexes[n - 1][n - 1]));

            vec![end_dot]
        };

        let mut dot_complexity = HashMap::new();
        dot_complexity.insert(DotIndex(2), DotComplexity::BlackHexagon);

        Puzzle {
            dots,
            lines,
            panes,
            cell_size,
            pane_nears,
            start_dots,
            end_dots,
            dot_complexity,
            line_complexity: HashMap::new(),
            pane_complexity: HashMap::new(),

            line_width: 0.035,
            background_color: Color32::from_rgb(228, 165, 0),
            puzzle_color: Color32::from_rgb(61, 46, 3),
            // solution_color: Color32::from_rgb(255, 234, 84),
            solution_color: Color32::from_rgb(255, 255, 255),
        }
    }
}
