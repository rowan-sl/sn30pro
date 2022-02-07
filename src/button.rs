use crate::raw::{self, ButtonEvent};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ButtonState {
    Pressed,
    Released,
}

impl From<ButtonState> for bool {
    fn from(s: ButtonState) -> Self {
        s == ButtonState::Pressed
    }
}

impl From<ButtonEvent> for ButtonState {
    fn from(event: ButtonEvent) -> Self {
        match event {
            ButtonEvent::Pressed => Self::Pressed,
            ButtonEvent::Released => Self::Released
        }
    }
}

impl Default for ButtonState {
    fn default() -> Self {
        Self::Released
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Button {
    state: ButtonState,
    pressed_since: bool,
    released_since: bool,
}

impl From<raw::ButtonEvent> for Button {
    fn from(be: raw::ButtonEvent) -> Self {
        Self {
            state: match be {
                raw::ButtonEvent::Pressed => ButtonState::Pressed,
                raw::ButtonEvent::Released => ButtonState::Released,
            },
            pressed_since: false,
            released_since: false,
        }
    }
}

impl Default for Button {
    fn default() -> Self {
        Self {
            pressed_since: false,
            released_since: false,
            state: ButtonState::default(),
        }
    }
}

impl Button {
    /// Is the button currently pressed
    pub fn pressed(&self) -> bool {
        self.state.into()
    }

    /// has the button been pressed since it last was checked
    pub fn pressed_since(&mut self) -> bool {
        if self.pressed_since {
            self.pressed_since = false;
            true
        } else {
            false
        }
    }

    /// has the released been pressed since it last was checked
    pub fn released_since(&mut self) -> bool {
        if self.released_since {
            self.released_since = false;
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, state: ButtonState) {
        if state != self.state {
            match state {
                ButtonState::Pressed => {
                    self.pressed_since = true;
                }
                ButtonState::Released => {
                    self.released_since = true;
                }
            }
            self.state = state;
        }
    }
}
