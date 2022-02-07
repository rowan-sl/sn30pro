use crate::raw::{DirectionX, DirectionY};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Default)]
pub struct DirectionPad {
    pub(crate) dir_x: DirectionX,
    pub(crate) dir_y: DirectionY,
}

impl DirectionPad {
    pub fn up(&self) -> bool {
        self.dir_x == DirectionX::Up
    }

    pub fn down(&self) -> bool {
        self.dir_x == DirectionX::Down
    }
}