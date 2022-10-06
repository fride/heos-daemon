use serde_json::Value as Json;
use std::fmt;

pub type PlayerId = i64;
pub type GroupId = i64;
pub type QueueId = i64;
pub type SourceId = i64;
pub type AlbumId = String;
pub type MediaId = String;
pub type ContainerId = String;
pub type Level = u8;
pub type Milliseconds = u64;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub enum PlayState {
    #[serde(rename = "play")]
    Play,
    #[serde(rename = "pause")]
    Pause,
    #[serde(rename = "stop")]
    Stop,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum OnOrOff {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Repeat {
    Off,
    OnOne,
    OnAll,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CommandResult {
    Success,
    Fail,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandResponse {
    pub command_name: String,
    pub message: Json, //
    pub payload: Json, // can be Null
    pub options: Json, // can be Null
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub command_name: String,
    pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventResponse {
    pub event_name: String,
    pub message: Json
}

//////
impl fmt::Display for OnOrOff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                &OnOrOff::Off => "off",
                &OnOrOff::On => "on",
            }
        )
    }
}
impl std::str::FromStr for OnOrOff {
    type Err = String;

    fn from_str(string: &str) -> Result<OnOrOff, String> {
        return match string {
            "on" => Ok(OnOrOff::On),
            "off" => Ok(OnOrOff::Off),
            c => Err(format!("can't convert {} to OnOff", c)),
        };
    }
}
impl fmt::Display for PlayState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                &PlayState::Play => "play",
                &PlayState::Pause => "pause",
                &PlayState::Stop => "stop",
            }
        )
    }
}
