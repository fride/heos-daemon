use crate::{Connection, Level, OnOrOff, PlayState, PlayerId};
use std::fmt::{format, write, Display, Formatter};
use tracing::enabled;

const SYSTEM: &str = "system";
const HEOS: &str = "heos";

pub struct CommandPayload(String);
impl Display for CommandPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "heos://{}", self.0)
    }
}

pub enum SystemCommand {
    RegisterForChangeEvents { enable: OnOrOff },
    AccountCheck,
    SignIn { un: String, pw: String },
    SignOut,
    HeartBeat,
    SpeakerReboot,
    PrettifyJson,
}

impl From<SystemCommand> for CommandPayload {
    fn from(cmd: SystemCommand) -> Self {
        match cmd {
            SystemCommand::RegisterForChangeEvents { enable } => CommandPayload(format!(
                "system/register_for_change_events?enable={}",
                enable
            )),
            SystemCommand::AccountCheck => CommandPayload("system/account_check".to_owned()),
            SystemCommand::SignIn { un, pw } => {
                CommandPayload(format!("system/sign_in?un={}&pw={}", un, pw))
            }
            SystemCommand::SignOut => CommandPayload("system/sign_out".to_owned()),
            SystemCommand::HeartBeat => CommandPayload("system/heart_beat".to_owned()),
            SystemCommand::SpeakerReboot => CommandPayload("system/speaker_reboot".to_owned()),
            SystemCommand::PrettifyJson => CommandPayload("system/prettify_json".to_owned()),
        }
    }
}
pub enum PlayerCommand {
    GetPlayers,
    GetPlayerInfo { pid: PlayerId },
    GetPlayState { pid: PlayerId },
    SetPlayState { pid: PlayerId, state: PlayState },
    GetNowPlayingMedia { pid: PlayerId },
    GetPlayerVolume { pid: PlayerId },
    SetPlayerVolume { pid: PlayerId, level: Level },
}
impl From<PlayerCommand> for CommandPayload {
    fn from(command: PlayerCommand) -> Self {
        match command {
            PlayerCommand::GetPlayers => CommandPayload("player/get_players".to_owned()),
            PlayerCommand::GetPlayerInfo { pid } => {
                CommandPayload(format!("player/get_player?pid={}", pid))
            }
            PlayerCommand::GetPlayState { pid } => {
                CommandPayload(format!("player/get_play_state?pid={}", pid))
            }
            PlayerCommand::SetPlayState { pid, state } => {
                CommandPayload(format!("player/get_play_state?pid={}&state={}", pid, state))
            }
            PlayerCommand::GetNowPlayingMedia { pid } => {
                CommandPayload(format!("player/get_now_playing_media?pid={}", pid))
            }
            PlayerCommand::GetPlayerVolume { pid } => {
                CommandPayload(format!("player/get_volume?pid={}", pid))
            }
            PlayerCommand::SetPlayerVolume { pid, level } => {
                CommandPayload(format!("player/set_volume?pid={}&leve={}", pid, level))
            }
        }
    }
}

pub enum HeosCommand {
    System(SystemCommand),
    Player(PlayerCommand),
}

impl From<HeosCommand> for CommandPayload {
    fn from(cmd: HeosCommand) -> Self {
        match cmd {
            HeosCommand::System(cmd) => cmd.into(),
            HeosCommand::Player(cmd) => cmd.into(),
        }
    }
}
