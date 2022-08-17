use thiserror::Error;

#[derive(Error, Debug)]
pub enum SmartHomeErrorEnum {
    #[error("Device was not found ({0})")]
    NotFoundDeviceError(String),
    #[error("Room was not found ({0})")]
    NotFoundRoomError(String),
}
