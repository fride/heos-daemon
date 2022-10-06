use heos_daemon_rust::{Connection, Frame, HeosResult, PlayerCommand, SystemCommand};
use pretty_env_logger::env_logger;
use serde_json::{to_string_pretty, to_value};
use heos_daemon_rust::OnOrOff::On;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> HeosResult<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    let mut connection = Connection::connect("192.168.178.35:1255").await?;

    let res = connection
        .execute_command(PlayerCommand::GetPlayers)
        .await?;
    let json = to_value(&res).unwrap();
    println!("{}", serde_json::to_string_pretty(&json).unwrap());

    let res = connection
        .execute_command(PlayerCommand::GetPlayerVolume { pid: 1128532863 })
        .await?;
    let json = to_value(&res).unwrap();
    println!("{}", serde_json::to_string_pretty(&json).unwrap());

    let res = connection
        .execute_command(PlayerCommand::GetPlayState { pid: 1128532863 })
        .await?;
    let json = to_value(&res).unwrap();
    println!("{}", serde_json::to_string_pretty(&json).unwrap());

    // println!("errrrrror!");
    // let res = connection
    //     .execute_command(PlayerCommand::GetPlayState { pid: 11283 })
    //     .await?;
    // let json = to_value(&res).unwrap();
    // println!("{}", serde_json::to_string_pretty(&json).unwrap());

    let _ = connection.execute_command(SystemCommand::RegisterForChangeEvents { enable: On}).await?;
    loop {
        let response = connection.read_frame().await?;
        match response {
            None => {}
            Some(frame) => {
                println!("{:?}", frame);
            }
        }
    }
    Ok(())
}
