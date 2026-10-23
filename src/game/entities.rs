use crate::engine::canvas::Canvas;
use crate::engine::canvas::Drawable;

enum Direction { Up, Down, Left, Right, None }
enum EntityKind { Ghost, Maze, Wall, Gate, Fruit, Pellet, Pacman, Empty }
trait IEntity {
    fn update(&mut self);
    fn entity(&self) -> Entity;
}

struct Entity {
    kind: EntityKind,
    position: (i32, i32),
    direction: Direction,
}

impl Entity {
    fn new(kind: EntityKind, position: (i32, i32)) -> Entity {
        Entity {
            kind,
            position,
            direction: Direction::None,
        }
    }
    
    fn update(&mut self) {}
    fn draw(&self, canvas: &mut Canvas) {}
    fn collision(&self, direction: &Direction) -> (bool, &Entity) { 
        todo!()
    }
    fn change_direction(&mut self, direction: Direction) { 
        todo!()
    }
    
}

struct Maze {
    dynamics_entities: Vec<Box<dyn IEntity>>,
    static_entities: Vec<Box<dyn IEntity>>,
}

impl Maze {

}

impl Drawable for Maze {
    fn draw(&self, canvas: &mut Canvas) {
    }
}