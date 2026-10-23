use crossterm::style::Color;
use crate::engine::canvas::{Canvas, Cell};
use crate::engine::engine::{Engine, ApplicationBehavior};

pub mod engine;
pub mod game;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    Engine::new(60, 60, game::application::Application::new()).run()?;

    Ok(())
}