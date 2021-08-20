use std::collections::HashMap;
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
pub fn run(config: serde_json::Value, app_type: &str) -> serde_yaml::Value {
    let patterns: HashMap<String, String> = serde_json::from_value( config["_custom_patterns"].clone() ).unwrap();

    // Get an important config fields
    let app_config = &config[app_type];
    let glob_patt = app_config["glob"].as_str().unwrap();
    let parsed_patt = app_config["parsed_patt"].as_str().unwrap();
    let group_fields: Vec<String> = serde_json::from_value( app_config["group_fields"].clone() ).unwrap();
    let included_fields: Vec<String> = serde_json::from_value( app_config["included_fields"].clone() ).unwrap();

    let files = paths_utils::get_files( glob_patt );
    let records = paths_utils::parse_grok(included_fields, &parsed_patt, files, &patterns);

    DataGroupLayer::create_nested_map(&records, &group_fields)
}



/** Run module as CLI (TESTING ONLY) */
pub fn run_cli() {
    println!( "RUN FINDER AS CLI..." );
}
