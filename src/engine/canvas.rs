use std::io::Write;
use std::io::Result;
use crossterm::{style::Color, queue};
use crossterm::cursor::MoveTo;
use crossterm::style::{Print, SetBackgroundColor, SetForegroundColor};
use crossterm::style::Color::Reset;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Cell {
    pub symbol: char,
    pub visible: bool,
    pub background_color: Color,
    pub foreground_color: Color,
}

impl Cell {
    pub(crate) fn new(symbol: char, visible: bool, background_color: Color, foreground_color: Color) -> Self {
        Self {
            symbol,
            visible,
            background_color,
            foreground_color,
        }
    }

    fn default() -> Self {
        Self::new(' ', true, Reset, Reset)
    }
}

pub struct Canvas {
    size: usize,
    width: usize,
    height: usize,
    x_offset: usize,
    y_offset: usize,
    foreground_cells: Vec<Cell>,
    background_cells: Vec<Cell>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            size,
            width,
            height,
            x_offset: 0,
            y_offset: 0,
            foreground_cells: vec![Cell::default(); size],
            background_cells: vec![Cell::default(); size],
        }
    }

    pub fn render(&mut self) -> Result<()> {
        let out = &mut std::io::stdout();
        let (ix_position, iy_position) = crossterm::cursor::position()?;

        for x in 0..self.width {
            for y in 0..self.height {
                let index = x + y * self.width;
                let foreground_cell = self.foreground_cells[index];
                let mut background_cell= self.background_cells[index];
                if foreground_cell != background_cell && foreground_cell.visible {
                    queue!(
                        out,
                        MoveTo((x + self.x_offset) as u16, (y + self.y_offset) as u16),
                        SetBackgroundColor(foreground_cell.background_color),
                        SetForegroundColor(foreground_cell.foreground_color),
                        Print(foreground_cell.symbol),
                    )?;

                    foreground_cell.clone_into(&mut self.background_cells[index]);
                }
            }
        }
        queue!(
            out,
            MoveTo(ix_position, iy_position),
            SetForegroundColor(Reset),
            SetBackgroundColor(Reset),
        )?;

        out.flush()
    }

    pub fn draw<T: DrawableBehavior>(&mut self, drawable: T) {
        drawable.draw(self);
    }

    pub fn draw_cell(&mut self, x_offset: usize, y_offset: usize, cell: &Cell){
        if !cell.visible || x_offset >= self.width || y_offset >= self.height { return; }
        self.foreground_cells[x_offset + y_offset * self.width] = cell.clone();
    }

    pub(crate) fn clear(&mut self) {
        self.foreground_cells.fill(Cell::default());
    }
}

pub trait DrawableBehavior {
    fn draw(&self, canvas: &mut Canvas);
}

impl DrawableBehavior for Canvas {
    fn draw(&self, canvas: &mut Canvas) {
        for x in 0..self.width {
            for y in 0..self.height {
                canvas.draw_cell(x, y, &self.foreground_cells[x + y * self.width]);
            }
        }
    }
}