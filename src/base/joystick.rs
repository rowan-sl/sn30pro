use super::button::Button;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Joystick {
    pub x: i16,
    pub y: i16,
    pub btn: Button,
}

impl Default for Joystick {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            btn: Button::default(),
        }
    }
}