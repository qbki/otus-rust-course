use std::fmt;

#[derive(Debug)]
pub enum SmartHomeErrorEnum {
    NotFoundDeviceError,
    NotFoundRoomError,
}

impl fmt::Display for SmartHomeErrorEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SmartHomeErrorEnum::NotFoundDeviceError => write!(f, "Device was not found"),
            SmartHomeErrorEnum::NotFoundRoomError => write!(f, "Room was not found"),
        }
    }
}

impl From<SmartHomeErrorEnum> for String {
    fn from(value: SmartHomeErrorEnum) -> Self {
        format!("{}", value)
    }
}
