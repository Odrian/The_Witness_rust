use super::*;
use std::collections::HashMap;

#[derive(Debug)]
pub enum SolutionError {
    Incomplete,
    DotError(DotIndex),
    LineError(LineIndex),
    PaneError,
}

struct SolutionChecker<'a> {
    puzzle: &'a Puzzle,
    dot_path: &'a Vec<DotIndex>,
    line_path: &'a Vec<LineIndex>,
    components: Vec<Vec<PaneIndex>>,
}

pub fn check_solution<'a>(solution: &'a PuzzleSolutionManager<'a>) -> Result<(), SolutionError> {
    let checker = SolutionChecker::new(solution)?;
    checker.check_correctness()?;
    checker.check_dots()?;
    checker.check_lines()?;
    checker.check_panes()?;
    Ok(())
}

impl<'a> SolutionChecker<'a> {
    fn new(solution: &'a PuzzleSolutionManager<'a>) -> Result<Self, SolutionError> {
        if !solution.now_at_dot() {
            return Err(SolutionError::Incomplete);
        }
        Ok(SolutionChecker {
            puzzle: solution.puzzle(),
            dot_path: solution.dot_path(),
            line_path: solution.line_path(),
            components: find_components(solution.puzzle(), solution.line_path()),
        })
    }
    fn check_correctness(&self) -> Result<(), SolutionError> {
        if self.dot_path.is_empty() {
            panic!("Unreachable: dot_path is empty")
        }
        let end_dot = self.dot_path.last().expect("Unreachable");
        if self.puzzle.end_dots.contains(end_dot) {
            Ok(())
        } else {
            Err(SolutionError::Incomplete)
        }
    }
    fn check_dots(&self) -> Result<(), SolutionError> {
        let map = &self.puzzle.dot_complexity;
        for (dot_index, dot_complexity) in map {
            match dot_complexity {
                DotComplexity::BlackHexagon => {
                    if !self.dot_path.contains(dot_index) {
                        return Err(SolutionError::DotError(*dot_index));
                    }
                }
            }
        }
        Ok(())
    }
    fn check_lines(&self) -> Result<(), SolutionError> {
        let map = &self.puzzle.line_complexity;
        for (line_index, line_complexity) in map {
            match line_complexity {
                LineComplexity::BlackHexagon => {
                    if !self.line_path.contains(line_index) {
                        return Err(SolutionError::LineError(*line_index));
                    }
                }
                LineComplexity::LineBreak => {
                    if self.line_path.contains(line_index) {
                        return Err(SolutionError::LineError(*line_index));
                    }
                }
            }
        }
        Ok(())
    }
    fn check_panes(&self) -> Result<(), SolutionError> {
        let map = &self.puzzle.pane_complexity;
        for component in &self.components {
            let component_complexity: Vec<&PaneComplexity> =
                component.iter().filter_map(|pane_index| map.get(pane_index)).collect();

            let mut squares: HashMap<ComplexityColor, i32> = HashMap::new();
            for complexity in component_complexity {
                match complexity {
                    PaneComplexity::Square(color) => {
                        let x = squares.entry(*color).or_insert(0);
                        *x += 1;
                    }
                }
            }
            if squares.keys().count() > 1 {
                return Err(SolutionError::PaneError);
            }
        }
        Ok(())
    }
}

fn find_components(puzzle: &Puzzle, line_path: &[LineIndex]) -> Vec<Vec<PaneIndex>> {
    let n = puzzle.pane_nears.len();
    let mut color: Vec<i32> = vec![0; n]; // 0

    let mut components = Vec::new();

    for id in 0..n {
        if color[id] == 2 {
            continue;
        }

        let group_id = components.len();
        components.push(Vec::new());

        let mut stack = vec![id];
        color[id] = 1;
        while let Some(id1) = stack.pop() {
            color[id1] = 2;
            components[group_id].push(PaneIndex(id1 as u16));
            let near = &puzzle.pane_nears[id1];
            for (line_index, PaneIndex(id2)) in near {
                let id2 = (*id2) as usize;
                if !line_path.contains(line_index) && color[id2] == 0 {
                    color[id2] = 1;
                    stack.push(id2);
                }
            }
        }
    }

    components
}

#[cfg(test)]
mod test {
    use super::*;
 
    #[test]
    fn test_find_components() {
        let puzzle = Puzzle::default();
        let empty_path = Vec::new();
        let vecs = find_components(&puzzle, &empty_path);
        assert_eq!(1, vecs.len());
        assert_eq!(puzzle.panes.len(), vecs[0].len());
    }
}