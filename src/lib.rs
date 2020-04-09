#[macro_use] extern crate clap;
extern crate grok;
extern crate itertools;
extern crate serde_yaml;
extern crate serde_json;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use clap::App;

use std::fs;

mod core;
use crate::core::paths_utils;
use crate::core::config_utils;
use crate::core::data_utils::{DataGroupLayer};
use crate::core::user_types::{GroupedPathsType, SortedPathsType, SimpleMappingType, GroupedRecordsType};
pub use crate::core::io_utils;



#[derive(Debug, Serialize, Deserialize)]
struct AppResult {
    order: Vec<String>,
    mapping: GroupedRecordsType
}


/// Get paths sordet by configuration file
fn get_result(sorted_ids: Vec<String>, assets_mapping: GroupedRecordsType) -> AppResult {
    AppResult { order: sorted_ids, mapping: assets_mapping }
}


/// Run module as function
pub fn run(config: serde_json::Value, app_type: &str) -> serde_yaml::Value {
    // Load config objects from paths
    let patterns: HashMap<String, String> = serde_json::from_value(config["custom_patterns"].clone()).unwrap();

    // Get an important config fields
    let app_config = &config[app_type];
    let glob_patt = app_config["glob"].as_str().unwrap();
    let parsed_patt = app_config["parsed_patt"].as_str().unwrap();
    let group_fields: Vec<String> = serde_json::from_value(app_config["group_fields"].clone()).unwrap();
    let included_fields: Vec<String> = serde_json::from_value(app_config["included_fields"].clone()).unwrap();

    let files = paths_utils::get_files(glob_patt);
    let records = paths_utils::parse_grok(included_fields, &parsed_patt, files, &patterns);

    DataGroupLayer::create_nested_map(&records, &group_fields)
}



/// Run module as CLI
pub fn run_cli() {
    // let cli_config = load_yaml!("../cli.yml");
    // let parser = App::from_yaml(cli_config).get_matches();
    //
    // let config_path = &parser.args["config"].vals[0].to_str().unwrap();
    // let app_type = parser.args["type"].vals[0].to_str().unwrap();
    //
    // let results = run(config_path, app_type);
    // fs::write("results.yml", serde_yaml::to_string(&results).unwrap());
    //
    // println!("{:?}", results);

    println!("START FINDER !");
}