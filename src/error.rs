use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlcError {
    NotInitialized,
    InvalidAddress,
    NullPointer,
    CommunicationFailure(i32),
}

impl fmt::Display for PlcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotInitialized => {
                write!(f, "PLC is not initialized")
            }

            Self::InvalidAddress => {
                write!(f, "invalid PLC address")
            }

            Self::NullPointer => {
                write!(f, "null pointer supplied to PLC interface")
            }

            Self::CommunicationFailure(code) => {
                write!(f, "PLC communication failure: {}", code)
            }
        }
    }
}

impl std::error::Error for PlcError {}

impl From<i32> for PlcError {
    fn from(code: i32) -> Self {
        match code {
            -1 => Self::InvalidAddress,
            -2 => Self::NullPointer,
            other => Self::CommunicationFailure(other),
        }
    }
}
