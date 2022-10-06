use crate::error::HeosErrorCode;
use crate::types::*;
use crate::HeosCommand::Player;
use anyhow::anyhow;
use serde::Deserialize;
use serde_json::{Map, Value};
use std::collections::BTreeMap;

// this turns the rather strange query string into a json object
// nice to easy parsing upstream.
pub fn qs_to_json(message: &str) -> Value {
    use serde_json::*;
    if message.is_empty() {
        Value::Null
    } else {
        // remove spaces to make it a valid url encoded string.
        let clean_message = message.replace(" ", "%20");
        let value = qs::from_str::<EventQueryParams>(&clean_message);
        match value {
            Ok(params) => remove_null(serde_json::to_value(params).unwrap()),
            Err(err) => {
                Value::String(message.to_owned())
            }
        }
    }
}

// this is used to collect all possible parameters in heos strange query string format
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
struct EventQueryParams {
    pid: Option<i64>,
    gid: Option<i64>,
    level: Option<u8>,
    mute: Option<OnOrOff>,
    shuffle: Option<OnOrOff>,
    repeat: Option<Repeat>,
    un: Option<String>,
    text: Option<String>,
    eid: Option<u8>,
    cur_pos: Option<Milliseconds>,
    duration: Option<Milliseconds>,
    state: Option<PlayState>,
}

// remove all null values from the object!
// TODO maybe use also https://github.com/jmfiaschi/json_value_merge/blob/main/src/lib.rs
pub fn remove_null(value: Value) -> Value {
    fn non_null_object(entries: Map<String, Value>) -> Value {
        let clean = entries.into_iter().filter(|e| !e.1.is_null()).collect();
        Value::Object(clean)
    }
    match value {
        Value::Object(map) => non_null_object(map),
        other => other,
    }
}

impl<'de> Deserialize<'de> for HeosErrorCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match i8::deserialize(deserializer)? {
            1 => Ok(HeosErrorCode::UnrecognizedCommand),
            2 => Ok(HeosErrorCode::InvalidId),
            3 => Ok(HeosErrorCode::WrongNumberOfArguments),
            4 => Ok(HeosErrorCode::RequestedDataNotAvailable),
            5 => Ok(HeosErrorCode::ResourceCurrentlyNotAvailable),
            6 => Ok(HeosErrorCode::InvalidCredentials),
            7 => Ok(HeosErrorCode::CommandCouldNitBeExecuted),
            8 => Ok(HeosErrorCode::UserNotLoggedIn),
            9 => Ok(HeosErrorCode::ParameterOutOfRange),
            10 => Ok(HeosErrorCode::UserNotFound),
            11 => Ok(HeosErrorCode::InternalError),
            12 => Ok(HeosErrorCode::SystemError),
            13 => Ok(HeosErrorCode::ProcessingPreviousCommand),
            14 => Ok(HeosErrorCode::MediaCantBePlayed),
            15 => Ok(HeosErrorCode::OptionNotSupported),
            _ => Ok(HeosErrorCode::Unknown),
        }
    }
}
