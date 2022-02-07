#![allow(non_snake_case)] //for crate name only

mod raw;
mod base;
mod parts;
mod controller;
use base::button;
use base::joystick;

use io::Read;
use std::fs::OpenOptions;
use std::io;

use raw::{RawEvent, EVENT_SIZE, is_init_event, parse_event};
use controller::Controller;


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
