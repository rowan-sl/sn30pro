use std::io;

use tokio::io::AsyncReadExt;
use tokio::fs::{File, OpenOptions};
use bytes::BytesMut;

use crate::base::button::Button;
use crate::base::joystick::Joystick;
use crate::parts::triggers::Triggers;
use crate::parts::button_pad::ButtonPad;
use crate::parts::direction_pad::DirectionPad;
use crate::raw;


#[derive(Debug)]
pub struct Controller {
    src: File,
    read_buf: BytesMut,
    //controller components
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
    pub async fn new(controller_id: usize) -> io::Result<Self> {
        let path = format!("/dev/input/js{}", controller_id);
        let file = OpenOptions::new().read(true).open(path).await?;
        Ok(Self {
            src: file,
            read_buf: BytesMut::with_capacity(raw::EVENT_SIZE*3),
            triggers: Triggers::default(),
            r_joystick: Joystick::default(),
            l_joystick: Joystick::default(),
            start: Button::default(),
            select: Button::default(),
            heart: Button::default(),
            button_pad: ButtonPad::default(),
            direction_pad: DirectionPad::default(),
        })
    }

    fn sink(&mut self, event: raw::InputUpdate) {
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
    
    /// Reads data from the internal source file,
    /// and if there is enough, produces and processes a event,
    /// 
    /// returns if a new event was processed
    /// 
    /// # Cancel saftey
    /// this is cancel safe, if it is canceled, no bytes will have been read or data lost.
    pub async fn update(&mut self) -> io::Result<bool> {
        if self.read_buf.len() >= raw::EVENT_SIZE {
            let bytes_data = self.read_buf.split_to(raw::EVENT_SIZE).freeze();
            let mut buf: [u8; raw::EVENT_SIZE] = [0; raw::EVENT_SIZE];
            for i in 0..raw::EVENT_SIZE {
                buf[i] = bytes_data[i];
            }
            // insert this is fine meme here
            let raw_event: raw::RawEvent = unsafe { std::mem::transmute(buf) };
            //ignore init events
            if !raw::is_init_event(&raw_event) {
                let parsed_event = crate::raw::parse_event(raw_event);
                self.sink(parsed_event);
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            self.src.read_buf(&mut self.read_buf).await?;
            Ok(false)
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

