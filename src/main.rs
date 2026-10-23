use crossterm::style::Color;
use crate::engine::canvas::{Canvas, Cell};
use crate::engine::engine::{Engine, IApplication};

pub mod engine;
pub mod game;

struct App {

}

impl IApplication for App {
    fn on_init(&mut self) {
    }

    fn on_update(&mut self, delta_time: f32) {
    }

    fn on_draw(&mut self, canvas: &mut Canvas) {
        let cell = Cell::new('#', true, crossterm::style::Color::White, crossterm::style::Color::Black);
        canvas.draw_cell(0, 0, &cell);
    }
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    Engine::new(60, 60, App{}).run()?;

    Ok(())
}