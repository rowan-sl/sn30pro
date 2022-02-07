#![allow(non_snake_case)] //for crate name only

mod raw;
mod button;
mod joystick;
mod parts;

use io::Read;
use std::fs::OpenOptions;
use std::io;

use raw::{RawEvent, EVENT_SIZE, is_init_event, parse_event};
use button::Button;
use joystick::Joystick;
use parts::triggers::Triggers;
use parts::button_pad::ButtonPad;
use parts::direction_pad::DirectionPad;


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
}

fn main() -> io::Result<()> {
    let mut js_src = OpenOptions::new().read(true).open("/dev/input/js0")?;
    let mut buf = [0; EVENT_SIZE];
    let mut c = Controller::new();
    loop {
        js_src.read_exact(&mut buf)?;
        let event_dat = buf;
        // insert this is fine meme here
        let raw_event: RawEvent = unsafe { std::mem::transmute(event_dat) };

        //ignore init events
        if !is_init_event(&raw_event) {
            let parsed_event = parse_event(raw_event);
            c.sink(parsed_event);
            println!("{:#?}", c);
        }
    }
}
