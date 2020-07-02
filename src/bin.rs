use youtrack_rs::client::{Executor, YouTrack};

use serde_json::Value;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "youtrack")]
pub struct YouTrackOpt {
    #[structopt(long, env = "YOUTRACK_URL")]
    pub youtrack_url: String,

    #[structopt(long, env = "YOUTRACK_TOKEN")]
    pub youtrack_token: String,
}

#[tokio::main]
async fn main() {
    let opt = YouTrackOpt::from_args();

    let yt = YouTrack::new(opt.youtrack_url.clone(), opt.youtrack_token.clone()).unwrap();

    let me = yt
        .get()
        .users()
        .me()
        .fields("login")
        .execute::<Value>()
        .await
        .unwrap();

    let (headers, status, json) = me;
    println!("{:#?}", headers);
    println!("{}", status);
    println!("{:?}", json);
}
