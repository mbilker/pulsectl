use std::error::Error;
use std::fmt;

use pulse::error::{Code, PAErr};

#[derive(Debug)]
pub(crate) enum PulseCtlErrorType {
    ConnectError,
    OperationError,
    PulseAudioError,
}

/// Error thrown when PulseAudio throws an error code, there are 3 variants
/// `PulseCtlErrorType::ConnectError` when there's an error establishing a connection
/// `PulseCtlErrorType::OperationError` when the requested operation quis unexpecdatly or is cancelled
/// `PulseCtlErrorType::PulseAudioError` when PulseAudio returns an error code in any circumstance
#[derive(Debug)]
pub struct PulseCtlError {
    error: PulseCtlErrorType,
    message: String,
}

impl PulseCtlError {
    pub(crate) fn new(err: PulseCtlErrorType, msg: &str) -> Self {
        PulseCtlError {
            error: err,
            message: msg.to_string(),
        }
    }
}

impl From<PAErr> for PulseCtlError {
    fn from(error: PAErr) -> Self {
        let code: Code = error.into();
        PulseCtlError {
            error: PulseCtlErrorType::PulseAudioError,
            message: format!("PulseAudio returned error code {:?}", code),
        }
    }
}

impl fmt::Display for PulseCtlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_str = match self.error {
            PulseCtlErrorType::ConnectError => "ConnectError",
            PulseCtlErrorType::OperationError => "OperationError",
            PulseCtlErrorType::PulseAudioError => "PulseAudioError",
        };
        write!(f, "[{}]: {}", error_str, self.message)
    }
}

impl Error for PulseCtlError {}
