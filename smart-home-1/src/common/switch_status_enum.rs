use std::fmt;

#[derive(Copy, Clone)]
pub enum SwitchStatusEnum {
    On,
    Off,
}

impl fmt::Display for SwitchStatusEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SwitchStatusEnum::On => write!(f, "On"),
            SwitchStatusEnum::Off => write!(f, "Off"),
        }
    }
}
