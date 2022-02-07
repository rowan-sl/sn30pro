use crate::button::Button;
use crate::joystick::Joystick;
use crate::parts::triggers::Triggers;
use crate::parts::button_pad::ButtonPad;
use crate::parts::direction_pad::DirectionPad;
use crate::raw;


#[derive(Clone, Debug, Hash, PartialEq, Eq, Default)]
pub struct Controller {
    triggers: Triggers,
    r_joystick: Joystick,
    l_joystick: Joystick,
    start: Button,
    select: Button,
    heart: Button,
    button_pad: ButtonPad,
    direction_pad: DirectionPad,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn sink(&mut self, event: raw::InputUpdate) {
        use raw::InputUpdate;
        //TODO make direction pad support checking for last inputs
        match event {
            InputUpdate::R1(b_event) => {self.triggers.r1.update(b_event.into())}
            InputUpdate::R2(b_event) => {self.triggers.r2.update(b_event.into())}
            InputUpdate::L1(b_event) => {self.triggers.l1.update(b_event.into())}
            InputUpdate::L2(b_event) => {self.triggers.l2.update(b_event.into())}
            InputUpdate::BtnY(b_event) => {self.button_pad.y.update(b_event.into())}
            InputUpdate::BtnX(b_event) => {self.button_pad.x.update(b_event.into())}
            InputUpdate::BtnA(b_event) => {self.button_pad.a.update(b_event.into())}
            InputUpdate::BtnB(b_event) => {self.button_pad.b.update(b_event.into())}
            InputUpdate::BtnSelect(b_event) => {self.select.update(b_event.into())}
            InputUpdate::BtnStart(b_event) => {self.start.update(b_event.into())}
            InputUpdate::HeartButton(b_event) => {self.heart.update(b_event.into())}
            InputUpdate::ButtonPadX(direction) => {self.direction_pad.dir_x = direction}
            InputUpdate::ButtonPadY(direction) => {self.direction_pad.dir_y = direction}
            InputUpdate::LJoystickX(val) => {self.l_joystick.x = val}
            InputUpdate::LJoystickY(val) => {self.l_joystick.y = val}
            InputUpdate::RJoystickX(val) => {self.r_joystick.x = val}
            InputUpdate::RJoystickY(val) => {self.r_joystick.y = val}
        }
    }

    pub fn triggers(&mut self) -> &mut Triggers {
        &mut self.triggers
    }
    pub fn l_joy(&mut self) -> &mut Joystick {
        &mut self.l_joystick
    }
    pub fn r_joy(&mut self) -> &mut Joystick {
        &mut self.r_joystick
    }
    pub fn start(&mut self) -> &mut Button {
        &mut self.start
    }
    pub fn select(&mut self) -> &mut Button {
        &mut self.select
    }
    pub fn heart(&mut self) -> &mut Button {
        &mut self.heart
    }
    pub fn d_pad(&mut self) -> &mut DirectionPad {
        &mut self.direction_pad
    }
    pub fn btn_pad(&mut self) -> &mut ButtonPad {
        &mut self.button_pad
    }
}

