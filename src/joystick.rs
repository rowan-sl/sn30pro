#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Joystick {
    pub x: i16,
    pub y: i16,
}

impl Default for Joystick {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }
}