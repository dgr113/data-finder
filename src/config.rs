use std::collections::HashMap;

use serde::{ Serialize, Deserialize };




#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataFinderListElem {
    pub glob: String,
    pub parsed_patt: String,
    pub included_fields: Vec<String>,
    pub group_fields: Vec<String>
}



#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataFinderConfig {
    pub custom_patterns: HashMap<String, String>,
    pub list: HashMap<String, DataFinderListElem>
}
