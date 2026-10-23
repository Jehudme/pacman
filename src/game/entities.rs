use crate::engine::canvas::{Canvas, Cell, DrawableBehavior};
use crate::engine::entity::EntityBehavior;
use crossterm::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MazeLayout {
    Empty,
    Wall,
    Gate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MazeEntity {
    Pacman,
    Ghost,
    Pellet,
    Fruit,
}

pub struct Maze {
    pub size: (usize, usize),
    pub layout: Vec<MazeLayout>,
    pub entities: Vec<Box<dyn EntityBehavior>>,
}

impl Maze {
    pub fn create_from_memory(data: &str) -> Self {
        let mut layout = Vec::new();
        let mut rows = 0;
        let mut cols = None;

        for line in data.lines() {
            let line = line.strip_suffix('\r').unwrap_or(line);

            if line.is_empty() {
                continue;
            }

            let line_len = line.chars().count();
            match cols {
                None => cols = Some(line_len),
                Some(expected) if expected != line_len => {
                    panic!(
                        "Inconsistent row length: expected {}, found {}",
                        expected, line_len
                    );
                }
                _ => {}
            }

            for ch in line.chars() {
                let tile = match ch {
                    '#' => MazeLayout::Wall,
                    '*' => MazeLayout::Gate,
                    _ => MazeLayout::Empty,
                };
                layout.push(tile);
            }

            rows += 1;
        }

        let cols = cols.unwrap_or(0);
        Self {
            size: (cols, rows),
            layout,
            entities: Vec::new(),
        }
    }

    pub fn create_from_file(file_path: &str) -> Self {
        let contents = std::fs::read_to_string(file_path)
            .unwrap_or_else(|e| panic!("Failed to read maze file {}: {}", file_path, e));
        Self::create_from_memory(&contents)
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<MazeLayout> {
        if x < self.size.0 && y < self.size.1 {
            Some(self.layout[x + y * self.size.0])
        } else {
            None
        }
    }

    pub fn add_entity<E: EntityBehavior + 'static>(&mut self, entity: E) {
        self.entities.push(Box::new(entity));
    }
}

impl DrawableBehavior for Maze {
    fn draw(&self, canvas: &mut Canvas) {
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let tile = self.get_tile(x, y).unwrap_or(MazeLayout::Empty);
                let cell = match tile {
                    MazeLayout::Wall => {
                        Cell::new('#', true, Color::Black, Color::Blue)
                    }
                    MazeLayout::Gate => {
                        Cell::new('-', true, Color::Black, Color::Yellow)
                    }
                    MazeLayout::Empty => {
                        Cell::new(' ', false, Color::Reset, Color::Reset)
                    }
                };
                canvas.draw_cell(x, y, &cell);
            }
        }

        for entity in &self.entities {
            entity.draw(canvas);
        }
    }
}

impl EntityBehavior for Maze {
    fn update(&mut self, delta_time: f32) {
        for entity in &mut self.entities {
            entity.update(delta_time);
        }
    }
}