use std::collections::HashMap;
use serde_yaml;




pub(crate) struct DataGroupLayer;

impl DataGroupLayer {
    /** Walk on Hashmap-like based by keys (group_indexes) and records values and return mutable end-node of this way */
    fn walk_by_keys<'a>(root_node: &'a mut serde_yaml::Value, group_indexes: &[String], rec: &HashMap<String, String>) -> &'a mut serde_yaml::Value {
            let mut curr_link = root_node;
            for k in group_indexes {
                let kv = rec[k].clone();
                curr_link = &mut curr_link[kv];
            }
            curr_link
    }

    /** Create nested map tree */
    pub(crate) fn create_nested_map(data_records: &[HashMap<String, String>], group_indexes: &[String]) -> serde_yaml::Value {
        let mut results = serde_yaml::Value::default();
        for rec in data_records {
            let data = serde_yaml::to_value( rec ).expect( "Error: Failed convert value into XML-tree!" );
            // let mut node_link = Self::walk_by_keys(&mut results, &group_indexes, &rec);
            let node_link = Self::walk_by_keys(&mut results, &group_indexes, &rec);
            match node_link.is_null() {
                true => {
                    *node_link = serde_yaml::to_value( vec![data,] ).unwrap();
                },
                false => {
                    node_link.as_sequence_mut().unwrap().push( data );
                }
            };
        }
        results
    }
}
