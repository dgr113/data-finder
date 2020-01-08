use std::collections::HashMap;


pub type GroupedPathsType = HashMap<String, Vec<HashMap<String, String>>>;
pub type SortedPathsType = Vec<Vec<String>>;
pub type SimpleMappingType = HashMap<String, String>;


pub type SingleRecType = HashMap<String, String>;
pub type ManyRecordsType = Vec<SingleRecType>;
pub type GroupedRecordsType = HashMap<Vec<String>, Vec<SingleRecType>>;
