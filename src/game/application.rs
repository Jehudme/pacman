use crate::engine::canvas::{Canvas, DrawableBehavior};
use crate::engine::engine::ApplicationBehavior;
use crate::engine::entity::EntityBehavior;
use crate::game::entities::Maze;

pub(crate) struct Application {
    maze: Maze,
}



impl Application {
    pub(crate) fn new() -> Self {
        let maze_data = include_str!("../../assets/maze.txt");
        let maze = Maze::create_from_memory(maze_data);
        Self { maze }
    }
}

impl EntityBehavior for Application {
    fn update(&mut self, delta_time: f32) {
        self.maze.update(delta_time);
    }
}

impl DrawableBehavior for Application {
    fn draw(&self, canvas: &mut Canvas) {
        self.maze.draw(canvas);
    }
}

impl ApplicationBehavior for Application {
    fn init(&mut self) {
        // Initialize the application state here
    }
}

