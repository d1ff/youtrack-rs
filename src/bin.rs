use youtrack_rs::client::{Executor, YouTrack};

use serde::de::{self, Deserializer, Visitor};
use serde::{forward_to_deserialize_any, Deserialize, Serialize};
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

#[derive(Debug, Serialize, Deserialize)]
struct FieldType {
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CustomField {
    id: String,
    name: String,
    #[serde(rename = "fieldType")]
    field_type: FieldType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Bundle {
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProjectCustomField {
    field: CustomField,
    ordinal: i32,
    #[serde(rename = "canBeEmpty")]
    can_be_emtpy: bool,
    bundle: Option<Bundle>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Project {
    id: String,
    name: String,
    fields: Vec<ProjectCustomField>,
}

#[tokio::main]
async fn main() {
    let opt = YouTrackOpt::from_args();

    let yt = YouTrack::new(opt.youtrack_url.clone(), opt.youtrack_token.clone()).unwrap();

    let me = yt
        .get()
        .admin()
        .projects()
        .top("1")
        .skip("0")
        .fields("id,name,fields(field(id,name,fieldType(id)),canBeEmpty,ordinal,bundle(id))")
        .execute::<Vec<Project>>()
        .await
        .unwrap();

    let (headers, status, json) = me;
    println!("{:#?}", headers);
    println!("{}", status);
    println!("{:?}", json);
    let project = json.unwrap();
    let project = project.first().unwrap();

    for field in project.fields.iter() {
        let f = &field.field;
        if f.name != "Type" {
            continue;
        }
        println!("{:?}", f);
        let ref bundle = field.bundle;
        let bundle = bundle.clone().unwrap();
        let vals = yt
            .get()
            .admin()
            .custom_field_settings()
            .bundles()
            .enum_()
            .id(bundle.id.as_str())
            .fields("id,name,values(id,name)")
            .execute::<Value>()
            .await
            .unwrap();

        let (headers, status, json) = vals;
        println!("{:#?}", headers);
        println!("{}", status);
        println!("{:?}", json);
    }
}
