use std::fmt;
use std::error::Error;
use crate::common::DscError;

#[derive(Debug)]
pub enum SmartHomeErrorEnum {
    NotFoundDeviceError(DscError),
    NotFoundRoomError,
}

impl fmt::Display for SmartHomeErrorEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SmartHomeErrorEnum::NotFoundDeviceError(_) => write!(f, "Device was not found"),
            SmartHomeErrorEnum::NotFoundRoomError => write!(f, "Room was not found"),
        }
    }
}

impl Error for SmartHomeErrorEnum {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NotFoundDeviceError(source) => Some(source),
            _ => None,
        }
    }
}
