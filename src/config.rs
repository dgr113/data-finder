use std::collections::HashMap;

use serde::{ Serialize, Deserialize };




#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FinderListElem {
    pub glob: String,
    pub parsed_patt: String,
    pub included_fields: Vec<String>,
    pub group_fields: Vec<String>
}



#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FinderConfig {
    pub file_path_field_name: String,  // Name of field that containing contents of the file
    pub custom_patterns: HashMap<String, String>,
    pub list: HashMap<String, FinderListElem>
}
