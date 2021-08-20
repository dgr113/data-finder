#[macro_use] extern crate clap;

use std::fs;
use std::collections::HashMap;

use clap::App;
use serde::{ Serialize, Deserialize };

mod core;
use crate::core::data_utils::DataGroupLayer;
use crate::core::user_types::GroupedRecordsType;

pub use crate::core::io_utils;
pub use crate::core::config_utils;
pub use crate::core::paths_utils;




#[derive(Debug, Serialize, Deserialize)]
pub struct AppResult {
    order: Vec<String>,
    mapping: GroupedRecordsType
}


/** Get paths sorted by configuration file */
pub fn get_result(sorted_ids: Vec<String>, assets_mapping: GroupedRecordsType) -> AppResult {
    AppResult { order: sorted_ids, mapping: assets_mapping }
}


/** Run module as function */
pub fn run(config_path: &str, app_type: &str) -> serde_yaml::Value {
    // Load config objects from paths
    let config = config_utils::load_config( config_path );
    let patterns: HashMap<String, String> = serde_yaml::from_value( config["_custom_patterns"].clone() ).unwrap();

    // Get an important config fields
    let app_config = &config[app_type];
    let glob_patt = app_config["glob"].as_str().unwrap();
    let parsed_patt = app_config["parsed_patt"].as_str().unwrap();
    let group_fields: Vec<String> = serde_yaml::from_value( app_config["group_fields"].clone() ).unwrap();
    let included_fields: Vec<String> = serde_yaml::from_value( app_config["included_fields"].clone() ).unwrap();

    let files = paths_utils::get_files( glob_patt );
    let records = paths_utils::parse_grok(included_fields, &parsed_patt, files, &patterns);

    DataGroupLayer::create_nested_map(&records, &group_fields)
}



/** Run module as CLI */
pub fn run_cli() {
    let cli_config = load_yaml!( "../cli.yml" );
    let parser = App::from_yaml( cli_config ).get_matches();

    let config_path = &parser.args["config"].vals[0].to_str().unwrap();
    let app_type = parser.args["type"].vals[0].to_str().unwrap();

    let results = run(config_path, app_type);
    fs::write("results.yml", serde_yaml::to_string( &results ).unwrap()).unwrap();

    println!("{:?}", results);
}
