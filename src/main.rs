use clap::Parser;
use reqwest::blocking::{Client, get};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    #[arg(short, long, default_value_t = -2.0)]
    offset: f64,

    #[arg(short, long, default_value_t = 4)]
    camera_index: u8,

    #[arg(short, long)]
    url: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct SessionInfo {
    currentEventTime: f64,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct ReplayEvent {
    slotID: u32,
    eventTime: f64,
    cameraIndex: u8,
}

mod rf2_api {
    pub const REPLAY_EVENTS: &str = "/webdata/replayEvents";
    pub const SESSION_INFO: &str = "/rest/watch/sessionInfo";
    pub const FOCUS: &str = "/rest/watch/focus";
}

fn main() {
    let args = Args::parse();

    let session_info = get(format!("{}/{}", &args.url, rf2_api::SESSION_INFO))
        .unwrap()
        .json::<SessionInfo>()
        .unwrap();

    let focused: u32 = get(format!("{}/{}", &args.url, rf2_api::FOCUS))
        .unwrap()
        .text()
        .unwrap().parse().unwrap();

    let mut existing_events = get(format!("{}/{}", &args.url, rf2_api::REPLAY_EVENTS))
        .unwrap()
        .json::<Vec<ReplayEvent>>()
        .unwrap();

    existing_events.push(ReplayEvent {
        slotID: focused,
        eventTime: session_info.currentEventTime + args.offset,
        cameraIndex: args.camera_index,
    });

    Client::new().post(format!("{}/{}", &args.url, rf2_api::REPLAY_EVENTS))
        .json(&existing_events)
        .send()
        .unwrap();
}
