use crate::raw::{DirectionX, DirectionY};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Default)]
pub struct DirectionPad {
    pub(crate) dir_x: DirectionX,
    pub(crate) dir_y: DirectionY,
}

impl DirectionPad {
    pub fn x(&self) -> &DirectionX {
        &self.dir_x
    }

    pub fn y(&self) -> &DirectionY {
        &self.dir_y
    }

    pub fn up(&self) -> bool {
        self.dir_x == DirectionX::Up
    }

    pub fn down(&self) -> bool {
        self.dir_x == DirectionX::Down
    }

    pub fn left(&self) -> bool {
        self.dir_y == DirectionY::Left
    }

    pub fn right(&self) -> bool {
        self.dir_y == DirectionY::Right
    }
}