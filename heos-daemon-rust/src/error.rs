use qs;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Hash, Serialize)]
pub enum HeosErrorCode {
    UnrecognizedCommand = 1,
    InvalidId = 2,
    WrongNumberOfArguments = 3,
    RequestedDataNotAvailable = 4,
    ResourceCurrentlyNotAvailable = 5,
    InvalidCredentials = 6,
    CommandCouldNitBeExecuted = 7,
    UserNotLoggedIn = 8,
    ParameterOutOfRange = 9,
    UserNotFound = 10,
    InternalError = 11,
    SystemError = 12,
    ProcessingPreviousCommand = 13,
    MediaCantBePlayed = 14,
    OptionNotSupported = 15,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub eid: HeosErrorCode,
    pub text: String,
    pub context: Option<String>,
}

#[derive(Error, Debug)]
pub enum HeosError {

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("no devices found")]
    NoDevicesFound,

    // An invalid command was send to the heos box
    #[error("Invalid command ")]
    InvalidCommand(ErrorMessage),
}
