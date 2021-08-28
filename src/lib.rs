use std::hash::Hash;
use std::borrow::Borrow;

use serde::{ Deserialize, Serialize };

mod core;
pub mod config;
pub mod errors;

use crate::errors::ApiError;
use crate::config::FinderConfig;
use crate::core::data_utils::DataGroupLayer;
use crate::core::user_types::GroupedRecordsType;

pub use crate::core::io_utils;
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
pub fn run<S>(config: FinderConfig, app_type: S) -> Result<serde_yaml::Value, ApiError>
    where S: Into<String> + Hash + Eq, String: Borrow<S>
{
    let app_config = &config.list.get( &app_type ).ok_or( ApiError::IndexError ) ?;

    let glob_patt = &app_config.glob;
    let parsed_patt = &app_config.parsed_patt;
    let group_fields= &app_config.group_fields;
    let included_fields = &app_config.included_fields;

    let files = paths_utils::get_files( glob_patt ) ?;
    let records = paths_utils::parse_grok(included_fields, parsed_patt, files, &config.custom_patterns, &config.file_path_field_name) ?;

    Ok( DataGroupLayer::create_nested_map(&records, group_fields) )
}



/** Run module as CLI (TESTING ONLY) */
pub fn run_cli() {
    println!( "RUN FINDER AS CLI..." );
}
