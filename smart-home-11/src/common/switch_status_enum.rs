use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SwitchStatusEnum {
    On,
    Off,
}

impl Default for SwitchStatusEnum {
    fn default() -> Self {
        SwitchStatusEnum::Off
    }
}

impl fmt::Display for SwitchStatusEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SwitchStatusEnum::On => write!(f, "On"),
            SwitchStatusEnum::Off => write!(f, "Off"),
        }
    }
}

impl From<bool> for SwitchStatusEnum {
    fn from(value: bool) -> Self {
        match value {
            true => SwitchStatusEnum::On,
            false => SwitchStatusEnum::Off,
        }
    }
}

impl From<SwitchStatusEnum> for bool {
    fn from(value: SwitchStatusEnum) -> Self {
        match value {
            SwitchStatusEnum::On => true,
            SwitchStatusEnum::Off => false,
        }
    }
}
