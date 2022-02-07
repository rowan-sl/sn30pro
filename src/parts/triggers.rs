use crate::button::Button;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Default)]
pub struct Triggers {
    pub r1: Button,
    pub r2: Button,
    pub l1: Button,
    pub l2: Button,
}
