use std::{fs, io};
use std::path::Path;
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use anyhow::*;
use csv::Writer;
use std::string::String;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct Activity {
      accountId: String, //0e90da3b-2acb-4aa2-80b7-4871cccd05e9,
      comment: Option<String>, //Swapped from atom on osmo,
      fee: f64, //0,
      quantity: f64, //1.38,
      r#type: String, //BUY,
      unitPrice: f64, //0.65,
      currency: String,// USD,
      dataSource: String, //YAHOO,
      date: String, // 2023-11-15T19:57:26.699Z,
      symbol: String //OSMOUSD
}

impl From<&serde_json::Value> for Activity {
    fn from(v: &serde_json::Value) -> Self {

        // Convert from str to String, otherwise serde will add extra quotation marks around the
        Activity {
            accountId: v["accountId"].as_str().unwrap().to_string(),
            comment: v["comment"].as_str().map(|str| str.to_string()),
            currency: v["currency"].as_str().unwrap().to_string(),
            dataSource: v["dataSource"].as_str().unwrap().to_string(),
            date: v["date"].as_str().unwrap().to_string(),
            fee: v["fee"].as_f64().unwrap(),
            symbol: v["symbol"].as_str().unwrap().to_string(),
            quantity: v["quantity"].as_f64().unwrap(),
            unitPrice: v["unitPrice"].as_f64().unwrap(),
            r#type: v["type"].as_str().unwrap().to_string(),
        }
    }
}

fn parse_activities<P: AsRef<Path>>(path: P) -> Result<Value, Error> {
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    let v: Value = serde_json::from_str(&contents)?;

    Ok(v)
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: String,
}

fn main() {
    // Parse activities from JSON file
    let args = Args::parse();
    let json = parse_activities(args.filename).unwrap();
    let activities_json = json["activities"].as_array().unwrap();
    let mut activities: Vec<Activity> = vec![];
    for activity_json in activities_json {
        let activity: Activity = Activity::from(activity_json);
        activities.push(activity);
    }
    
    // CSV
    let mut wtr = Writer::from_writer(io::stdout());
    for activity in activities {
        wtr.serialize(activity).unwrap();
    }
    wtr.flush().unwrap();
}
