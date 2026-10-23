use crate::engine::canvas::{Canvas, Cell, DrawableBehavior};

pub trait EntityBehavior: DrawableBehavior {
    fn update(&mut self, delta_time: f32);
}

pub struct Drawable {
    pub position: (i32, i32),
    pub cell: Cell,
}

impl DrawableBehavior for Drawable {
    fn draw(&self, canvas: &mut Canvas) {
        if self.position.0 >= 0 && self.position.1 >= 0 {
            canvas.draw_cell(
                self.position.0 as usize,
                self.position.1 as usize,
                &self.cell,
            );
        }
    }
}