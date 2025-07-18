use super::units::{ PlayerCoords, PlayerDirection };

pub struct Player {
    pub position: PlayerCoords,
    pub direction: PlayerDirection
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: PlayerCoords { x: 0., y: 0., z: 0. },
            direction: PlayerDirection { pitch: 0., yaw: 0. }
        }
    }
}