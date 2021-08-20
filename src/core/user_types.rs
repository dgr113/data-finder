use std::collections::HashMap;




#[allow(dead_code)] pub type GroupedPathsType = HashMap<String, Vec<HashMap<String, String>>>;
#[allow(dead_code)] pub type SortedPathsType = Vec<Vec<String>>;
#[allow(dead_code)] pub type SimpleMappingType = HashMap<String, String>;

pub type SingleRecType = HashMap<String, String>;
pub type GroupedRecordsType = HashMap<Vec<String>, Vec<SingleRecType>>;
#[allow(dead_code)] pub type ManyRecordsType = Vec<SingleRecType>;
