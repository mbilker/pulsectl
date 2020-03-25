use std::error::Error;
use std::fmt;

use crate::PulseCtlError;

#[derive(Debug)]
pub(crate) enum ControllerErrorType {
    PulseCtlError,
    GetInfoError,
}

/// Error thrown while fetching data from pulseaudio,
/// has two variants: PulseCtlError for when PulseAudio returns an error code
/// and GetInfoError when a request for data fails for whatever reason
#[derive(Debug)]
pub struct ControllerError {
    error: ControllerErrorType,
    message: String,
}

impl ControllerError {
    pub(crate) fn new(err: ControllerErrorType, msg: &'static str) -> Self {
        ControllerError {
            error: err,
            message: msg.to_string(),
        }
    }
}

/// if the error occurs within the Mainloop, we bubble up the error with
/// this conversion
impl From<PulseCtlError> for ControllerError {
    fn from(error: super::errors::PulseCtlError) -> Self {
        ControllerError {
            error: ControllerErrorType::PulseCtlError,
            message: format!("{:?}", error),
        }
    }
}

impl fmt::Display for ControllerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_str = match self.error {
            ControllerErrorType::PulseCtlError => "PulseCtlError",
            ControllerErrorType::GetInfoError => "GetInfoError",
        };
        write!(f, "[{}]: {}", error_str, self.message)
    }
}

impl Error for ControllerError {}
