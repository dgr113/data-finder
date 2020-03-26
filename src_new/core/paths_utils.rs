use grok::Grok;
use std::collections::HashMap;
use std::str;
use glob;
use itertools::Itertools;
use crate::core::user_types::{SingleRecType, ManyRecordsType, GroupedRecordsType};



/// Get GROK Parser object from patterns mapping
/// * `patterns`: GROK patterns mapping
fn get_grok_patt(patterns: &HashMap<String, String>) -> Grok {
    let mut grok = Grok::default();
    for (k, v) in patterns {
        grok.insert_definition(k,  v);
    }
    grok
}


/// Filter grok function
fn filter_grok(included_keys: Vec<String>) -> impl Fn((&str, &str)) -> Option<(String, String)> {
  move |key_value_pair: (&str, &str)| {
      if included_keys.contains(&key_value_pair.0.to_string()) {
          Some((key_value_pair.0.to_string(), key_value_pair.1.to_string()))
      }
      else { None }
  }
}


/// Parse documents with GROK patterns
/// * `patterns`: GROK patterns mapping
pub fn parse_grok(keys: Vec<String>, grok_patt: &str, files: Vec<String>, patterns: &HashMap<String, String>) -> Vec<HashMap<String, String>> {
    let mut grok = get_grok_patt(patterns);
    let pattern = grok.compile(&grok_patt, false).expect("Error compile pattern!");
    let mut results = Vec::new();

    let filter_partial = filter_grok(keys);

    for filename in files {
        match pattern.match_against(&filename) {
            Some(m) => {
                let mut found_rec: HashMap<String, String> = m.iter().filter_map(&filter_partial).collect();
                found_rec.insert("MESSAGE".to_string(), filename.to_string());  // Insert full original string into GROK results
                results.push(found_rec);
            },
            None => println!("No matches found!")
        }
    };
    results
}


/// Get group keys from HashMap
fn get_group_keys(group_keys: &[String], rec: &SingleRecType) -> Vec<String> {
    group_keys.iter().map(|ind| (&rec[ind]).clone()).collect::<Vec<String>>()
}


/// Groupby grok patterns on TEST MODE
pub fn group_grok_old<'a>(group_indexes: &[String], data: ManyRecordsType) -> GroupedRecordsType {
    let mut results = HashMap::new();
    for (k, group) in &data.into_iter().group_by(|rec| get_group_keys(&group_indexes, rec)) {
        results.insert(k, group.collect::<Vec<_>>());
    }
    results
}


/// Get files by glob pattern
pub fn get_files(glob_patt: &str) -> Vec<String> {
    glob::glob(glob_patt).expect("Error: Wrong grok pattern for find files paths")
        .filter_map(Result::ok)
        .map(|path| path.to_str().unwrap().to_string())
        .collect()
}
