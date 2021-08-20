use std::io::{ prelude::*, BufReader, Error };
use std::fs::{ File, DirEntry, read_to_string };

use rayon::prelude::*;
use serde_json::Value;




fn _parse_json( file_content: String ) -> Value {
    serde_json::from_str( &file_content ).unwrap()
}


fn _read_file_content( path: &String ) -> String {
    read_to_string( path ).unwrap().to_string()
}


fn _path_unwrap( path: Result<DirEntry, Error> ) -> String {
    path.unwrap().path().to_str().unwrap().to_owned()
}



pub fn read_file_lines( path: &String ) -> Vec<String> {
    let file = File::open( path ).expect( "File not found!" );
    let reader = BufReader::new( file );
    let results: Vec<String> = reader.lines()
        .map( |line| line.unwrap_or( String::new() ) )
        .filter( |line| !line.is_empty() )
        .collect();
    results
}


pub fn read_files( paths: Vec<String> ) -> Vec<Value> {
    let results: Vec<Value> = paths.par_iter()
        .map( _read_file_content )
        .map( _parse_json )
        .collect();
    results
}
