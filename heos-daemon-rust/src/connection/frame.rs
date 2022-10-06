use anyhow::anyhow;
use anyhow::Context;
use std::io::Cursor;

use crate::{CommandResponse, EventResponse};
use bytes::Buf;
use serde_json::Value as Json;

use crate::error::{ErrorMessage, HeosError, HeosErrorCode};

use super::response_line::*;

#[derive(Debug)]
pub enum Frame {
    UnderProcess(String),
    Response(CommandResponse),
    Event(EventResponse),
    Error(ErrorMessage),
}

pub struct Incomplete;

impl Frame {
    // checks only for a line. Could also check json but that would mean duplicate parsing?
    pub fn check(src: &mut Cursor<&[u8]>) -> Result<(), Incomplete> {
        if !src.has_remaining() {
            return Err(Incomplete);
        }
        let _ = get_line(src)?;
        Ok(())
    }
    pub fn parse(src: &mut Cursor<&[u8]>) -> Result<Frame, HeosError> {
        if let Ok(line) = get_line(src) {
            let json =
                serde_json::from_slice::<Json>(line).context("Failed to parse response as json")?;
            parsers::parse_response(json)
        } else {
            Err(anyhow!("Connection reset by peer while reading line").into())
        }
    }
}
// /// Find a line
fn get_line<'a>(src: &mut Cursor<&'a [u8]>) -> Result<&'a [u8], Incomplete> {
    // Scan the bytes directly
    let start = src.position() as usize;
    // Scan to the second to last byte
    let end = src.get_ref().len() - 1;

    for i in start..end {
        if src.get_ref()[i] == b'\r' && src.get_ref()[i + 1] == b'\n' {
            // We found a line, update the position to be *after* the \n
            src.set_position((i + 2) as u64);

            // Return the line
            return Ok(&src.get_ref()[start..i]);
        }
    }

    Err(Incomplete)
}

mod parsers {
    use super::*;

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub enum HeosResultState {
        #[serde(rename = "success")]
        Success,

        #[serde(rename = "fail")]
        Failure,
    }

    #[derive(Clone, Debug)]
    pub enum ResponseName {
        CommandName(String),
        EventName(String),
    }

    impl ResponseName {
        pub fn name(&self) -> String {
            match self {
                ResponseName::CommandName(n) => n.clone(),
                ResponseName::EventName(n) => n.clone(),
            }
        }
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct HeosHeader {
        #[serde(with = "response_name_parser")]
        command: ResponseName,

        result: Option<HeosResultState>,

        #[serde(default)]
        message: String,
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct HeosReponse {
        heos: HeosHeader,

        #[serde(default)]
        payload: Json,
        #[serde(default)]
        options: Json,
    }

    pub fn parse_response(json: Json) -> Result<Frame, HeosError> {
        let response: HeosReponse =
            serde_json::from_value(json).context("Failed to parse heos response")?;
        match (
            &response.heos.command,
            &response.heos.result,
            &response.heos.message,
        ) {
            (cmd, Some(HeosResultState::Failure), message) => {
                let mut error: ErrorMessage = {
                    let json = qs_to_json(&message);
                    serde_json::from_value(json)
                        .context(format!("could not parse {} as json", &message))?
                };
                error.context = Some(cmd.name());
                Ok(Frame::Error(error))
            }
            (ResponseName::EventName(name), _, message) => {
                let json = qs_to_json(&message);
                Ok(Frame::Event(EventResponse {
                    event_name: name.clone(),
                    message: json,
                }))
            },
            (ResponseName::CommandName(name), _, message) if message == "command under process" => {
                Ok(Frame::UnderProcess(name.clone()))
            }
            (ResponseName::CommandName(name), _, message) => {
                let parsed_message = qs_to_json(&response.heos.message);
                Ok(Frame::Response(CommandResponse {
                    command_name: name.clone(),
                    message: parsed_message,
                    payload: response.payload,
                    options: response.options,
                }))
            }
        }
    }

    mod response_name_parser {
        use serde::{self, Deserialize, Deserializer, Serializer};

        use super::ResponseName;

        pub fn serialize<S>(name: &ResponseName, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match name {
                ResponseName::CommandName(s) => serializer.serialize_str(&s),
                ResponseName::EventName(s) => serializer.serialize_str(&s),
            }
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<ResponseName, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            if s.starts_with("event") {
                Ok(ResponseName::EventName(s))
            } else {
                Ok(ResponseName::CommandName(s))
            }
        }
    }
}
//
// #[cfg(test)]
// mod tests {
//     use serde_json::*;
//
//     use crate::model;
//
//     use super::*;
//
//     struct RegisterForChangeEvents {
//         enable: model::OnOrOff,
//     }
//
//     #[test]
//     fn it_works() {
//         let _json_str = "
//         {
//             \
//         }
//         ";
//         let json = json!({
//             "heos": {
//                 "command": "command/name",
//                 "result": "success",
//                 "message": "enable=on"
//             }
//         });
//
//         let _read = parsers::parse_response(json).unwrap();
//         assert_eq!(2 + 2, 4);
//     }
// }
