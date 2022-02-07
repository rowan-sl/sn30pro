//possible values of event
pub const JS_EVENT_BUTTON: u8 = 0x01; /* button pressed/released */
pub const JS_EVENT_AXIS: u8 = 0x02; /* joystick moved */
pub const JS_EVENT_INIT: u8 = 0x80; /* initial state of device */

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ButtonEvent {
    Pressed,
    Released,
}

impl ButtonEvent {
    pub(crate) fn from_1or0(v: i16) -> ButtonEvent {
        match v {
            1 => ButtonEvent::Pressed,
            0 => ButtonEvent::Released,
            _ => panic!("you did a nono")
        }
    }

    pub(crate) fn from_i16max(v: i16) -> ButtonEvent {
        match v {
            32767 => ButtonEvent::Pressed,
            -32767 => ButtonEvent::Released,
            _ => panic!("you did a nono")
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum DirectionX {
    Up,
    Center,
    Down,
}

impl DirectionX {
    pub(crate) fn from_i16max(v: i16) -> DirectionX {
        match v {
            32767 => DirectionX::Down,
            0 => DirectionX::Center,
            -32767 => DirectionX::Up,
            _ => panic!("you did a nono")
        }
    }
}

impl Default for DirectionX {
    fn default() -> Self {
        Self::Center
    }
}

// 8 => InputUpdate::HeartButton(ButtonEvent::from_1or0(raw_event.value)),
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum DirectionY {
    Left,
    Center,
    Right,
}

impl DirectionY {
    pub(crate) fn from_i16max(v: i16) -> DirectionY {
        match v {
            32767 => DirectionY::Right,
            0 => DirectionY::Center,
            -32767 => DirectionY::Left,
            _ => panic!("you did a nono")
        }
    }
}

impl Default for DirectionY {
    fn default() -> Self {
        Self::Center
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum InputUpdate {
    R1(ButtonEvent),
    R2(ButtonEvent),
    L1(ButtonEvent),
    L2(ButtonEvent),
    BtnY(ButtonEvent),
    BtnX(ButtonEvent),
    BtnA(ButtonEvent),
    BtnB(ButtonEvent),
    BtnSelect(ButtonEvent),
    BtnStart(ButtonEvent),
    HeartButton(ButtonEvent),
    ButtonPadX(DirectionX),
    ButtonPadY(DirectionY),
    LJoystickX(i16),
    LJoystickY(i16),
    LJoystickButton(ButtonEvent),
    RJoystickX(i16),
    RJoystickY(i16),
    RJoystickButton(ButtonEvent),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct RawEvent {
    pub time: u32, // event timestamp in ms (unknown base time)
    pub value: i16, //value of the event, for axis changes it is -32767 to 32767, for buttons it is 0 or 1
    pub event: u8,
    pub number: u8,
}

pub const EVENT_SIZE: usize = std::mem::size_of::<RawEvent>();

pub fn parse_event(raw_event: RawEvent) -> InputUpdate {
    match raw_event.event {
        JS_EVENT_BUTTON => {
            match raw_event.number {
                0 => InputUpdate::BtnB(ButtonEvent::from_1or0(raw_event.value)),
                1 => InputUpdate::BtnA(ButtonEvent::from_1or0(raw_event.value)),
                2 => InputUpdate::BtnY(ButtonEvent::from_1or0(raw_event.value)),
                3 => InputUpdate::BtnX(ButtonEvent::from_1or0(raw_event.value)),
                4 => InputUpdate::L1(ButtonEvent::from_1or0(raw_event.value)),
                5 => InputUpdate::R1(ButtonEvent::from_1or0(raw_event.value)),
                6 => InputUpdate::BtnSelect(ButtonEvent::from_1or0(raw_event.value)),
                7 => InputUpdate::BtnStart(ButtonEvent::from_1or0(raw_event.value)),
                8 => InputUpdate::HeartButton(ButtonEvent::from_1or0(raw_event.value)),
                9 => InputUpdate::LJoystickButton(ButtonEvent::from_1or0(raw_event.value)),
                10 => InputUpdate::RJoystickButton(ButtonEvent::from_1or0(raw_event.value)),
                b => panic!("Invalid button {}", b),
            }
        }
        JS_EVENT_AXIS => {
            match raw_event.number {
                0 => InputUpdate::LJoystickY(raw_event.value),
                1 => InputUpdate::LJoystickX(-raw_event.value),
                2 => InputUpdate::RJoystickY(raw_event.value),
                3 => InputUpdate::RJoystickX(-raw_event.value),
                4 => InputUpdate::R2(ButtonEvent::from_i16max(raw_event.value)),
                5 => InputUpdate::L2(ButtonEvent::from_i16max(raw_event.value)),
                6 => InputUpdate::ButtonPadY(DirectionY::from_i16max(raw_event.value)),
                7 => InputUpdate::ButtonPadX(DirectionX::from_i16max(raw_event.value)),
                _ => unreachable!(),
            }
        }
        _ => unreachable!()
    }
}

pub fn is_init_event(event: &RawEvent) -> bool {
    event.event & JS_EVENT_INIT == JS_EVENT_INIT
}