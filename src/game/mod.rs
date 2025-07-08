mod generation;

//use generation::world::World;
use crate::application::Input;

pub struct Game {
    //world: World
}

impl Game {
    pub fn new() -> Self{
        Self {}
    }

    pub fn tick(&mut self, input: Input) {

    }
}