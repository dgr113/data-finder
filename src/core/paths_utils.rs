use std::str;
use std::collections::HashMap;

use glob;
use grok::Grok;

use crate::errors::ApiError;
use crate::core::user_types::SingleRecType;




/** Get GROK Parser object from patterns mapping
* `patterns`: GROK patterns mapping
*/
fn get_grok( patterns: &HashMap<String, String> ) -> Grok {
    let mut grok = Grok::default();
    for (k, v) in patterns {
        grok.insert_definition(k,  v);
    }
    grok
}


/** Filter grok function */
fn filter_grok<'a>( included_keys: &'a [String] )
    -> impl Fn( (&str, &str) ) -> Option<(String, String)> + 'a
{
  move | key_value_pair: (&str, &str) | {
      if included_keys.contains( &key_value_pair.0.to_string() ) {
          Some( (key_value_pair.0.to_string(), key_value_pair.1.to_string()) )
      }
      else { None }
  }
}


/** Parse documents with GROK patterns
* `patterns`: GROK patterns mapping
*/
pub fn parse_grok(keys: &[String], grok_patt: &str, files: Vec<String>, patterns: &HashMap<String, String>, file_path_field_name: &str)
    -> Result<Vec<HashMap<String, String>>, ApiError>
{
    let mut grok = get_grok( patterns );
    let pattern = grok.compile(&grok_patt, false) ?;
    let mut results = Vec::new();

    let filter_func = filter_grok( keys );

    for file_path in files {
        match pattern.match_against( &file_path) {
            Some( m ) => {
                let mut found_rec: HashMap<String, String> = m.iter().filter_map( &filter_func ).collect();
                found_rec.insert(file_path_field_name.to_string(), file_path.to_string());  // Insert full original string into GROK results
                results.push( found_rec );
            },
            None => println!("No matches found!")
        }
    };
    Ok( results )
}


/** Get group keys from HashMap */
pub fn get_group_keys(group_keys: &[String], rec: &SingleRecType) -> Vec<String> {
    group_keys.iter().filter_map( |k| rec.get( k ).cloned() ).collect()
}


/** Get files by glob pattern */
pub fn get_files( glob_patt: &str ) -> Result<Vec<String>, ApiError> {
    let patt = glob::glob( glob_patt ) ?;
    let res = patt.filter_map( Result::ok )
        .map( |path| path.to_str().unwrap().to_string() )
        .collect();
    Ok( res )
}
