use crate::base::button::Button;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Default)]
pub struct ButtonPad {
    pub x: Button,
    pub y: Button,
    pub b: Button,
    pub a: Button,
}